// NOTE: Adapted from cortex-m/build.rs

use std::{collections::HashSet, env, fs, io, path::PathBuf};

fn add_linker_script(arch_width: u32) -> io::Result<()> {
    // Read the file to a string and replace all occurrences of ${ARCH_WIDTH} with the arch width
    let mut content = fs::read_to_string("link.x.in")?;
    content = content.replace("${ARCH_WIDTH}", &arch_width.to_string());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), content)?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");

    Ok(())
}

/// Parse the target RISC-V architecture and returns its bit width and the extension set
fn parse_target(target: &str) -> (u32, HashSet<char>) {
    // isolate bit width and extensions from the rest of the target information
    let arch = target
        .trim_start_matches("riscv")
        .split('-')
        .next()
        .unwrap();

    let bits = arch
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    let mut extensions: HashSet<char> = arch.chars().skip_while(|c| c.is_ascii_digit()).collect();
    // get rid of the 'g' shorthand extension
    if extensions.remove(&'g') {
        extensions.insert('i');
        extensions.insert('m');
        extensions.insert('a');
        extensions.insert('f');
        extensions.insert('d');
    }

    (bits, extensions)
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let _name = env::var("CARGO_PKG_NAME").unwrap();

    // This is required until target_feature risc-v work is stable and in-use (rust 1.75.0)
    let cargo_flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    let cargo_flags = cargo_flags
        .split(0x1fu8 as char)
        .filter(|arg| !arg.is_empty());
    
    let target_features = cargo_flags
        .filter(|k| k.starts_with("target-feature=")).flat_map(|str| {
            let flags = str.split('=').collect::<Vec<&str>>()[1];
            flags.split(',')
        });

    // set configuration flags depending on the target
    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");

        let (bits, mut extensions) = parse_target(&target);

        // generate the linker script and expose the ISA width
        let arch_width = match bits {
            32 => {
                println!("cargo:rustc-cfg=riscv32");
                4
            }
            64 => {
                println!("cargo:rustc-cfg=riscv64");
                8
            }
            _ => panic!("Unsupported bit width"),
        };
        add_linker_script(arch_width).unwrap();

        target_features.for_each(|feature| {
            let chars = feature.chars().collect::<Vec<char>>();
            if chars[0] == '+' {
                extensions.insert(chars[1]);
            } else if chars[0] == '-' {
                extensions.remove(&chars[1]);
            }
        });

        // expose the ISA extensions
        for ext in &extensions {
            println!("cargo:rustc-cfg=riscv{}", ext);
        }
    }
}
