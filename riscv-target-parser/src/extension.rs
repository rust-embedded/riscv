use crate::Error;
use std::collections::HashSet;

/// RISC-V standard extensions
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Extension {
    /// Base Integer Instruction Set
    I,
    /// Base Integer Instruction Set (embedded, only 16 registers)
    E,
    /// Integer Multiplication and Division
    M,
    /// Atomic Instructions
    A,
    /// Single-Precision Floating-Point
    F,
    /// Double-Precision Floating-Point
    D,
    /// Quad-Precision Floating-Point
    Q,
    /// Compressed Instructions
    C,
    /// Bit Manipulation
    B,
    /// Packed-SIMD Instructions
    P,
    /// Vector Operations
    V,
    /// Hypervisor
    H,
    /// Standard Z-type extension
    Z(String),
    /// Standard S-type extension
    S(String),
    /// Vendor extension
    X(String),
}

impl Extension {
    /// Determines if the extension is a base extension.
    pub const fn is_base(&self) -> bool {
        matches!(self, Self::I | Self::E)
    }
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::I => "i",
            Self::E => "e",
            Self::M => "m",
            Self::A => "a",
            Self::F => "f",
            Self::D => "d",
            Self::Q => "q",
            Self::C => "c",
            Self::B => "b",
            Self::P => "p",
            Self::V => "v",
            Self::H => "h",
            Self::Z(s) | Self::S(s) | Self::X(s) => s,
        };
        write!(f, "{repr}")
    }
}

impl<'a> TryFrom<&'a str> for Extension {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "i" => Ok(Extension::I),
            "e" => Ok(Extension::E),
            "m" => Ok(Extension::M),
            "a" => Ok(Extension::A),
            "f" => Ok(Extension::F),
            "d" => Ok(Extension::D),
            "q" => Ok(Extension::Q),
            "c" => Ok(Extension::C),
            "b" => Ok(Extension::B),
            "p" => Ok(Extension::P),
            "v" => Ok(Extension::V),
            "h" => Ok(Extension::H),
            _ => {
                if value.starts_with('z') {
                    Ok(Extension::Z(value.to_string()))
                } else if value.starts_with('s') {
                    Ok(Extension::S(value.to_string()))
                } else if value.starts_with('x') {
                    Ok(Extension::X(value.to_string()))
                } else {
                    Err(Self::Error::UnknownExtension(value))
                }
            }
        }
    }
}

/// Collection of RISC-V extensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extensions {
    extensions: HashSet<Extension>,
}

impl Extensions {
    /// Returns a vector with the list of extensions. Extensions are sorted in canonical order.
    ///
    /// The canonical order is defined as follows:
    /// 1. Base ISA (I or E)
    /// 2. Standard non-base extensions (M, A, F, D, Q, C, B, P, V, H)
    /// 3. Standard Z-type extensions (e.g., Zicsr)
    /// 4. Standard S-type extensions (e.g., Ssccfg)
    /// 5. Vendor X-type extensions (e.g., XSifivecdiscarddlone)
    ///
    /// Z, S, and X-type extensions are sorted by their string representation.
    pub fn extensions(&self) -> Vec<Extension> {
        let mut res = self.extensions.iter().cloned().collect::<Vec<_>>();
        res.sort();
        res
    }

    /// Returns the base extension (I or E) if present.
    pub fn base_extension(&self) -> Option<Extension> {
        if self.extensions.contains(&Extension::I) {
            Some(Extension::I)
        } else if self.extensions.contains(&Extension::E) {
            Some(Extension::E)
        } else {
            None
        }
    }

    /// Returns `true` if the collection contains the given extension.
    pub fn contains(&self, extension: &Extension) -> bool {
        self.extensions.contains(extension)
    }

    pub fn is_g(&self) -> bool {
        self.extensions.contains(&Extension::I)
            && self.extensions.contains(&Extension::M)
            && self.extensions.contains(&Extension::A)
            && self.extensions.contains(&Extension::F)
            && self.extensions.contains(&Extension::D)
    }

    /// Adds an extension to the collection. Returns `true` if the extension was not present.
    pub fn insert(&mut self, extension: Extension) -> bool {
        self.extensions.insert(extension)
    }

    /// Removes an extension from the collection. Returns `true` if the extension was present.
    pub fn remove(&mut self, extension: &Extension) -> bool {
        self.extensions.remove(extension)
    }
}

impl<'a> TryFrom<&'a str> for Extensions {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut value = value;
        let mut extensions = HashSet::new();

        while !value.is_empty() {
            let extension =
                if value.starts_with("Z") || value.starts_with("S") || value.starts_with("X") {
                    match value.find('_') {
                        Some(pos) => {
                            let (ext, _) = value.split_at(pos);
                            ext
                        }
                        None => value,
                    }
                } else {
                    &value[0..1] // single character extension
                };
            value = value.trim_start_matches(extension).trim_start_matches("_");

            match Extension::try_from(extension) {
                Ok(ext) => {
                    extensions.insert(ext);
                }
                Err(Self::Error::UnknownExtension(ext)) => {
                    if ext == "g" {
                        // G is a shorthand for IMAFD
                        extensions.insert(Extension::I);
                        extensions.insert(Extension::M);
                        extensions.insert(Extension::A);
                        extensions.insert(Extension::F);
                        extensions.insert(Extension::D);
                    } else {
                        return Err(Self::Error::UnknownExtension(ext));
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(Extensions { extensions })
    }
}

impl std::fmt::Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut extensions = String::new();
        let mut prev_zsx = false;
        for ext in &self.extensions() {
            if prev_zsx {
                extensions.push('_');
            }
            extensions.push_str(ext.to_string().as_str());
            prev_zsx = matches!(ext, Extension::Z(_) | Extension::S(_) | Extension::X(_));
        }
        match extensions.strip_prefix("imafd") {
            Some(extensions) => write!(f, "g{}", extensions),
            None => match extensions.strip_prefix("iemafd") {
                Some(extensions) => write!(f, "ge{}", extensions),
                None => write!(f, "{}", extensions),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extension_try_from() {
        assert_eq!(Extension::try_from("i"), Ok(Extension::I));
        assert_eq!(Extension::try_from("e"), Ok(Extension::E));
        assert_eq!(Extension::try_from("m"), Ok(Extension::M));
        assert_eq!(Extension::try_from("a"), Ok(Extension::A));
        assert_eq!(Extension::try_from("f"), Ok(Extension::F));
        assert_eq!(Extension::try_from("d"), Ok(Extension::D));
        assert_eq!(Extension::try_from("q"), Ok(Extension::Q));
        assert_eq!(Extension::try_from("c"), Ok(Extension::C));
        assert_eq!(Extension::try_from("b"), Ok(Extension::B));
        assert_eq!(Extension::try_from("p"), Ok(Extension::P));
        assert_eq!(Extension::try_from("v"), Ok(Extension::V));
        assert_eq!(Extension::try_from("h"), Ok(Extension::H));
        assert_eq!(
            Extension::try_from("Zicsr"),
            Ok(Extension::Z("Zicsr".to_string()))
        );
        assert_eq!(
            Extension::try_from("Ssccfg"),
            Ok(Extension::S("Ssccfg".to_string()))
        );
        assert_eq!(
            Extension::try_from("XSifivecdiscarddlone"),
            Ok(Extension::X("XSifivecdiscarddlone".to_string()))
        );
        assert_eq!(
            Extension::try_from("unknown"),
            Err(Error::UnknownExtension("unknown"))
        );
    }

    #[test]
    fn test_extension_to_string() {
        assert_eq!(Extension::I.to_string(), "i");
        assert_eq!(Extension::E.to_string(), "e");
        assert_eq!(Extension::M.to_string(), "m");
        assert_eq!(Extension::A.to_string(), "a");
        assert_eq!(Extension::F.to_string(), "f");
        assert_eq!(Extension::D.to_string(), "d");
        assert_eq!(Extension::Q.to_string(), "q");
        assert_eq!(Extension::C.to_string(), "c");
        assert_eq!(Extension::B.to_string(), "b");
        assert_eq!(Extension::P.to_string(), "p");
        assert_eq!(Extension::V.to_string(), "v");
        assert_eq!(Extension::H.to_string(), "h");
        assert_eq!(Extension::Z("Zicsr".to_string()).to_string(), "Zicsr");
        assert_eq!(Extension::S("Ssccfg".to_string()).to_string(), "Ssccfg");
        assert_eq!(
            Extension::X("XSifivecdiscarddlone".to_string()).to_string(),
            "XSifivecdiscarddlone"
        );
    }

    #[test]
    fn test_extension_cmp() {
        let mut extensions = vec![
            Extension::I,
            Extension::M,
            Extension::A,
            Extension::F,
            Extension::D,
            Extension::Q,
            Extension::C,
            Extension::B,
            Extension::P,
            Extension::V,
            Extension::H,
            Extension::Z("Zicsr".to_string()),
            Extension::S("Ssccfg".to_string()),
            Extension::X("XSifivecdiscarddlone".to_string()),
        ];
        extensions.reverse();
        extensions.sort();
        assert_eq!(
            extensions,
            vec![
                Extension::I,
                Extension::M,
                Extension::A,
                Extension::F,
                Extension::D,
                Extension::Q,
                Extension::C,
                Extension::B,
                Extension::P,
                Extension::V,
                Extension::H,
                Extension::Z("Zicsr".to_string()),
                Extension::S("Ssccfg".to_string()),
                Extension::X("XSifivecdiscarddlone".to_string()),
            ]
        );
    }

    #[test]
    fn test_extensions_try_from() {
        let mut try_extensions = Extensions::try_from("");
        assert!(try_extensions.is_ok());
        let mut extensions = try_extensions.unwrap();
        assert!(extensions.extensions().is_empty());
        assert!(extensions.base_extension().is_none());

        try_extensions =
            Extensions::try_from("giemafdqcbpvhXSifivecdiscarddlone_Ssccfg_Zicsr_Zaamo_u");
        assert!(try_extensions.is_err());
        assert_eq!(try_extensions, Err(Error::UnknownExtension("u")));

        try_extensions = Extensions::try_from("geqcbpvhXSifivecdiscarddlone_Ssccfg_Zicsr_Zaamo_");
        assert!(try_extensions.is_ok());
        extensions = try_extensions.unwrap();
        assert_eq!(
            extensions.extensions(),
            vec![
                Extension::I,
                Extension::E,
                Extension::M,
                Extension::A,
                Extension::F,
                Extension::D,
                Extension::Q,
                Extension::C,
                Extension::B,
                Extension::P,
                Extension::V,
                Extension::H,
                Extension::Z("Zaamo".to_string()),
                Extension::Z("Zicsr".to_string()),
                Extension::S("Ssccfg".to_string()),
                Extension::X("XSifivecdiscarddlone".to_string()),
            ]
        );
        assert_eq!(extensions.base_extension(), Some(Extension::I));

        try_extensions =
            Extensions::try_from("iemafdqcbpvhXSifivecdiscarddlone_Ssccfg_Zicsr_Zaamo_");
        assert!(try_extensions.is_ok());
        extensions = try_extensions.unwrap();
        assert_eq!(
            extensions.extensions(),
            vec![
                Extension::I,
                Extension::E,
                Extension::M,
                Extension::A,
                Extension::F,
                Extension::D,
                Extension::Q,
                Extension::C,
                Extension::B,
                Extension::P,
                Extension::V,
                Extension::H,
                Extension::Z("Zaamo".to_string()),
                Extension::Z("Zicsr".to_string()),
                Extension::S("Ssccfg".to_string()),
                Extension::X("XSifivecdiscarddlone".to_string()),
            ]
        );
        assert_eq!(extensions.base_extension(), Some(Extension::I));

        try_extensions =
            Extensions::try_from("emafdqcbpvhXSifivecdiscarddlone_Ssccfg_Zicsr_Zaamo_");
        assert!(try_extensions.is_ok());
        extensions = try_extensions.unwrap();
        assert_eq!(
            extensions.extensions(),
            vec![
                Extension::E,
                Extension::M,
                Extension::A,
                Extension::F,
                Extension::D,
                Extension::Q,
                Extension::C,
                Extension::B,
                Extension::P,
                Extension::V,
                Extension::H,
                Extension::Z("Zaamo".to_string()),
                Extension::Z("Zicsr".to_string()),
                Extension::S("Ssccfg".to_string()),
                Extension::X("XSifivecdiscarddlone".to_string()),
            ]
        );
        assert_eq!(extensions.base_extension(), Some(Extension::E));
    }

    #[test]
    fn test_extensions_insert_remove() {
        let mut extensions = Extensions::try_from("gc").unwrap();

        assert_eq!(extensions.extensions.len(), 6);
        assert!(extensions.contains(&Extension::I));
        assert!(extensions.contains(&Extension::M));
        assert!(extensions.contains(&Extension::A));
        assert!(extensions.contains(&Extension::F));
        assert!(extensions.contains(&Extension::D));
        assert!(extensions.contains(&Extension::C));
        assert!(!extensions.contains(&Extension::E));
        assert!(!extensions.contains(&Extension::Q));
        assert_eq!(extensions.base_extension(), Some(Extension::I));

        assert!(!extensions.insert(Extension::I));
        assert!(!extensions.remove(&Extension::E));
        assert_eq!(extensions.extensions.len(), 6);

        assert!(extensions.insert(Extension::E));
        assert_eq!(extensions.extensions.len(), 7);
        assert!(extensions.contains(&Extension::E));
        assert_eq!(extensions.base_extension(), Some(Extension::I));

        assert!(extensions.remove(&Extension::I));
        assert_eq!(extensions.extensions.len(), 6);
        assert!(!extensions.contains(&Extension::I));
        assert_eq!(extensions.base_extension(), Some(Extension::E));

        assert!(extensions.remove(&Extension::E));
        assert_eq!(extensions.extensions.len(), 5);
        assert!(!extensions.contains(&Extension::E));
        assert_eq!(extensions.base_extension(), None);
    }

    #[test]
    fn test_extensions_to_string() {
        let mut extensions = Extensions::try_from("imafdc").unwrap();
        assert_eq!(extensions.to_string(), "gc");

        extensions.insert(Extension::try_from("Ssccfg").unwrap());
        assert_eq!(extensions.to_string(), "gcSsccfg");

        extensions.insert(Extension::try_from("Zicsr").unwrap());
        assert_eq!(extensions.to_string(), "gcZicsr_Ssccfg");

        extensions.insert(Extension::try_from("Zaamo").unwrap());
        assert_eq!(extensions.to_string(), "gcZaamo_Zicsr_Ssccfg");

        extensions.insert(Extension::try_from("XSifivecdiscarddlone").unwrap());
        assert_eq!(
            extensions.to_string(),
            "gcZaamo_Zicsr_Ssccfg_XSifivecdiscarddlone"
        );

        extensions.insert(Extension::try_from("e").unwrap());
        assert_eq!(
            extensions.to_string(),
            "gecZaamo_Zicsr_Ssccfg_XSifivecdiscarddlone"
        );

        extensions.remove(&Extension::I);
        assert_eq!(
            extensions.to_string(),
            "emafdcZaamo_Zicsr_Ssccfg_XSifivecdiscarddlone"
        );

        extensions.remove(&Extension::E);
        assert_eq!(
            extensions.to_string(),
            "mafdcZaamo_Zicsr_Ssccfg_XSifivecdiscarddlone"
        );

        extensions.insert(Extension::I);
        assert_eq!(
            extensions.to_string(),
            "gcZaamo_Zicsr_Ssccfg_XSifivecdiscarddlone"
        );
    }
}
