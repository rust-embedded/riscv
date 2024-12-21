//! mtvec register

use crate::result::{Error, Result};

const MASK: usize = usize::MAX;
const TRAP_MASK: usize = 0b11;

read_write_csr! {
    /// mtvec register
    Mtvec: 0x305,
    mask: MASK,
}

csr_field_enum! {
    /// Trap mode
    TrapMode {
        default: Direct,
        Direct = 0,
        Vectored = 1,
    }
}

read_write_csr_field! {
    Mtvec,
    /// Accesses the trap-vector mode.
    trap_mode,
    TrapMode: [0:1],
}

impl Mtvec {
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
                field: "mtvec::address",
                value: address,
            })
        } else {
            self.bits = address | (self.bits & TRAP_MASK);
            Ok(())
        }
    }
}
