use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let s_mode = env::var_os("CARGO_FEATURE_S_MODE").is_some();
    let multi_hart = env::var_os("CARGO_FEATURE_MULTI_HART").is_some();

    let memory_x = match (s_mode, multi_hart) {
        (true, _) => include_bytes!("memory-s-mode.x").as_slice(),
        (false, true) => include_bytes!("memory-m-mode-multihart.x").as_slice(),
        (false, false) => include_bytes!("memory-m-mode.x").as_slice(),
    };

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(memory_x)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rustc-link-arg=-Tmemory.x");

    println!("cargo:rerun-if-changed=memory-m-mode.x");
    println!("cargo:rerun-if-changed=memory-m-mode-multihart.x");
    println!("cargo:rerun-if-changed=memory-s-mode.x");
    println!("cargo:rerun-if-changed=build.rs");
}
