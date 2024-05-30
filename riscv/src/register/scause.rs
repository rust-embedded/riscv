//! scause register

use riscv_pac::CoreInterruptNumber;
pub use riscv_pac::{ExceptionNumber, InterruptNumber}; // re-export useful riscv-pac traits

/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Interrupt {
    SupervisorSoft = 1,
    SupervisorTimer = 5,
    SupervisorExternal = 9,
}

/// SAFETY: `Interrupt` represents the standard RISC-V interrupts
unsafe impl InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: usize = Self::SupervisorExternal as usize;

    #[inline]
    fn number(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_number(value: usize) -> Result<Self, usize> {
        if value == 1 || value == 5 || value == 9 {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute::<usize, Self>(value) })
        } else {
            Err(value)
        }
    }
}

/// SAFETY: `Interrupt` represents the standard RISC-V core interrupts
unsafe impl CoreInterruptNumber for Interrupt {}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

/// SAFETY: `Exception` represents the standard RISC-V exceptions
unsafe impl ExceptionNumber for Exception {
    const MAX_EXCEPTION_NUMBER: usize = Self::StorePageFault as usize;

    #[inline]
    fn number(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_number(value: usize) -> Result<Self, usize> {
        if value == 10 || value == 11 || value == 14 || value > 15 {
            Err(value)
        } else {
            // SAFETY: valid exception number
            unsafe { Ok(core::mem::transmute::<usize, Self>(value)) }
        }
    }
}

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

    /// Try to get the trap cause
    #[inline]
    pub fn try_cause<I, E>(&self) -> Result<Trap<I, E>, TrapError>
    where
        I: CoreInterruptNumber,
        E: ExceptionNumber,
    {
        if self.is_interrupt() {
            match I::from_number(self.code()) {
                Ok(interrupt) => Ok(Trap::Interrupt(interrupt)),
                Err(code) => Err(TrapError::InvalidInterrupt(code)),
            }
        } else {
            match E::from_number(self.code()) {
                Ok(exception) => Ok(Trap::Exception(exception)),
                Err(code) => Err(TrapError::InvalidException(code)),
            }
        }
    }

    /// Trap Cause
    #[inline]
    pub fn cause<I: CoreInterruptNumber, E: ExceptionNumber>(&self) -> Trap<I, E> {
        self.try_cause().unwrap()
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

#[cfg(test)]
mod test {
    use super::*;
    use Exception::*;
    use Interrupt::*;

    #[test]
    fn test_interrupt() {
        assert_eq!(Interrupt::from_number(1), Ok(SupervisorSoft));
        assert_eq!(Interrupt::from_number(2), Err(2));
        assert_eq!(Interrupt::from_number(3), Err(3));
        assert_eq!(Interrupt::from_number(4), Err(4));
        assert_eq!(Interrupt::from_number(5), Ok(SupervisorTimer));
        assert_eq!(Interrupt::from_number(6), Err(6));
        assert_eq!(Interrupt::from_number(7), Err(7));
        assert_eq!(Interrupt::from_number(8), Err(8));
        assert_eq!(Interrupt::from_number(9), Ok(SupervisorExternal));
        assert_eq!(Interrupt::from_number(10), Err(10));
        assert_eq!(Interrupt::from_number(11), Err(11));
        assert_eq!(Interrupt::from_number(12), Err(12));

        assert_eq!(SupervisorSoft.number(), 1);
        assert_eq!(SupervisorTimer.number(), 5);
        assert_eq!(SupervisorExternal.number(), 9);

        assert_eq!(SupervisorExternal.number(), Interrupt::MAX_INTERRUPT_NUMBER)
    }

    #[test]
    fn test_exception() {
        assert_eq!(Exception::from_number(0), Ok(InstructionMisaligned));
        assert_eq!(Exception::from_number(1), Ok(InstructionFault));
        assert_eq!(Exception::from_number(2), Ok(IllegalInstruction));
        assert_eq!(Exception::from_number(3), Ok(Breakpoint));
        assert_eq!(Exception::from_number(4), Ok(LoadMisaligned));
        assert_eq!(Exception::from_number(5), Ok(LoadFault));
        assert_eq!(Exception::from_number(6), Ok(StoreMisaligned));
        assert_eq!(Exception::from_number(7), Ok(StoreFault));
        assert_eq!(Exception::from_number(8), Ok(UserEnvCall));
        assert_eq!(Exception::from_number(9), Ok(SupervisorEnvCall));
        assert_eq!(Exception::from_number(10), Err(10));
        assert_eq!(Exception::from_number(11), Err(11));
        assert_eq!(Exception::from_number(12), Ok(InstructionPageFault));
        assert_eq!(Exception::from_number(13), Ok(LoadPageFault));
        assert_eq!(Exception::from_number(14), Err(14));
        assert_eq!(Exception::from_number(15), Ok(StorePageFault));
        assert_eq!(Exception::from_number(16), Err(16));

        assert_eq!(InstructionMisaligned.number(), 0);
        assert_eq!(InstructionFault.number(), 1);
        assert_eq!(IllegalInstruction.number(), 2);
        assert_eq!(Breakpoint.number(), 3);
        assert_eq!(LoadMisaligned.number(), 4);
        assert_eq!(LoadFault.number(), 5);
        assert_eq!(StoreMisaligned.number(), 6);
        assert_eq!(StoreFault.number(), 7);
        assert_eq!(UserEnvCall.number(), 8);
        assert_eq!(SupervisorEnvCall.number(), 9);
        assert_eq!(InstructionPageFault.number(), 12);
        assert_eq!(LoadPageFault.number(), 13);
        assert_eq!(StorePageFault.number(), 15);

        assert_eq!(StorePageFault.number(), Exception::MAX_EXCEPTION_NUMBER)
    }
}
