// NOTE: Adapted from cortex-m/build.rs

use riscv_target_parser::{Extension, RiscvTarget, Width};
use std::{env, fs, io, path::PathBuf};

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
    for ext in ['i', 'e', 'm', 'a', 'f', 'd', 'g', 'c'] {
        println!("cargo:rustc-check-cfg=cfg(riscv{})", ext);
    }

    let target = env::var("TARGET").unwrap();
    let cargo_flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();

    if let Ok(target) = RiscvTarget::build(&target, &cargo_flags) {
        let width = target.width();
        let base = target.base_extension().expect("No base extension found");

        // set environmet variable RISCV_RT_BASE_ISA to the width of the target
        // this is used in riscv_rt_macros to determine the base ISA
        let env_var = match (width, base) {
            (Width::W32, Extension::I) => "rv32i",
            (Width::W32, Extension::E) => "rv32e",
            (Width::W64, Extension::I) => "rv64i",
            (Width::W64, Extension::E) => "rv64e",
            _ => panic!("Unsupported target"),
        };
        println!("cargo:rustc-env=RISCV_RT_BASE_ISA={env_var}");

        for flag in target.rustc_flags() {
            // Required until target_feature risc-v is stable and in-use
            println!("cargo:rustc-check-cfg=cfg({flag})");
            println!("cargo:rustc-cfg={flag}");
        }
        add_linker_script(width.into()).unwrap();
    }
}
