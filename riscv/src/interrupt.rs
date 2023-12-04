//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs
use crate::register::mstatus;

/// Disables all interrupts in the current hart.
#[inline]
pub unsafe fn disable() {
    match () {
        #[cfg(riscv)]
        () => mstatus::clear_mie(),
        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Enables all the interrupts in the current hart.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[inline]
pub unsafe fn enable() {
    match () {
        #[cfg(riscv)]
        () => mstatus::set_mie(),
        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Execute closure `f` with interrupts disabled in the current hart.
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
    unsafe {
        disable();
    }

    let r = f();

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if mstatus.mie() {
        unsafe {
            enable();
        }
    }

    r
}
