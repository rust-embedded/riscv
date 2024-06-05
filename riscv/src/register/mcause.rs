//! mcause register

use riscv_pac::CoreInterruptNumber;
pub use riscv_pac::{ExceptionNumber, InterruptNumber}; // re-export useful riscv-pac traits

/// Standard M-mode RISC-V interrupts
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Interrupt {
    SupervisorSoft = 1,
    MachineSoft = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
}

/// SAFETY: `Interrupt` represents the standard RISC-V interrupts
unsafe impl InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: usize = Self::MachineExternal as usize;

    #[inline]
    fn number(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_number(value: usize) -> Result<Self, usize> {
        match value {
            1 => Ok(Self::SupervisorSoft),
            3 => Ok(Self::MachineSoft),
            5 => Ok(Self::SupervisorTimer),
            7 => Ok(Self::MachineTimer),
            9 => Ok(Self::SupervisorExternal),
            11 => Ok(Self::MachineExternal),
            _ => Err(value),
        }
    }
}

/// SAFETY: `Interrupt` represents the standard RISC-V core interrupts
unsafe impl CoreInterruptNumber for Interrupt {}

/// Standard M-mode RISC-V exceptions
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    MachineEnvCall = 11,
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
        match value {
            0 => Ok(Self::InstructionMisaligned),
            1 => Ok(Self::InstructionFault),
            2 => Ok(Self::IllegalInstruction),
            3 => Ok(Self::Breakpoint),
            4 => Ok(Self::LoadMisaligned),
            5 => Ok(Self::LoadFault),
            6 => Ok(Self::StoreMisaligned),
            7 => Ok(Self::StoreFault),
            8 => Ok(Self::UserEnvCall),
            9 => Ok(Self::SupervisorEnvCall),
            11 => Ok(Self::MachineEnvCall),
            12 => Ok(Self::InstructionPageFault),
            13 => Ok(Self::LoadPageFault),
            15 => Ok(Self::StorePageFault),
            _ => Err(value),
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

read_csr_as!(Mcause, 0x342);

#[cfg(test)]
mod test {
    use super::*;
    use Exception::*;
    use Interrupt::*;

    #[test]
    fn test_interrupt() {
        assert_eq!(Interrupt::from_number(1), Ok(SupervisorSoft));
        assert_eq!(Interrupt::from_number(2), Err(2));
        assert_eq!(Interrupt::from_number(3), Ok(MachineSoft));
        assert_eq!(Interrupt::from_number(4), Err(4));
        assert_eq!(Interrupt::from_number(5), Ok(SupervisorTimer));
        assert_eq!(Interrupt::from_number(6), Err(6));
        assert_eq!(Interrupt::from_number(7), Ok(MachineTimer));
        assert_eq!(Interrupt::from_number(8), Err(8));
        assert_eq!(Interrupt::from_number(9), Ok(SupervisorExternal));
        assert_eq!(Interrupt::from_number(10), Err(10));
        assert_eq!(Interrupt::from_number(11), Ok(MachineExternal));
        assert_eq!(Interrupt::from_number(12), Err(12));

        assert_eq!(SupervisorSoft.number(), 1);
        assert_eq!(MachineSoft.number(), 3);
        assert_eq!(SupervisorTimer.number(), 5);
        assert_eq!(MachineTimer.number(), 7);
        assert_eq!(SupervisorExternal.number(), 9);
        assert_eq!(MachineExternal.number(), 11);

        assert_eq!(MachineExternal.number(), Interrupt::MAX_INTERRUPT_NUMBER)
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
        assert_eq!(Exception::from_number(11), Ok(MachineEnvCall));
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
        assert_eq!(MachineEnvCall.number(), 11);
        assert_eq!(InstructionPageFault.number(), 12);
        assert_eq!(LoadPageFault.number(), 13);
        assert_eq!(StorePageFault.number(), 15);

        assert_eq!(StorePageFault.number(), Exception::MAX_EXCEPTION_NUMBER)
    }
}
