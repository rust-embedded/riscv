//! Supervisor-level Software Interrupt Device.

pub use super::HartIdNumber;
use crate::common::unsafe_peripheral;
use riscv::register::{sie, sip};

/// Trait for a Supervisor-level Software Interrupt device.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with an SSWI device.
/// * The SSWI device base address `BASE` must be valid for the target.
pub unsafe trait Sswi: Copy {
    /// Base address of the SSWI peripheral.
    const BASE: usize;
}

/// SSWI peripheral.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SSWI<M> {
    _marker: core::marker::PhantomData<M>,
}

impl<S: Sswi> SSWI<S> {
    /// Creates a new `SSWI` device.
    #[inline]
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the base address of the `SSWI` device.
    #[inline]
    const fn as_ptr(self) -> *const u32 {
        S::BASE as *const u32
    }

    /// Returns `true` if a supervisor software interrupt is pending.
    #[inline]
    pub fn is_interrupting(self) -> bool {
        sip::read().ssoft()
    }

    /// Returns `true` if supervisor software interrupts are enabled.
    #[inline]
    pub fn is_enabled(self) -> bool {
        sie::read().ssoft()
    }

    /// Enables supervisor software interrupts in the current HART.
    ///
    /// # Safety
    ///
    /// Enabling interrupts may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(self) {
        unsafe { sie::set_ssoft() };
    }

    /// Disables supervisor software interrupts in the current HART.
    #[inline]
    pub fn disable(self) {
        // SAFETY: it is safe to disable interrupts
        unsafe { sie::clear_ssoft() };
    }

    /// Returns the `SETSSIP` register for the HART which ID is `hart_id`.
    #[inline]
    pub fn setssip<H: HartIdNumber>(self, hart_id: H) -> SETSSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { SETSSIP::new(self.as_ptr().add(hart_id.number()) as _) }
    }

    /// Returns the `SETSSIP` register for HART 0.
    ///
    /// # Note
    ///
    /// According to the RISC-V specification, HART 0 is mandatory.
    /// Thus, this function is specially useful in single-HART mode, where HART 0 is the only HART available.
    /// In multi-HART mode, it is recommended to use [`SSWI::setssip`] instead.
    #[inline]
    pub const fn setssip0(self) -> SETSSIP {
        // SAFETY: HART 0 is mandatory
        unsafe { SETSSIP::new(S::BASE) }
    }
}

unsafe_peripheral!(SETSSIP, u32, RW);

impl SETSSIP {
    /// Returns `true` if a supervisor software interrupt is pending.
    #[inline]
    pub fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    /// Writes to the register to trigger a supervisor software interrupt.
    #[inline]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a supervisor software interrupt.
    #[inline]
    pub fn unpend(self) {
        self.register.write(0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sswi() {
        // Variable to emulate the interrupt pending register
        let mut raw_reg = 0u32;
        // SAFETY: valid memory address
        let setssip = unsafe { SETSSIP::new(&mut raw_reg as *mut u32 as usize) };

        assert!(!setssip.is_pending());
        assert_eq!(raw_reg, 0);
        setssip.pend();
        assert!(setssip.is_pending());
        assert_ne!(raw_reg, 0);
        setssip.unpend();
        assert!(!setssip.is_pending());
        assert_eq!(raw_reg, 0);
    }
}
