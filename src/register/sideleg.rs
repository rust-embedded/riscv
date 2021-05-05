//! sideleg register

use bit_field::BitField;

/// sideleg register
#[derive(Clone, Copy, Debug)]
pub struct Sideleg {
    bits: usize,
}

impl Sideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Delegate
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Timer Interrupt Delegate
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// User External Interrupt Delegate
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Sideleg, 0x103, __read_sideleg);
set!(0x103, __set_sideleg);
clear!(0x103, __clear_sideleg);

set_clear_csr!(
    /// User Software Interrupt Delegate
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// User Timer Interrupt Delegate
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// User External Interrupt Delegate
    , set_uext, clear_uext, 1 << 8);
