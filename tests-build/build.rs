use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // Put device.x somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("device.x"))
        .unwrap()
        .write_all(include_bytes!("device.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=device.x");

    // Put memory.x somewhere the linker can find it
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rerun-if-changed=build.rs");
}
