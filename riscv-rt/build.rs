// NOTE: Adapted from cortex-m/build.rs

use riscv_target_parser::RiscvTarget;
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
        for flag in target.rustc_flags() {
            // Required until target_feature risc-v is stable and in-use
            println!("cargo:rustc-check-cfg=cfg({flag})");
            println!("cargo:rustc-cfg={flag}");
        }
        add_linker_script(target.width().into()).unwrap();
    }
}
