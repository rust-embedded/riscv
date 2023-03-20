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

/// Trait for enums of interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a `u16` of its interrupt number.
///
/// # Note
///
/// Recall that the interrupt number `0` is reserved as "no interrupt".
///
/// # Safety
///
/// This trait must only be implemented on enums of external interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u16) -> Result<Self, u16>;
}

/// Trait for enums of interrupt priority numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available priority numbers for a specific device.
/// Each variant must convert to a `u8` of its priority level.
///
/// # Note
///
/// Recall that the priority number `0` is reserved as "never interrupt".
///
/// # Safety
///
/// This trait must only be implemented on enums of priority levels. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// There must be a valid priority number set to 0 (i.e., never interrupt).
/// All the priority level numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: u8;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u8;

    /// Tries to convert a number to a valid priority level.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u8) -> Result<Self, u8>;
}
