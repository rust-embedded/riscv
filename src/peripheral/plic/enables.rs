//! Interrupt enables register of a PLIC context.

use super::{InterruptNumber, ENABLES};
use crate::peripheral::common::{Reg, RW};

impl ENABLES {
    /// Creates a new Interrupts enables register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts enables register.
    #[inline(always)]
    pub unsafe fn new(address: usize) -> Self {
        Self { base: address }
    }

    /// Returns the base address of the Interrupts enables register.
    #[inline(always)]
    pub fn ptr(&self) -> *mut u32 {
        self.base as _
    }

    /// Checks if an interrupt source is enabled for the PLIC context.
    #[inline(always)]
    pub fn is_enabled<I: InterruptNumber>(&self, source: I) -> bool {
        let source = source.number() as u32;
        let offset = (source / u32::BITS) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr().offset(offset)) };
        reg.read_bit(source % u32::BITS)
    }

    /// Enables an interrupt source for the PLIC context.
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
        let offset = (source / u32::BITS) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr().offset(offset)) };
        reg.set_bit(source % u32::BITS)
    }

    /// Disables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// It performs non-atomic read-modify-write operations, which may lead to **wrong** behavior.
    #[inline(always)]
    pub fn disable<I: InterruptNumber>(&self, source: I) {
        let source: u32 = source.number() as _;
        let offset = (source / u32::BITS) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr().offset(offset)) };
        reg.clear_bit(source % u32::BITS);
    }

    /// Enables all the external interrupt sources for the PLIC context.
    ///
    /// # Safety
    ///
    /// Enabling all interrupt sources can break mask-based critical sections.
    #[inline(always)]
    pub unsafe fn enable_all<I: InterruptNumber>(&self) {
        for offset in 0..=(I::MAX_INTERRUPT_NUMBER as u32 / u32::BITS) as isize {
            // SAFETY: valid offset
            let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr().offset(offset)) };
            reg.write(0xFFFF_FFFF);
        }
    }

    /// Disables all the external interrupt sources for the PLIC context.
    #[inline(always)]
    pub fn disable_all<I: InterruptNumber>(&self) {
        for offset in 0..=(I::MAX_INTERRUPT_NUMBER as u32 / u32::BITS) as _ {
            // SAFETY: valid offset
            let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr().offset(offset)) };
            reg.write(0);
        }
    }
}
