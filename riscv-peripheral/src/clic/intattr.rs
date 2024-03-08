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
    ptr: *mut u32,
}

impl INTATTR {
    const INTATTR_OFFSET: usize = 0x2;

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
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bits(6 + 8 * Self::INTATTR_OFFSET, 7 + 8 * Self::INTATTR_OFFSET) {
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
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(
            6 + 8 * Self::INTATTR_OFFSET,
            7 + 8 * Self::INTATTR_OFFSET,
            mode as _,
        )
    }

    /// Check the trigger type for this interrupt.
    #[inline]
    pub fn trig(self) -> Trig {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bit(1 + 8 * Self::INTATTR_OFFSET) {
            false => Trig::Level,
            true => Trig::Edge,
        }
    }

    /// Set the trigger type for this interrupt.
    #[inline]
    pub fn set_trig(self, trig: Trig) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        match trig {
            Trig::Level => reg.clear_bit(1 + 8 * Self::INTATTR_OFFSET),
            Trig::Edge => reg.set_bit(1 + 8 * Self::INTATTR_OFFSET),
        }
    }

    /// Check the polarity for this interrupt.
    #[inline]
    pub fn polarity(self) -> Polarity {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        match reg.read_bit(2 + 8 * Self::INTATTR_OFFSET) {
            false => Polarity::Pos,
            true => Polarity::Neg,
        }
    }

    /// Set the polarity for this interrupt.
    #[inline]
    pub fn set_polarity(self, polarity: Polarity) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        match polarity {
            Polarity::Pos => reg.clear_bit(2 + 8 * Self::INTATTR_OFFSET),
            Polarity::Neg => reg.set_bit(2 + 8 * Self::INTATTR_OFFSET),
        }
    }

    /// Check the selective hardware vectoring mode for this interrupt.
    #[inline]
    #[cfg(feature = "clic-smclicshv")]
    pub fn shv(self) -> bool {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.read_bit(0 + 8 * Self::INTATTR_OFFSET)
    }

    /// Set selective hardware vectoring mode for this interrupt.
    #[inline]
    #[cfg(feature = "clic-smclicshv")]
    pub fn set_shv(self, shv: bool) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        if shv {
            reg.set_bit(8 * Self::INTATTR_OFFSET)
        } else {
            reg.clear_bit(8 * Self::INTATTR_OFFSET)
        }
    }
}
