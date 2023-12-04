//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs

pub mod machine {
    use crate::register::mstatus;

    /// Disables all interrupts in the current hart (machine mode).
    #[inline]
    pub fn disable() {
        unsafe { mstatus::clear_mie() }
    }

    /// Enables all the interrupts in the current hart (machine mode).
    ///
    /// # Safety
    ///
    /// Do not call this function inside a critical section.
    #[inline]
    pub unsafe fn enable() {
        mstatus::set_mie()
    }

    /// Execute closure `f` with interrupts disabled in the current hart (machine mode).
    ///
    /// This method does not synchronise multiple harts, so it is not suitable for
    /// using as a critical section. See the `critical-section` crate for a cross-platform
    /// way to enter a critical section which provides a `CriticalSection` token.
    ///
    /// This crate provides an implementation for `critical-section` suitable for single-hart systems,
    /// based on disabling all interrupts. It can be enabled with the `critical-section-single-hart` feature.
    #[inline]
    pub fn free<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let mstatus = mstatus::read();

        // disable interrupts
        disable();

        let r = f();

        // If the interrupts were active before our `disable` call, then re-enable
        // them. Otherwise, keep them disabled
        if mstatus.mie() {
            unsafe { enable() };
        }

        r
    }
}
pub mod supervisor {
    use crate::register::sstatus;

    /// Disables all interrupts in the current hart (supervisor mode).
    #[inline]
    pub fn disable() {
        unsafe { sstatus::clear_sie() }
    }

    /// Enables all the interrupts in the current hart (supervisor mode).
    ///
    /// # Safety
    ///
    /// Do not call this function inside a critical section.
    #[inline]
    pub unsafe fn enable() {
        sstatus::set_sie()
    }

    /// Execute closure `f` with interrupts disabled in the current hart (supervisor mode).
    ///
    /// This method does not synchronise multiple harts, so it is not suitable for
    /// using as a critical section. See the `critical-section` crate for a cross-platform
    /// way to enter a critical section which provides a `CriticalSection` token.
    ///
    /// This crate provides an implementation for `critical-section` suitable for single-hart systems,
    /// based on disabling all interrupts. It can be enabled with the `critical-section-single-hart` feature.
    #[inline]
    pub fn free<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let sstatus = sstatus::read();

        // disable interrupts
        disable();

        let r = f();

        // If the interrupts were active before our `disable` call, then re-enable
        // them. Otherwise, keep them disabled
        if sstatus.sie() {
            unsafe { enable() };
        }

        r
    }
}

#[cfg(not(feature = "s-mode"))]
pub use machine::*;
#[cfg(feature = "s-mode")]
pub use supervisor::*;
