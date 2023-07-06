// NOTE: Adapted from cortex-m/build.rs

use riscv_target::Target;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let _name = env::var("CARGO_PKG_NAME").unwrap();

    // set configuration flags depending on the target
    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");
        let target = Target::from_target_str(&target);
        match target.bits {
            32 => {
                println!("cargo:rustc-cfg=riscv32");
            }
            64 => {
                println!("cargo:rustc-cfg=riscv64");
            }
            _ => panic!("Unsupported bit width"),
        }
        if target.has_extension('m') {
            println!("cargo:rustc-cfg=riscvm"); // we can expose extensions this way
        }
    }

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");
}
