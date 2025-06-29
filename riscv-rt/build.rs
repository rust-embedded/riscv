// NOTE: Adapted from cortex-m/build.rs

use riscv_target_parser::{RiscvTarget, Width};
use std::env;

// List of all possible RISC-V configurations to check for in risv-rt
const RISCV_CFG: [&str; 4] = ["riscvi", "riscvm", "riscvf", "riscvd"];

fn add_linker_script(arch_width: Width) {
    // `CARGO_CFG_TARGET_POINTER_WIDTH` technically be used, but instruction
    // alignment and pointer width aren't necessarily the same things.
    unsafe {
        std::env::set_var("CARGO_CFG_ARCH_WIDTH", arch_width.to_string());
    }

    minilink::register_template("./link.x.in", "link.x");
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

        // set environment variable RISCV_RT_BASE_ISA to the base ISA of the target.
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

        add_linker_script(width);
    }
}
