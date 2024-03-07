//! CLIC interrupt attribute register.

use crate::common::{Reg, RW};

/// Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Mode {
    Machine = 0b11,
    Supervisor = 0b01,
    User = 0b00,
}

/// Trigger type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Trig {
    Level = 0b0,
    Edge = 0b1,
}

/// Polarity
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Polarity {
    Pos = 0b0,
    Neg = 0b1,
}

/// CLIC interrupt attribute register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTATTR {
    ptr: *mut u8,
}

impl INTATTR {
    /// Creates a new interrupt attribute register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt attribute register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    /// Check which privilege mode this interrupt operates in.
    #[inline]
    pub fn mode(self) -> Mode {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bits(6, 7) {
            0b00 => Mode::User,
            0b01 => Mode::Supervisor,
            0b11 => Mode::Machine,
            _ => unreachable!(),
        }
    }

    /// Set the privilege mode this interrupt operates in.
    #[inline]
    pub fn set_mode(self, mode: Mode) {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(6, 7, mode as u8)
    }

    /// Check the trigger type for this interrupt.
    #[inline]
    pub fn trig(self) -> Trig {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bit(1) {
            false => Trig::Level,
            true => Trig::Edge,
        }
    }

    /// Set the trigger type for this interrupt.
    #[inline]
    pub fn set_trig(self, trig: Trig) {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(1, 1, trig as u8)
    }

    /// Check the polarity for this interrupt.
    #[inline]
    pub fn polarity(self) -> Polarity {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bit(2) {
            false => Polarity::Pos,
            true => Polarity::Neg,
        }
    }

    /// Set the polarity for this interrupt.
    #[inline]
    pub fn set_polarity(self, polarity: Polarity) {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(2, 2, polarity as u8)
    }
}
