//! scause register

/// scause register
#[derive(Clone, Copy)]
pub struct Scause {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Interrupt {
    SupervisorSoft = 1,
    SupervisorTimer = 5,
    SupervisorExternal = 9,
    Unknown,
}

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
    Unknown,
}

impl From<usize> for Interrupt {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            1 => Self::SupervisorSoft,
            5 => Self::SupervisorTimer,
            9 => Self::SupervisorExternal,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Interrupt> for usize {
    type Error = Interrupt;

    #[inline]
    fn try_from(value: Interrupt) -> Result<Self, Self::Error> {
        match value {
            Interrupt::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
        }
    }
}

impl From<usize> for Exception {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            0 => Self::InstructionMisaligned,
            1 => Self::InstructionFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadMisaligned,
            5 => Self::LoadFault,
            6 => Self::StoreMisaligned,
            7 => Self::StoreFault,
            8 => Self::UserEnvCall,
            9 => Self::SupervisorEnvCall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Exception> for usize {
    type Error = Exception;

    #[inline]
    fn try_from(value: Exception) -> Result<Self, Self::Error> {
        match value {
            Exception::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
        }
    }
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
pub unsafe fn set(cause: Trap) {
    let bits = match cause {
        Trap::Interrupt(i) => {
            let i = usize::try_from(i).expect("unknown interrupt");
            i | (1 << (usize::BITS as usize - 1)) // interrupt bit is 1
        }
        Trap::Exception(e) => usize::try_from(e).expect("unknown exception"),
    };
    _write(bits);
}
