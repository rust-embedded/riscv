//! Interrupt pending bits register.

use super::{InterruptNumber, PENDINGS, PLIC};
use crate::peripheral::common::{Reg, RO};

impl PENDINGS {
    /// Creates a new Interrupts pending bits register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts pending bits register.
    pub unsafe fn new(address: usize) -> Self {
        Self {
            base: PENDING::new(address as _),
        }
    }

    /// Checks if an interrupt triggered by a given source is pending.
    pub fn is_pending<I: InterruptNumber>(&self, source: I) -> bool {
        let source = source.number() as u32;
        let offset = (source / PLIC::MAX_FLAGS_WORDS) as _;
        // SAFETY: valid interrupt number
        let reg = unsafe { PENDING::new(self.base.get_ptr().offset(offset)) };
        reg.read_bit(source % PLIC::MAX_FLAGS_WORDS)
    }
}

pub type PENDING = Reg<u32, RO>;
