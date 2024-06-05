//! mcause register

pub use crate::interrupt::Trap;

/// mcause register
#[derive(Clone, Copy, Debug)]
pub struct Mcause {
    bits: usize,
}

impl From<usize> for Mcause {
    #[inline]
    fn from(bits: usize) -> Self {
        Self { bits }
    }
}

impl Mcause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        self.bits & !(1 << (usize::BITS as usize - 1))
    }

    /// Returns the trap cause represented by this register.
    ///
    /// # Note
    ///
    /// This method returns a **raw trap cause**, which means that values are represented as `usize`.
    /// To get a target-specific trap cause, use [`Trap::try_into`] with your target-specific M-Mode trap cause types.
    #[inline]
    pub fn cause(&self) -> Trap<usize, usize> {
        if self.is_interrupt() {
            Trap::Interrupt(self.code())
        } else {
            Trap::Exception(self.code())
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits & (1 << (usize::BITS as usize - 1)) != 0
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

read_csr_as!(Mcause, 0x342);
