use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    minilink::register_template("device.x", "device.x");
    minilink::register_template("memory.x", "memory.x");
}
