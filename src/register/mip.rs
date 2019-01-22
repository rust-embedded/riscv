//! mip register

/// mip register
#[derive(Clone, Copy, Debug)]
pub struct Mip {
    bits: usize,
}

impl Mip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Pending
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Software Interrupt Pending
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// Machine Software Interrupt Pending
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits & (1 << 3) == 1 << 3
    }

    /// User Timer Interrupt Pending
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Timer Interrupt Pending
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// Machine Timer Interrupt Pending
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    /// User External Interrupt Pending
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    /// Supervisor External Interrupt Pending
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }

    /// Machine External Interrupt Pending
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits & (1 << 11) == 1 << 11
    }
}

read_csr_as!(Mip, 0x344, __read_mip);
