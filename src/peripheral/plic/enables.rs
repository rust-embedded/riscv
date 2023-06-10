//! Interrupt enables register.

use super::{InterruptNumber, ENABLES, PLIC};
use crate::peripheral::common::{Reg, RW};

impl ENABLES {
    /// Creates a new Interrupts enables register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts enables register.
    #[inline(always)]
    pub unsafe fn new(address: usize) -> Self {
        Self {
            base: ENABLE::new(address as _),
        }
    }

    /// Checks if an interrupt source is enabled.
    #[inline(always)]
    pub fn is_enabled<I: InterruptNumber>(&self, source: I) -> bool {
        let source = source.number() as u32;
        let offset = (source / PLIC::MAX_FLAGS_WORDS) as _;
        // SAFETY: valid interrupt number
        let reg = unsafe { ENABLE::new(self.base.get_ptr().offset(offset)) };
        reg.read_bit(source % PLIC::MAX_FLAGS_WORDS)
    }

    /// Enables an interrupt source.
    ///
    /// # Note
    ///
    /// It performs non-atomic read-modify-write operations, which may lead to **wrong** behavior.
    ///
    /// # Safety
    ///
    /// Enabling an interrupt source can break mask-based critical sections.
    #[inline(always)]
    pub unsafe fn enable<I: InterruptNumber>(&self, source: I) {
        let source: u32 = source.number() as _;
        let offset = (source / PLIC::MAX_FLAGS_WORDS) as _;
        // SAFETY: valid interrupt number
        let reg = unsafe { ENABLE::new(self.base.get_ptr().offset(offset)) };
        reg.set_bit(source % PLIC::MAX_FLAGS_WORDS)
    }

    /// Disables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// It performs non-atomic read-modify-write operations, which may lead to **wrong** behavior.
    #[inline(always)]
    pub fn disable<I: InterruptNumber>(&self, source: I) {
        let source: u32 = source.number() as _;
        let offset = (source / PLIC::MAX_FLAGS_WORDS) as _;
        // SAFETY: valid interrupt number
        let reg = unsafe { ENABLE::new(self.base.get_ptr().offset(offset)) };
        reg.clear_bit(source % PLIC::MAX_FLAGS_WORDS);
    }
}

pub type ENABLE = Reg<u32, RW>;
