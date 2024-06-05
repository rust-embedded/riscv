//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs

pub use riscv_pac::{CoreInterruptNumber, ExceptionNumber, InterruptNumber}; // re-export useful riscv-pac traits

pub mod machine;
pub mod supervisor;

#[cfg(not(feature = "s-mode"))]
pub use machine::*;
#[cfg(feature = "s-mode")]
pub use supervisor::*;

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap<I, E> {
    Interrupt(I),
    Exception(E),
}

/// Trap Error
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrapError {
    InvalidInterrupt(usize),
    InvalidException(usize),
}

impl Trap<usize, usize> {
    /// Converts a target-specific trap cause to a generic trap cause
    #[inline]
    pub fn from<I: CoreInterruptNumber, E: ExceptionNumber>(trap: Trap<I, E>) -> Self {
        match trap {
            Trap::Interrupt(interrupt) => Trap::Interrupt(interrupt.number()),
            Trap::Exception(exception) => Trap::Exception(exception.number()),
        }
    }

    /// Tries to convert the generic trap cause to a target-specific trap cause
    #[inline]
    pub fn try_into<I, E>(self) -> Result<Trap<I, E>, TrapError>
    where
        I: CoreInterruptNumber,
        E: ExceptionNumber,
    {
        match self {
            Trap::Interrupt(code) => match I::from_number(code) {
                Ok(interrupt) => Ok(Trap::Interrupt(interrupt)),
                Err(code) => Err(TrapError::InvalidInterrupt(code)),
            },
            Trap::Exception(code) => match E::from_number(code) {
                Ok(exception) => Ok(Trap::Exception(exception)),
                Err(code) => Err(TrapError::InvalidException(code)),
            },
        }
    }
}

impl<I: CoreInterruptNumber, E: ExceptionNumber> Trap<I, E> {
    /// Converts a target-specific trap cause to a generic trap cause
    #[inline]
    pub fn into(self) -> Trap<usize, usize> {
        Trap::from(self)
    }

    /// Tries to convert the generic trap cause to a target-specific trap cause
    #[inline]
    pub fn try_from(trap: Trap<usize, usize>) -> Result<Self, TrapError> {
        trap.try_into()
    }
}
