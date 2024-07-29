//! scause register

pub use crate::interrupt::Trap;
pub use riscv_pac::{CoreInterruptNumber, ExceptionNumber, InterruptNumber}; // re-export useful riscv-pac traits

/// scause register
#[derive(Clone, Copy)]
pub struct Scause {
    bits: usize,
}

impl Scause {
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
    /// To get a target-specific trap cause, use [`Trap::try_into`] with your target-specific S-Mode trap cause types.
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

read_csr_as!(Scause, 0x142);
write_csr!(0x142);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}

/// Set supervisor cause register to corresponding cause.
#[inline]
pub unsafe fn set<I: CoreInterruptNumber, E: ExceptionNumber>(cause: Trap<I, E>) {
    let bits = match cause {
        Trap::Interrupt(i) => {
            i.number() | (1 << (usize::BITS as usize - 1)) // interrupt bit is 1
        }
        Trap::Exception(e) => e.number(),
    };
    _write(bits);
}
