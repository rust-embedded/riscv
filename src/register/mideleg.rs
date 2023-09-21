//! mideleg register

/// mideleg register
#[derive(Clone, Copy, Debug)]
pub struct Mideleg {
    bits: usize,
}

impl Mideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Supervisor Software Interrupt Delegate
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Supervisor Timer Interrupt Delegate
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// Supervisor External Interrupt Delegate
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits & (1 << 9) != 0
    }
}

read_csr_as!(Mideleg, 0x303);
set!(0x303);
clear!(0x303);

set_clear_csr!(
    /// Supervisor Software Interrupt Delegate
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Supervisor Timer Interrupt Delegate
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Supervisor External Interrupt Delegate
    , set_sext, clear_sext, 1 << 9);
