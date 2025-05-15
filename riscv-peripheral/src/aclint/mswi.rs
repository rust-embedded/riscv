//! Machine-level Software Interrupt Device.

pub use super::{Clint, HartIdNumber};
use crate::common::unsafe_peripheral;
use riscv::register::{mhartid, mie, mip};

/// Trait for a Machine-level Software Interrupt device.
///
/// # Note
///
/// For CLINT peripherals, this trait is automatically implemented.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with an MSWI device.
/// * The MSWI device base address `BASE` must be valid for the target.
pub unsafe trait Mswi: Copy {
    /// Base address of the MSWI peripheral.
    const BASE: usize;
}

// SAFETY: the offset of the MSWI peripheral is fixed in the CLINT peripheral
unsafe impl<C: Clint> Mswi for C {
    const BASE: usize = C::BASE;
}

/// MSWI peripheral.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSWI<M> {
    _marker: core::marker::PhantomData<M>,
}

impl<M: Mswi> MSWI<M> {
    /// Creates a new `MSWI` device.
    #[inline]
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the base address of the `MSWI` device.
    #[inline]
    const fn as_ptr(&self) -> *const u32 {
        M::BASE as *const u32
    }

    /// Returns `true` if a machine software interrupt is pending.
    #[inline]
    pub fn is_interrupting(&self) -> bool {
        mip::read().msoft()
    }

    /// Returns `true` if machine software interrupts are enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        mie::read().msoft()
    }

    /// Enables machine software interrupts in the current HART.
    ///
    /// # Safety
    ///
    /// Enabling interrupts may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(&self) {
        mie::set_msoft();
    }

    /// Disables machine software interrupts in the current HART.
    #[inline]
    pub fn disable(&self) {
        // SAFETY: it is safe to disable interrupts
        unsafe { mie::clear_msoft() };
    }

    /// Returns the `MSIP` register for the HART which ID is `hart_id`.
    #[inline]
    pub fn msip<H: HartIdNumber>(&self, hart_id: H) -> MSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MSIP::new(self.as_ptr().add(hart_id.number()) as _) }
    }

    /// Returns the `MSIP` register for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the `mhartid` CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`MSWI::msip`] instead.
    #[inline]
    pub fn msip_mhartid(&self) -> MSIP {
        let hart_id = mhartid::read();
        // SAFETY: `hart_id` is valid for the target and is the current hart
        unsafe { MSIP::new(self.as_ptr().add(hart_id) as _) }
    }
}

unsafe_peripheral!(MSIP, u32, RW);

impl MSIP {
    /// Returns `true` if a machine software interrupt is pending.
    #[inline]
    pub fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    /// Writes to the register to trigger a machine software interrupt.
    #[inline]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a machine software interrupt.
    #[inline]
    pub fn unpend(self) {
        self.register.write(0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mswi() {
        // Variable to emulate the interrupt pending register
        let mut raw_reg = 0u32;
        // SAFETY: valid memory address
        let msip = unsafe { MSIP::new(&mut raw_reg as *mut u32 as usize) };

        assert!(!msip.is_pending());
        assert_eq!(raw_reg, 0);
        msip.pend();
        assert!(msip.is_pending());
        assert_ne!(raw_reg, 0);
        msip.unpend();
        assert!(!msip.is_pending());
        assert_eq!(raw_reg, 0);
    }
}
