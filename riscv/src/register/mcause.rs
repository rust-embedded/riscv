//! mcause register

pub use crate::interrupt::Trap;
use crate::{CoreInterruptNumber, ExceptionNumber};

read_only_csr! {
    /// `mcause` register
    Mcause: 0x342,
    mask: usize::MAX,
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Mcause,
    /// Returns the `code` field.
    code: [0:30],
}

#[cfg(not(target_arch = "riscv32"))]
read_only_csr_field! {
    Mcause,
    /// Returns the `code` field.
    code: [0:62],
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Mcause,
    /// Is the trap cause an interrupt.
    is_interrupt: 31,
}

#[cfg(not(target_arch = "riscv32"))]
read_only_csr_field! {
    Mcause,
    /// Is the trap cause an interrupt.
    is_interrupt: 63,
}

impl Mcause {
    /// Creates an `Mcause` value representing the given core interrupt source.
    ///
    /// The [interrupt bit](Self::IS_INTERRUPT_MASK) is set and the `code` field
    /// is set to the interrupt number.
    #[inline]
    pub fn from_interrupt<I: CoreInterruptNumber>(interrupt: I) -> Self {
        Self::from_bits(Self::IS_INTERRUPT_MASK | interrupt.number())
    }

    /// Creates an `Mcause` value representing the given exception source.
    ///
    /// The interrupt bit is clear and the `code` field is set to the exception
    /// number.
    #[inline]
    pub fn from_exception<E: ExceptionNumber>(exception: E) -> Self {
        Self::from_bits(exception.number())
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

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interrupt::machine::{Exception, Interrupt};

    #[test]
    fn test_mcause() {
        let m = Mcause::from_interrupt(Interrupt::MachineExternal);
        assert!(m.is_interrupt());
        assert!(!m.is_exception());
        assert_eq!(m.code(), Interrupt::MachineExternal as usize);
        assert_eq!(
            m.cause(),
            Trap::Interrupt(Interrupt::MachineExternal as usize)
        );

        let m = Mcause::from_exception(Exception::Breakpoint);
        assert!(m.is_exception());
        assert!(!m.is_interrupt());
        assert_eq!(m.code(), Exception::Breakpoint as usize);
        assert_eq!(m.cause(), Trap::Exception(Exception::Breakpoint as usize));
    }
}
