//! CLIC interrupt enable register.

use crate::common::{Reg, RW};

/// CLIC interrupt enable register.
///
/// Each interrupt input has a dedicated interrupt-enable bit (clicintie[i]) and occupies one byte
/// in the memory map for ease of access. This control bit is read-write to enable/disable the
/// corresponding interrupt. The enable bit is located in bit 0 of the byte. Software should assume
/// clicintie[i]=0 means no interrupt enabled, and clicintie[i]!=0 indicates an interrupt is enabled
/// to accommodate possible future expansion of the clicintie field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTIE {
    ptr: *mut u8,
}

impl INTIE {
    /// Creates a new interrupt enable register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt enable register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn address(self) -> usize {
        self.ptr as _
    }

    /// Checks if an interrupt source is enabled.
    #[inline]
    pub fn is_enabled(self) -> bool {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        // > Software should assume clicintie[i]=0 means no interrupt enabled, and clicintie[i]!=0
        // > indicates an interrupt is enabled to accommodate possible future expansion of the
        // > clicintie field.
        reg.read() != 0
    }

    /// Enables an interrupt source.
    ///
    /// # Safety
    ///
    /// * Enabling an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(self) {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        // >  The enable bit is located in bit 0 of the byte.
        reg.set_bit(0);
    }

    /// Disables an interrupt source.
    #[inline]
    pub fn disable(self) {
        // SAFETY: valid interrupt number
        let reg: Reg<u8, RW> = unsafe { Reg::new(self.ptr) };

        // >  The enable bit is located in bit 0 of the byte.
        reg.clear_bit(0);
    }
}
