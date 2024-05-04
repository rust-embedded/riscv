use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "riscv32" {
        println!("cargo:rustc-cfg=riscv");
        println!("cargo:rustc-cfg=riscv32");
    } else if target_arch == "riscv64" {
        println!("cargo:rustc-cfg=riscv");
        println!("cargo:rustc-cfg=riscv64");
    }
}
