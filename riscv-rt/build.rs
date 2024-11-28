// NOTE: Adapted from cortex-m/build.rs

use riscv_target_parser::RiscvTarget;
use std::{env, fs, io, path::PathBuf};

// List of all possible RISC-V configurations to check for in risv-rt
const RISCV_CFG: [&str; 5] = ["riscvi", "riscve", "riscvm", "riscvf", "riscvd"];

fn add_linker_script(arch_width: u32) -> io::Result<()> {
    // Read the file to a string and replace all occurrences of ${ARCH_WIDTH} with the arch width
    let mut content = fs::read_to_string("link.x.in")?;
    content = content.replace("${ARCH_WIDTH}", &arch_width.to_string());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), content)?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");

    Ok(())
}

fn main() {
    // Required until target_feature risc-v is stable and in-use (rust 1.75)
    for ext in RISCV_CFG.iter() {
        println!("cargo:rustc-check-cfg=cfg({ext})");
    }

    let target = env::var("TARGET").unwrap();
    let cargo_flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();

    if let Ok(target) = RiscvTarget::build(&target, &cargo_flags) {
        let width = target.width();

        // set environmet variable RISCV_RT_BASE_ISA to the base ISA of the target.
        println!(
            "cargo:rustc-env=RISCV_RT_BASE_ISA={}",
            target.llvm_base_isa()
        );
        // set environment variable RISCV_RT_LLVM_ARCH_PATCH to patch LLVM bug.
        // (this env variable is temporary and will be removed after LLVM being fixed)
        println!(
            "cargo:rustc-env=RISCV_RT_LLVM_ARCH_PATCH={}",
            target.llvm_arch_patch()
        );
        // make sure that these env variables are not changed without notice.
        println!("cargo:rerun-if-env-changed=RISCV_RT_BASE_ISA");
        println!("cargo:rerun-if-env-changed=RISCV_RT_LLVM_ARCH_PATCH");

        for flag in target.rustc_flags() {
            // Required until target_feature risc-v is stable and in-use
            if RISCV_CFG.contains(&flag.as_str()) {
                println!("cargo:rustc-cfg={flag}");
            }
        }
        add_linker_script(width.into()).unwrap();
    }
}
