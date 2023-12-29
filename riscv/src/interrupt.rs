//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs

pub mod machine {
    use crate::register::{mepc, mstatus};

    /// Disables all interrupts in the current hart (machine mode).
    #[inline]
    pub fn disable() {
        // SAFETY: It is safe to disable interrupts
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

    /// Execute closure `f` with interrupts enabled in the current hart (machine mode).
    ///
    /// This method is assumed to be called within an interrupt handler, and allows
    /// nested interrupts to occur. After the closure `f` is executed, the [`mstatus`]
    /// and [`mepc`] registers are properly restored to their previous values.
    ///
    /// # Safety
    ///
    /// - Do not call this function inside a critical section.
    /// - This method is assumed to be called within an interrupt handler.
    /// - Make sure to clear the interrupt flag that caused the interrupt before calling
    /// this method. Otherwise, the interrupt will be re-triggered before executing `f`.
    #[inline]
    pub unsafe fn nested<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let mstatus = mstatus::read();
        let mepc = mepc::read();

        // enable interrupts to allow nested interrupts
        enable();

        let r = f();

        // If the interrupts were inactive before our `enable` call, then re-disable
        // them. Otherwise, keep them enabled
        if !mstatus.mie() {
            disable();
        }

        // Restore MSTATUS.PIE, MSTATUS.MPP, and SEPC
        if mstatus.mpie() {
            mstatus::set_mpie();
        }
        mstatus::set_mpp(mstatus.mpp());
        mepc::write(mepc);

        r
    }
}
pub mod supervisor {
    use crate::register::{sepc, sstatus};

    /// Disables all interrupts in the current hart (supervisor mode).
    #[inline]
    pub fn disable() {
        // SAFETY: It is safe to disable interrupts
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

    /// Execute closure `f` with interrupts enabled in the current hart (supervisor mode).
    /// This method is assumed to be called within an interrupt handler, and allows
    /// nested interrupts to occur. After the closure `f` is executed, the [`sstatus`]
    /// and [`sepc`] registers are properly restored to their previous values.
    ///
    /// # Safety
    ///
    /// - Do not call this function inside a critical section.
    /// - This method is assumed to be called within an interrupt handler.
    /// - Make sure to clear the interrupt flag that caused the interrupt before calling
    /// this method. Otherwise, the interrupt will be re-triggered before executing `f`.
    #[inline]
    pub unsafe fn nested<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let sstatus = sstatus::read();
        let sepc = sepc::read();

        // enable interrupts to allow nested interrupts
        enable();

        let r = f();

        // If the interrupts were inactive before our `enable` call, then re-disable
        // them. Otherwise, keep them enabled
        if !sstatus.sie() {
            disable();
        }

        // Restore SSTATUS.SPIE, SSTATUS.SPP, and SEPC
        if sstatus.spie() {
            sstatus::set_spie();
        }
        sstatus::set_spp(sstatus.spp());
        sepc::write(sepc);

        r
    }
}

#[cfg(not(feature = "s-mode"))]
pub use machine::*;
#[cfg(feature = "s-mode")]
pub use supervisor::*;
