// NOTE: Adapted from cortex-m/build.rs
extern crate riscv_target;

use riscv_target::Target;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("riscv") {
        let mut target = Target::from_target_str(&target);
        target.retain_extensions("imfdc");
        let archive: String;
        if cfg!(feature = "s-mode") {
            println!("======== compiling riscv-rt for s-mode");
            archive = format!("bin/{}-smode.a", target.to_string());
        } else {
            archive = format!("bin/{}.a", target.to_string());
        }

        fs::copy(&archive, out_dir.join(format!("lib{}.a", name))).unwrap();
        println!("cargo:rerun-if-changed={}", archive);
        println!("cargo:rustc-link-lib=static={}", name);
    }

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");
}
