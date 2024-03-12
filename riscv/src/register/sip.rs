//! sip register

/// sip register
#[derive(Clone, Copy, Debug)]
pub struct Sip {
    bits: usize,
}

impl Sip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Supervisor Software Interrupt Pending
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Supervisor Timer Interrupt Pending
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// Supervisor External Interrupt Pending
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits & (1 << 9) != 0
    }
}

read_csr_as!(Sip, 0x144);
set!(0x144);
clear!(0x144);

set_clear_csr!(
    /// Supervisor Software Interrupt Pending
    , set_ssoft, clear_ssoft, 1 << 1);
