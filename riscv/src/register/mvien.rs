//! mvien register

use crate::bits::{bf_extract, bf_insert};
use riscv_pac::result::{Error, Result};
use riscv_pac::InterruptNumber;

#[cfg(target_arch = "riscv32")]
const MASK: usize = 0xffff_e222;
#[cfg(not(target_arch = "riscv32"))]
const MASK: usize = 0xffff_ffff_ffff_e222;

read_write_csr! {
    /// `mvien` register
    Mvien: 0x308,
    mask: MASK,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mie.SSIE`
    ssoft: 1,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mie.STIE`
    stimer: 5,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mie.SEIE`
    sext: 9,
}

impl Mvien {
    /// Represents the minimum interrupt of the unlabelled virtual interrupt range.
    pub const MIN_INTERRUPT: usize = 13;
    /// Represents the maximum interrupt of the unlabelled virtual interrupt range.
    #[cfg(target_arch = "riscv32")]
    pub const MAX_INTERRUPT: usize = 31;
    /// Represents the maximum interrupt of the unlabelled virtual interrupt range.
    #[cfg(not(target_arch = "riscv32"))]
    pub const MAX_INTERRUPT: usize = 63;

    /// Gets whether the interrupt number is a valid virtual interrupt.
    #[inline]
    pub const fn is_valid_interrupt(int: usize) -> bool {
        matches!(int, 1 | 5 | 9 | Self::MIN_INTERRUPT..=Self::MAX_INTERRUPT)
    }

    /// Check if a specific core interrupt source is enabled.
    ///
    /// Returns `Error` if the interrupt number is invalid.
    #[inline]
    pub fn is_enabled<I: InterruptNumber>(&self, interrupt: I) -> bool {
        let n = interrupt.number();

        Self::is_valid_interrupt(n) && bf_extract(self.bits, n, 1) != 0
    }

    /// Enable a specific core interrupt source.
    ///
    /// Returns `Error` if the interrupt number is invalid.
    #[inline]
    pub fn enable<I: InterruptNumber>(&mut self, interrupt: I) -> Result<()> {
        let n = interrupt.number();

        if Self::is_valid_interrupt(n) {
            self.bits = bf_insert(self.bits, n, 1, 1);
            Ok(())
        } else {
            Err(Error::InvalidVariant(n))
        }
    }

    /// Disable a specific core interrupt source.
    ///
    /// Returns `Error` if the interrupt number is invalid.
    #[inline]
    pub fn disable<I: InterruptNumber>(&mut self, interrupt: I) -> Result<()> {
        let n = interrupt.number();

        if Self::is_valid_interrupt(n) {
            self.bits = bf_insert(self.bits, n, 1, 0);
            Ok(())
        } else {
            Err(Error::InvalidVariant(n))
        }
    }
}

set!(0x308);
clear!(0x308);

set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);

read_composite_csr!(super::mvienh::read().bits(), read().bits());

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
        const MAX_INTERRUPT_NUMBER: usize = Mvien::MAX_INTERRUPT;

        #[inline]
        fn number(self) -> usize {
            self.0
        }

        #[inline]
        fn from_number(value: usize) -> Result<Self> {
            if Mvien::is_valid_interrupt(value) {
                Ok(Self(value))
            } else {
                Err(Error::InvalidVariant(value))
            }
        }
    }

    #[test]
    fn test_mvien() {
        let mut m = Mvien::from_bits(0);

        test_csr_field!(m, ssoft);
        test_csr_field!(m, stimer);
        test_csr_field!(m, sext);

        (0..=VirtualInterrupt::MAX_INTERRUPT_NUMBER)
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
