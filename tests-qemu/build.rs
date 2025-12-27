use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let s_mode = env::var_os("CARGO_FEATURE_S_MODE").is_some();
    let multi_hart = env::var_os("CARGO_FEATURE_MULTI_HART").is_some();

    // Multi-hart is only supported in M-mode; s-mode takes priority if both are enabled
    let memory_x = match (s_mode, multi_hart) {
        (true, _) => include_bytes!("memory-s-mode.x").as_slice(),
        (_, true) => include_bytes!("memory-multihart.x").as_slice(),
        _ => include_bytes!("memory.x").as_slice(),
    };

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(memory_x)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rustc-link-arg=-Tmemory.x");

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=memory-multihart.x");
    println!("cargo:rerun-if-changed=memory-s-mode.x");
    println!("cargo:rerun-if-changed=build.rs");
}
