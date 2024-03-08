//! CLIC interrupt pending register.

use crate::common::{Reg, RW};

/// CLIC interrupt pending register.
///
/// > When the input is configured for edge-sensitive input, clicintip[i] is a read-write register
/// > that can be updated both by hardware interrupt inputs and by software. The bit is set by
/// > hardware after an edge of the appropriate polarity is observed on the interrupt input, as
/// > determined by the clicintattr[i] field. Software writes to clicintip[i] can set or clear
/// > edge-triggered pending bits directly by writes to the memory-mapped register. Edge-triggered
/// > pending bits can also be cleared when a CSR instruction that accesses xnxti includes a write.
/// > clicintip[i] behavior is unaffected by clicintie[i] setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTIP {
    ptr: *mut u32,
}

impl INTIP {
    const INTIP_OFFSET: usize = 0x0;

    /// Creates a new interrupt pending register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt pending register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    /// Checks if an interrupt source is pending.
    ///
    /// # Safety
    ///
    /// * The value in the clicintip[i] is undefined when switching from level-sensitive mode to
    ///   edge-triggered mode in clicintattr[i].
    /// * Software cannot rely on the underlying clicintip[i] register bits used in edge-triggered
    ///   mode to hold state while in level-sensitive mode.
    #[inline]
    pub unsafe fn is_pending(self) -> bool {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // > Software should assume clicintip[i]=0 means no interrupt pending, and clicintip[i]!=0
        // > indicates an interrupt is pending to accommodate possible future expansion of the
        // > clicintip field.
        reg.read_bit(0 + 8 * Self::INTIP_OFFSET)
    }

    /// Pends an interrupt source.
    ///
    /// # Safety
    ///
    /// * Interrupt controller may or may not support software writes.
    #[inline]
    pub unsafe fn pend(self) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // >  The enable bit is located in bit 0 of the byte.
        reg.set_bit(0 + 8 * Self::INTIP_OFFSET);
    }

    /// Unpends an interrupt source.
    ///
    /// # Safety
    ///
    /// * Interrupt controller may or may not support software writes.
    #[inline]
    pub unsafe fn unpend(self) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        // >  The enable bit is located in bit 0 of the byte.
        reg.clear_bit(0 + 8 * Self::INTIP_OFFSET);
    }
}
