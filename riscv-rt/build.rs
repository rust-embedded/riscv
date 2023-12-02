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

    // set configuration flags depending on the target
    if target.starts_with("riscv") {
        println!("cargo:rustc-cfg=riscv");

        let (bits, extensions) = parse_target(&target);

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

        // expose the ISA extensions
        for ext in &extensions {
            println!("cargo:rustc-cfg=riscv{}", ext);
        }
    }
}
