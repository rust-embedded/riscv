//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs

use crate::result::Result;

// re-export useful riscv-pac traits
pub use riscv_pac::{CoreInterruptNumber, ExceptionNumber, InterruptNumber};

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
    pub fn try_into<I, E>(self) -> Result<Trap<I, E>>
    where
        I: CoreInterruptNumber,
        E: ExceptionNumber,
    {
        match self {
            Trap::Interrupt(code) => Ok(Trap::Interrupt(I::from_number(code)?)),
            Trap::Exception(code) => Ok(Trap::Exception(E::from_number(code)?)),
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
    pub fn try_from(trap: Trap<usize, usize>) -> Result<Self> {
        trap.try_into()
    }
}
