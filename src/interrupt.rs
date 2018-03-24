//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs
pub use bare_metal::{CriticalSection, Mutex, Nr};

/// Disables all interrupts
#[inline]
pub fn disable() {
    match () {
        #[cfg(target_arch = "riscv")]
        () => ::csr::mstatus.clear(|w| w.mie()),
        #[cfg(not(target_arch = "riscv"))]
        () => {}
    }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    match () {
        #[cfg(target_arch = "riscv")]
        () => ::csr::mstatus.set(|w| w.mie()),
        #[cfg(not(target_arch = "riscv"))]
        () => {}
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let mstatus = ::csr::mstatus.read();

    // disable interrupts
    disable();

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if mstatus.mie() {
        unsafe { enable() }
    }

    r
}
