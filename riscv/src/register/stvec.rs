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
    /// Creates a new `Stvec` with the given address and trap mode.
    ///
    /// # Note
    ///
    /// Panics if the address is not aligned to 4-bytes.
    #[inline]
    pub fn new(address: usize, trap_mode: TrapMode) -> Self {
        Self::try_new(address, trap_mode).unwrap()
    }

    /// Attempts to create a new `Stvec` with the given address and trap mode.
    #[inline]
    pub fn try_new(address: usize, trap_mode: TrapMode) -> Result<Self> {
        let mut stvec = Stvec::from_bits(0);
        stvec.try_set_address(address)?;
        stvec.set_trap_mode(trap_mode);
        Ok(stvec)
    }

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

impl Stvec {
    /// Bit shift of the trap-vector base address (`BASE`) field.
    ///
    /// The `MODE` field consts (`TRAP_MODE_SHIFT`/`TRAP_MODE_WIDTH`/
    /// `TRAP_MODE_MASK`) are generated from the [`trap_mode`](Self::trap_mode)
    /// field.
    pub const BASE_SHIFT: usize = 2;
    /// Field-shifted bitmask of the trap-vector base address (`BASE`) field.
    pub const BASE_MASK: usize = !((1 << Self::BASE_SHIFT) - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stvec() {
        let mut stvec = Stvec::from_bits(0);

        [TrapMode::Direct, TrapMode::Vectored]
            .into_iter()
            .for_each(|trap_mode| {
                test_csr_field!(stvec, trap_mode: trap_mode);
            });

        (1..=usize::BITS)
            .map(|r| (((1u128 << r) - 1) as usize) & !TRAP_MASK)
            .for_each(|address| {
                stvec.set_address(address);
                assert_eq!(stvec.address(), address);

                assert_eq!(stvec.try_set_address(address), Ok(()));
                assert_eq!(stvec.address(), address);
            });

        (1..=usize::BITS)
            .filter_map(|r| match ((1u128 << r) - 1) as usize {
                addr if (addr & TRAP_MASK) != 0 => Some(addr),
                _ => None,
            })
            .for_each(|address| {
                assert_eq!(
                    stvec.try_set_address(address),
                    Err(Error::InvalidFieldVariant {
                        field: "stvec::address",
                        value: address,
                    })
                );
            });
    }
}
