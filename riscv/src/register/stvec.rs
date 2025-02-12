//! stvec register

pub use crate::register::mtvec::TrapMode;
use crate::result::{Error, Result};

const TRAP_MASK: usize = 0b11;

read_write_csr! {
    /// stvec register
    Stvec: 0x105,
    mask: usize::MAX,
}

read_write_csr_field! {
    Stvec,
    /// Returns the trap-vector mode
    trap_mode,
    TrapMode: [0:1],
}

impl Stvec {
    /// Returns the trap-vector base-address
    #[inline]
    pub const fn address(&self) -> usize {
        self.bits & !TRAP_MASK
    }

    /// Sets the trap-vector base-address.
    ///
    /// # Note
    ///
    /// Panics if the address is not aligned to 4-bytes.
    #[inline]
    pub fn set_address(&mut self, address: usize) {
        self.try_set_address(address).unwrap();
    }

    /// Attempts to set the trap-vector base-address.
    ///
    /// # Note
    ///
    /// Returns an error if the address is not aligned to 4-bytes.
    #[inline]
    pub fn try_set_address(&mut self, address: usize) -> Result<()> {
        // check for four-byte alignment
        if (address & TRAP_MASK) != 0 {
            Err(Error::InvalidFieldVariant {
                field: "stvec::address",
                value: address,
            })
        } else {
            self.bits = address | (self.bits & TRAP_MASK);
            Ok(())
        }
    }
}
