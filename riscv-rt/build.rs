// NOTE: Adapted from cortex-m/build.rs

use riscv_target::Target;
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
    let target = env::var("TARGET").unwrap();
    let _name = env::var("CARGO_PKG_NAME").unwrap();

    // set configuration flags depending on the target
    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");
        let target = Target::from_target_str(&target);

        // generate the linker script
        let arch_width = match target.bits {
            32 => {
                println!("cargo:rustc-cfg=riscv32");
                4
            }
            64 => {
                println!("cargo:rustc-cfg=riscv64");
                8
            }
            _ => panic!("Unsupported bit width"),
        };
        add_linker_script(arch_width).unwrap();

        // expose the ISA extensions
        if target.has_extension('m') {
            println!("cargo:rustc-cfg=riscvm");
        }
    }
}
