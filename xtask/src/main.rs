use anyhow::{bail, Context};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || args[0] != "qemu" {
        bail!("usage: cargo run -p xtask -- qemu --target <triple> --example <name>");
    }
    args.remove(0);
    let mut target = None;
    let mut example = None;
    let mut features: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--target" => {
                target = Some(args.get(i + 1).context("missing target")?.clone());
                i += 2;
            }
            "--example" => {
                example = Some(args.get(i + 1).context("missing example")?.clone());
                i += 2;
            }
            "--features" => {
                features = Some(args.get(i + 1).context("missing features")?.clone());
                i += 2;
            }
            _ => {
                bail!("unknown arg {}", args[i]);
            }
        }
    }
    let target = target.context("--target required")?;
    let example = example.context("--example required")?;
    let mut rustflags = "-C link-arg=-Triscv-rt/examples/device_virt_m.x".to_string();
    if let Some(f) = &features {
        if f.contains("s-mode") {
            rustflags = "-C link-arg=-Triscv-rt/examples/device_virt_s.x".into();
        }
    }

    let mut cmd = Command::new("cargo");
    cmd.env("RUSTFLAGS", rustflags).args([
        "build",
        "--package",
        "riscv-rt",
        "--release",
        "--target",
        &target,
        "--example",
        &example,
    ]);
    cmd.apply_features(features.as_deref());
    let status = cmd.status()?;
    if !status.success() {
        bail!("build failed");
    }

    let qemu = if target.starts_with("riscv32") {
        "qemu-system-riscv32"
    } else {
        "qemu-system-riscv64"
    };
    let mut qemu_args = vec![
        "-machine",
        "virt",
        "-nographic",
        "-serial",
        "stdio",
        "-monitor",
        "none",
    ];
    if !features.as_deref().unwrap_or("").contains("s-mode") {
        qemu_args.push("-bios");
        qemu_args.push("none");
    }
    let kernel_path = format!("target/{}/release/examples/{}", target, example);
    let mut child = Command::new(qemu)
        .args(&qemu_args)
        .arg("-kernel")
        .arg(&kernel_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("running qemu")?;
    thread::sleep(Duration::from_secs(2));
    let _ = child.kill();
    let output = child.wait_with_output()?;
    let raw_stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stdout = raw_stdout
        .lines()
        .filter(|l| !l.starts_with("QEMU ") && !l.contains("monitor"))
        .collect::<Vec<_>>()
        .join("\n");
    let stdout = if stdout.is_empty() {
        String::new()
    } else {
        format!("{}\n", stdout.trim())
    };

    let expected_path: PathBuf = ["ci", "expected", &target, &format!("{}.run", example)]
        .iter()
        .collect();
    if !expected_path.exists() {
        fs::create_dir_all(expected_path.parent().unwrap())?;
        fs::write(&expected_path, stdout.as_bytes())?;
        bail!("expected output created; re-run CI");
    }
    let expected = fs::read_to_string(&expected_path)?;
    if expected != stdout {
        bail!(
            "output mismatch\nexpected: {}\nactual: {}",
            expected,
            stdout
        );
    }
    if !stdout.is_empty() {
        println!("{}", stdout.trim_end());
    }
    Ok(())
}

trait CmdExt {
    fn apply_features(&mut self, f: Option<&str>) -> &mut Self;
}
impl CmdExt for std::process::Command {
    fn apply_features(&mut self, f: Option<&str>) -> &mut Self {
        if let Some(feat) = f {
            self.arg("--features").arg(feat);
        }
        self
    }
}
