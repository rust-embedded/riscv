//! `mconfigptr` register.

use crate::result::{Error, Result};

const MASK: usize = usize::MAX;

read_only_csr! {
    /// `mconfigptr` register.
    Mconfigptr: 0xf15,
    mask: MASK,
    sentinel: 0,
}

impl Mconfigptr {
    /// Represents the bitshift for a properly aligned configuration pointer.
    pub const ALIGN_SHIFT: usize = (usize::BITS / 8).ilog2() as usize;
    /// Represents the bitmask for a properly aligned configuration pointer.
    pub const ALIGN_MASK: usize = (1usize << Self::ALIGN_SHIFT) - 1;

    /// Gets the pointer to the machine configuration structure.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - the value is `0`, indicating no configuration structure
    /// - the pointer is not aligned to an MXLEN byte value
    pub fn as_ptr(&self) -> *const u8 {
        self.try_as_ptr().unwrap()
    }

    /// Attempts to get the pointer to the machine configuration structure.
    ///
    /// # Note
    ///
    /// Returns an error if:
    ///
    /// - the value is `0`, indicating no configuration structure
    /// - the pointer is not aligned to an MXLEN byte value
    pub const fn try_as_ptr(&self) -> Result<*const u8> {
        match self.bits() {
            0 => Err(Error::InvalidFieldVariant {
                field: "mconfigptr",
                value: 0,
            }),
            p if p & Self::ALIGN_MASK != 0 => Err(Error::InvalidFieldValue {
                field: "mconfigptr",
                value: p,
                bitmask: !Self::ALIGN_MASK,
            }),
            p => Ok(p as *const _),
        }
    }
}
