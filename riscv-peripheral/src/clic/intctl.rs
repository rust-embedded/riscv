//! CLIC interrupt control register.

use crate::common::{Reg, RW};

/// CLIC interrupt control register.
///
/// A configurable number of upper bits in clicintctl[i] are assigned to encode the interrupt
/// level.
///
/// The least-significant bits in clicintctl[i] that are not configured to be part of the
/// interrupt level are interrupt priority, which are used to prioritize among interrupts
/// pending-and-enabled at the same privilege mode and interrupt level. The highest-priority
/// interrupt at a given privilege mode and interrupt level is taken first. In case there are
/// multiple pending-and-enabled interrupts at the same highest priority, the highest-numbered
/// interrupt is taken first.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTCTL {
    ptr: *mut u32,
}

impl INTCTL {
    const INTCTL_OFFSET: usize = 0x3;

    /// Creates a new interrupt control register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt control register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    /// Check interrupt level for this interrupt.
    #[inline]
    pub fn level(self) -> u8 {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // TODO: need to figure out how many are actually priority bits and how many are level bits
        // and mask accordingly
        reg.read_bits(8 * Self::INTCTL_OFFSET, 7 + 8 * Self::INTCTL_OFFSET) as u8
    }

    /// Set interrupt level for this interrupt.
    #[inline]
    pub fn set_level(self, level: u8) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // TODO: need to figure out how many are actually priority bits and how many are level bits
        // and mask accordingly
        reg.write_bits(
            8 * Self::INTCTL_OFFSET,
            7 + 8 * Self::INTCTL_OFFSET,
            level as _,
        )
    }

    /// Check interrupt priority for this interrupt.
    ///
    /// N.b., 2024-03-11 rt-ss/CLIC does not implement priority bits at all at this time
    #[inline]
    pub fn priority(self) -> u32 {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // TODO: need to figure out how many are actually priority bits and how many are level bits
        // and mask accordingly
        reg.read()
    }

    /// Set interrupt priority for this interrupt.
    ///
    /// N.b., 2024-03-11 rt-ss/CLIC does not implement priority bits at all at this time
    #[inline]
    pub fn set_priority(self, priority: u32) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // TODO: need to figure out how many are actually priority bits and how many are level bits
        // and mask accordingly
        reg.write(priority)
    }
}
