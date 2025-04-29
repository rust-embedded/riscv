pub mod extension;
pub use extension::{Extension, Extensions};

/// Error variants for the RISC-V target parser.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error<'a> {
    InvalidTriple(&'a str),
    InvalidArch(&'a str),
    InvalidWidth(usize),
    UnknownExtension(&'a str),
    UnknownTargetFeature(&'a str),
}

/// Helper struct to parse and store a target triple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetTriple<'a> {
    arch: &'a str,
    vendor: &'a str,
    os: &'a str,
    bin: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for TargetTriple<'a> {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parts = value.split('-');

        let arch = parts.next().ok_or(Error::InvalidTriple(value))?;
        let vendor = parts.next().ok_or(Error::InvalidTriple(value))?;
        let os = parts.next().ok_or(Error::InvalidTriple(value))?;
        let bin = parts.next();

        Ok(Self {
            arch,
            vendor,
            os,
            bin,
        })
    }
}

impl std::fmt::Display for TargetTriple<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.arch, self.vendor, self.os)?;
        if let Some(bin) = self.bin {
            write!(f, "-{bin}")?;
        }
        Ok(())
    }
}

/// The width of the RISC-V architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Width {
    /// 32-bit RISC-V architecture.
    W32,
    /// 64-bit RISC-V architecture.
    W64,
    /// 128-bit RISC-V architecture.
    W128,
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::W32 => write!(f, "32"),
            Self::W64 => write!(f, "64"),
            Self::W128 => write!(f, "128"),
        }
    }
}

macro_rules! impl_try_from_width {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for Width {
                type Error = Error<'static>;
                fn try_from(bits: $t) -> Result<Self, Self::Error> {
                    match bits {
                        32 => Ok(Self::W32),
                        64 => Ok(Self::W64),
                        128 => Ok(Self::W128),
                        _ => Err(Self::Error::InvalidWidth(bits as usize)),
                    }
                }
            }
            impl From<Width> for $t {
                fn from(width: Width) -> Self {
                    match width {
                        Width::W32 => 32,
                        Width::W64 => 64,
                        Width::W128 => 128,
                    }
                }
            }
        )*
    };
}
impl_try_from_width!(u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);

/// Struct that represents a RISC-V target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiscvTarget {
    width: Width,
    extensions: Extensions,
}

impl RiscvTarget {
    /// Builds a RISC-V target from a target triple and cargo flags.
    /// This function is expected to be called from a build script.
    ///
    /// The target triple is expected to be in the form `riscv{width}{extensions}-vendor-os[-bin]`.
    /// If the target triple is invalid, an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// // In build.rs
    /// let target = std::env::var("TARGET").unwrap();
    /// let cargo_flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    /// let target = riscv_target_parser::RiscvTarget::build(&target, &cargo_flags).unwrap(); // This will panic if the target is invalid
    /// ```
    pub fn build<'a>(target: &'a str, cargo_flags: &'a str) -> Result<Self, Error<'a>> {
        let triple = TargetTriple::try_from(target)?;
        let mut target = Self::try_from(triple)?;

        for target_feature in cargo_flags
            .split(0x1fu8 as char)
            .filter(|arg| arg.starts_with("target-feature="))
            .flat_map(|arg| {
                let arg = arg.trim_start_matches("target-feature=");
                arg.split(',')
            })
        {
            if let Some(feature) = target_feature.strip_prefix('+') {
                let extension = Extension::try_from(feature)?;
                target.extensions.insert(extension);
            } else if let Some(feature) = target_feature.strip_prefix('-') {
                let extension = Extension::try_from(feature)?;
                target.extensions.remove(&extension);
            } else {
                return Err(Error::UnknownTargetFeature(target_feature));
            }
        }
        Ok(target)
    }

    /// Returns a list of flags to pass to `rustc` for the given RISC-V target.
    /// This function is expected to be called from a build script.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let target = std::env::var("TARGET").unwrap();
    /// let cargo_flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    /// let target = riscv_target_parser::RiscvTarget::build(&target, &cargo_flags).unwrap();
    /// for flag in target.rustc_flags() {
    ///    println!("cargo:rustc-check-cfg=cfg({})", flag);
    ///    println!("cargo:rustc-cfg={}", flag);
    /// }
    ///
    pub fn rustc_flags(&self) -> Vec<String> {
        self.extensions
            .extensions()
            .iter()
            .map(|e| format!("riscv{e}"))
            .collect::<Vec<_>>()
    }

    /// Returns the LLVM base ISA for the given RISC-V target.
    pub fn llvm_base_isa(&self) -> String {
        match (self.width, self.extensions.base_extension()) {
            (Width::W32, Some(Extension::I)) => String::from("rv32i"),
            (Width::W32, Some(Extension::E)) => String::from("rv32e"),
            (Width::W64, Some(Extension::I)) => String::from("rv64i"),
            (Width::W64, Some(Extension::E)) => String::from("rv64e"),
            (_, None) => panic!("RISC-V target must have a base extension"),
            _ => panic!("LLVM does not support this base ISA"),
        }
    }

    /// Returns the arch code to patch LLVM spurious errors.
    ///
    /// # Note
    ///
    /// This is a provisional patch and is limited to work for the riscv-rt crate only.
    ///
    /// # Related issues
    ///
    /// - <https://github.com/rust-embedded/riscv/issues/175>
    /// - <https://github.com/rust-lang/rust/issues/80608>
    /// - <https://github.com/llvm/llvm-project/issues/61991>
    pub fn llvm_arch_patch(&self) -> String {
        let mut patch = self.llvm_base_isa();
        if self.extensions.contains(&Extension::M) {
            patch.push('m');
        }
        if self.extensions.contains(&Extension::F) {
            patch.push('f');
        }
        if self.extensions.contains(&Extension::D) {
            patch.push('d');
        }
        patch
    }

    /// Returns the width of the RISC-V architecture.
    pub fn width(&self) -> Width {
        self.width
    }

    /// Returns the base extension of the RISC-V architecture (if any).
    pub fn base_extension(&self) -> Option<Extension> {
        self.extensions.base_extension()
    }
}

impl<'a> TryFrom<TargetTriple<'a>> for RiscvTarget {
    type Error = Error<'a>;

    fn try_from(triple: TargetTriple<'a>) -> Result<Self, Self::Error> {
        match triple.arch.strip_prefix("riscv") {
            Some(arch) => {
                match arch
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(arch.len())
                {
                    0 => Err(Error::InvalidArch(arch)),
                    digit_end => {
                        let (width_str, extensions_str) = arch.split_at(digit_end);
                        let width = width_str.parse::<u32>().unwrap().try_into()?;
                        let extensions = extensions_str.try_into()?;
                        Ok(Self { width, extensions })
                    }
                }
            }
            None => Err(Error::InvalidArch(triple.arch)),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_target() {
        let target = "riscv32imac-unknown-none-elf";
        let cargo_flags = "target-feature=+m,-a,+f";
        let target = super::RiscvTarget::build(target, cargo_flags).unwrap();
        let rustc_flags = target.rustc_flags();
        assert_eq!(rustc_flags, vec!["riscvi", "riscvm", "riscvf", "riscvc"]);
    }
}
