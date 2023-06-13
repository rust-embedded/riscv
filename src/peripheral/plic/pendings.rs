//! Interrupt pending bits register.

use super::{InterruptNumber, PENDINGS};
use crate::peripheral::common::{Reg, RO};

impl PENDINGS {
    /// Creates a new Interrupts pending bits register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts pending bits register.
    pub unsafe fn new(address: usize) -> Self {
        Self { base: address }
    }

    /// Returns the base address of the Interrupts priorities register.
    #[inline(always)]
    pub fn ptr(&self) -> *mut u32 {
        self.base as _
    }

    /// Checks if an interrupt triggered by a given source is pending.
    pub fn is_pending<I: InterruptNumber>(&self, source: I) -> bool {
        let source = source.number() as u32;
        let offset = (source / u32::BITS) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RO> = unsafe { Reg::new(self.ptr().offset(offset)) };
        reg.read_bit(source % u32::BITS)
    }
}
