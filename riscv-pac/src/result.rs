use core::fmt;

/// Convenience alias for the [Result](core::result::Result) type for the library.
pub type Result<T> = core::result::Result<T, Error>;

/// Represents error variants for the library.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Attempted out-of-bounds access.
    IndexOutOfBounds {
        index: usize,
        min: usize,
        max: usize,
    },
    /// Invalid field value.
    InvalidValue {
        field: &'static str,
        value: usize,
        bitmask: usize,
    },
    /// Invalid value of a register field that does not match any known variants.
    InvalidVariant { field: &'static str, value: usize },
    /// Unimplemented function or type.
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds { index, min, max } => write!(
                f,
                "out-of-bounds access, index: {index}, min: {min}, max: {max}"
            ),
            Self::InvalidValue {
                field,
                value,
                bitmask,
            } => write!(
                f,
                "invalid {field} field value: {value:#x}, valid bitmask: {bitmask:#x}",
            ),
            Self::InvalidVariant { field, value } => {
                write!(f, "invalid {field} field variant: {value:#x}")
            }
            Self::Unimplemented => write!(f, "unimplemented"),
        }
    }
}
