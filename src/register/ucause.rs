/*!
    # `ucause` register

    `ucause` register is a read/write register. When a trap is taken into U-mode, `ucause` is written with a code indicating the event that caused the trap.

    Since normally interrupts related to S-mode and higher privilege are not delegated to U-mode, currently only interrupts related to U-mode like USI/UTI/UEI are supported.

    In addition, *Environment call from S-mode* isn't supported currently.
*/

use bit_field::BitField;
use core::mem::size_of;

/// ucause register
#[derive(Clone, Copy, Debug)]
pub struct Ucause {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoft,
    UserTimer,
    UserExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Interrupt {
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Interrupt::UserSoft,
            4 => Interrupt::UserTimer,
            8 => Interrupt::UserExternal,
            _ => Interrupt::Unknown,
        }
    }
}

impl Exception {
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            _ => Exception::Unknown,
        }
    }
}

impl Ucause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    pub fn code(&self) -> usize {
        let bit = 1 << (size_of::<usize>() * 8 - 1);
        self.bits & !bit
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

read_csr_as!(Ucause, 0x042, __read_ucause);
write_csr!(0x042, __write_ucause);

/// Writes the CSR
///
/// # Safety
///
/// May cause the software behave unexpectedly
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}

/// Set supervisor cause register to corresponding cause.
///
/// # Safety
///
/// May cause the software behave unexpectedly
#[inline]
pub unsafe fn set(cause: Trap) {
    let bits = match cause {
        Trap::Interrupt(i) => {
            (match i {
                Interrupt::UserSoft => 0,
                Interrupt::UserTimer => 4,
                Interrupt::UserExternal => 8,
                Interrupt::Unknown => panic!("unknown interrupt"),
            } | (1 << (size_of::<usize>() * 8 - 1)))
        } // interrupt bit is 1
        Trap::Exception(e) => match e {
            Exception::InstructionMisaligned => 0,
            Exception::InstructionFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadFault => 5,
            Exception::StoreMisaligned => 6,
            Exception::StoreFault => 7,
            Exception::UserEnvCall => 8,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
            Exception::Unknown => panic!("unknown exception"),
        }, // interrupt bit is 0
    };
    _write(bits);
}
