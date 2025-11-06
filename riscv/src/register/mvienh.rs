//! mvienh register

use crate::bits::{bf_extract, bf_insert};
use riscv_pac::result::{Error, Result};
use riscv_pac::InterruptNumber;

read_write_csr! {
    /// `mvienh` register
    Mvienh: 0x318,
    mask: 0xffff_ffff,
}

set!(0x318);
clear!(0x318);

impl Mvienh {
    /// Represents the value to shift interrupt numbers to their relative value.
    pub const INTERRUPT_SHIFT: usize = 32;
    /// Represents the minimum interrupt of the unlabelled virtual interrupt range.
    pub const MIN_INTERRUPT: usize = 32;
    /// Represents the maximum interrupt of the unlabelled virtual interrupt range.
    pub const MAX_INTERRUPT: usize = 63;

    /// Gets whether the interrupt number is a valid virtual interrupt.
    #[inline]
    pub const fn is_valid_interrupt(int: usize) -> bool {
        matches!(int, Self::MIN_INTERRUPT..=Self::MAX_INTERRUPT)
    }

    /// Shifts the high-order interrupt number bits down to their relative value.
    #[inline]
    pub const fn shift_interrupt(int: usize) -> usize {
        int.saturating_sub(Self::INTERRUPT_SHIFT)
    }

    /// Check if a specific core interrupt source is enabled.
    ///
    /// Returns `Error` if the interrupt number is invalid.
    #[inline]
    pub fn is_enabled<I: InterruptNumber>(&self, interrupt: I) -> bool {
        let n = interrupt.number();

        Self::is_valid_interrupt(n) && bf_extract(self.bits, Self::shift_interrupt(n), 1) != 0
    }

    /// Enable a specific core interrupt source.
    #[inline]
    pub fn enable<I: InterruptNumber>(&mut self, interrupt: I) -> Result<()> {
        let n = interrupt.number();

        if Self::is_valid_interrupt(n) {
            self.bits = bf_insert(self.bits, Self::shift_interrupt(n), 1, 1);
            Ok(())
        } else {
            Err(Error::InvalidVariant(n))
        }
    }

    /// Disable a specific core interrupt source.
    #[inline]
    pub fn disable<I: InterruptNumber>(&mut self, interrupt: I) -> Result<()> {
        let n = interrupt.number();

        if Self::is_valid_interrupt(n) {
            self.bits = bf_insert(self.bits, Self::shift_interrupt(n), 1, 0);
            Ok(())
        } else {
            Err(Error::InvalidVariant(n))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Represents a custom set of virtual interrupts.
    ///
    /// NOTE: a real implementation may want to enumerate the valid virtual interrupt variants.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct VirtualInterrupt(usize);

    /// SAFETY: `VirtualInterrupt` represents the virtual RISC-V interrupts
    unsafe impl InterruptNumber for VirtualInterrupt {
        const MAX_INTERRUPT_NUMBER: usize = Mvienh::MAX_INTERRUPT;

        #[inline]
        fn number(self) -> usize {
            self.0
        }

        #[inline]
        fn from_number(value: usize) -> Result<Self> {
            if Mvienh::is_valid_interrupt(value) {
                Ok(Self(value))
            } else {
                Err(Error::InvalidVariant(value))
            }
        }
    }

    #[test]
    fn test_mvienh() {
        let mut m = Mvienh::from_bits(0);

        (Mvienh::MIN_INTERRUPT..=Mvienh::MAX_INTERRUPT)
            .filter_map(|n| VirtualInterrupt::from_number(n).ok())
            .for_each(|int| {
                assert!(!m.is_enabled(int));

                assert!(m.enable(int).is_ok());
                assert!(m.is_enabled(int));

                assert!(m.disable(int).is_ok());
                assert!(!m.is_enabled(int));
            });
    }
}
