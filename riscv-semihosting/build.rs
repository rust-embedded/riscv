use std::env;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(riscv)");

    let target = env::var("TARGET").unwrap();

    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");
    }
}
