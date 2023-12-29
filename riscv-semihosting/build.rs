use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");
    }
}
