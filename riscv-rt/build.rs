// NOTE: Adapted from cortex-m/build.rs

use riscv_target_parser::RiscvTarget;
use std::{env, fs, io, path::PathBuf};

// List of all possible RISC-V configurations to check for in risv-rt
const RISCV_CFG: [&str; 4] = ["riscvi", "riscvm", "riscvf", "riscvd"];

fn add_linker_script(arch_width: u32) -> io::Result<()> {
    // Read the file to a string and replace all occurrences of ${ARCH_WIDTH} with the arch width
    let mut content = fs::read_to_string("link.x.in")?;
    content = content.replace("${ARCH_WIDTH}", &arch_width.to_string());

    // Get target-dependent linker configuration and replace ${INCLUDE_LINKER_FILES} with it
    let mut include_content = String::new();

    // If no-exceptions is disabled, include the exceptions.x files
    if env::var_os("CARGO_FEATURE_NO_EXCEPTIONS").is_none() {
        let exceptions_content = fs::read_to_string("exceptions.x")?;
        include_content.push_str(&(exceptions_content + "\n"));
    }
    // If no-interrupts is disabled, include the interrupts.x files
    if env::var_os("CARGO_FEATURE_NO_INTERRUPTS").is_none() {
        let interrupts_content = fs::read_to_string("interrupts.x")?;
        include_content.push_str(&(interrupts_content + "\n"));
    }
    // If device is enabled, include the device.x file (usually, provided by PACs)
    if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        include_content.push_str("/* Device-specific exception and interrupt handlers */\n");
        include_content.push_str("INCLUDE device.x\n");
    }

    content = content.replace("${INCLUDE_LINKER_FILES}", &include_content);

    // Put the linker script somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
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
        if env::var_os("CARGO_FEATURE_V_TRAP").is_some()
            && env::var_os("CARGO_FEATURE_NO_INTERRUPTS").is_none()
        {
            // This environment variable is used by the `#[riscv::pac_enum()]` call in
            // `src/interrupts.rs` (when `v-trap` is enabled and `no-interrupts` disabled).
            println!("cargo:rerun-if-env-changed=RISCV_MTVEC_ALIGN");
        }

        for flag in target.rustc_flags() {
            // Required until target_feature risc-v is stable and in-use
            if RISCV_CFG.contains(&flag.as_str()) {
                println!("cargo:rustc-cfg={flag}");
            }
        }
        add_linker_script(width.into()).unwrap();
    }
}
