extern crate chrono;
extern crate rustc_version;

// NOTE: Adapted from cortex-m/build.rs
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::NaiveDate;

fn main() {
    let meta = rustc_version::version_meta().unwrap();
	// newest nightlies don't need 'extern crate compiler_builtins'
    if meta.commit_date.unwrap().parse::<NaiveDate>().unwrap() < NaiveDate::from_ymd(2018, 04, 07) {
        println!("cargo:rustc-cfg=needs_cb")
    }
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}
