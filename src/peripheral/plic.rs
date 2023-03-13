//! Platform-Level Interrupt Controller (PLIC) peripheral.
//!
//! Specification: https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc

use super::PLIC;
use volatile_register::{RO, RW};

/// Maximum number of interrupt sources supported by the PLIC standard.
const MAX_SOURCES: usize = 1_024;
/// Maximum number of words needed to represent interrupts with flags.
const MAX_FLAGS_WORDS: usize = MAX_SOURCES / (u32::BITS as usize);
/// Maximum number of contexts supported by the PLIC standard.
const MAX_CONTEXTS: usize = 15_872;

/// Register block.
#[repr(C)]
pub struct RegisterBlock {
    /// `0x0000_0000..=0x0000_0FFC` - Interrupt Priority Register.
    pub priority: [RW<u32>; MAX_SOURCES],
    /// `0x0000_1000..=0x0000_107C` - Interrupt Pending Register.
    pub pending: [RO<u32>; MAX_FLAGS_WORDS],
    /// `0x0000_1080..=0x0000_1FFC` - Reserved.
    _reserved1: [u32; 0x03e0],
    /// `0x0000_2000..=0x001F_1FFC` - Enable Registers (one per context).
    pub enables: [ContextEnable; MAX_CONTEXTS],
    /// `0x001F_2000..=0x001F_FFFF` - Reserved.
    _reserved2: [u32; 0x3800],
    /// `0x0020_0000..=0x03FF_FFFC` - State Registers (one per context).
    pub states: [ContextState; MAX_CONTEXTS],
}

/// Interrupt enable for a given context.
pub type ContextEnable = [RW<u32>; MAX_FLAGS_WORDS];

/// State of a single context.
#[repr(C)]
pub struct ContextState {
    /// `0x0000_0000` - Priority Threshold Register.
    pub threshold: RW<u32>,
    /// `0x0000_0004` - Claim/Complete Register.
    pub claim_complete: RW<u32>,
    /// `0x0000_0008..=0x0000_0FFC` - Reserved.
    _reserved: [u32; 0x3fe],
}

impl<const BASE: usize, const CONTEXT: usize> PLIC<BASE, CONTEXT> {
    /// Returns the priority level number associated to a given interrupt source.
    #[inline]
    pub fn get_priority<I: InterruptNumber>(source: I) -> u16 {
        let source = usize::from(source.number());
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).priority[source].read() as _ }
    }

    /// Sets the priority level of a given interrupt source.
    ///
    /// # Note
    ///
    /// Interrupt source priorities are shared among all the contexts of the PLIC.
    /// Thus, changing the priority of sources  may affect other PLIC contexts.
    ///
    /// # Safety
    ///
    /// Changing priority levels can break priority-based critical sections and compromise memory safety.
    #[inline]
    pub unsafe fn set_priority<I: InterruptNumber, P: PriorityLevel>(source: I, priority: P) {
        let source = usize::from(source.number());
        let priority = u32::from(priority.number());
        // SAFETY: atomic write with no side effects
        (*Self::PTR).priority[source].write(priority);
    }

    /// Checks if an interrupt triggered by a given source is pending.
    #[inline]
    pub fn is_pending<I: InterruptNumber>(source: I) -> bool {
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // SAFETY: atomic read with no side effects
        let flags = unsafe { (*Self::PTR).pending[source / MAX_FLAGS_WORDS].read() };
        (flags & mask) == mask
    }

    /// Checks if an interrupt source is enabled for the PLIC context.
    #[inline]
    pub fn is_enabled<I: InterruptNumber>(source: I) -> bool {
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // SAFETY: atomic read with no side effects
        let flags = unsafe { (*Self::PTR).enables[CONTEXT][source / MAX_FLAGS_WORDS].read() };
        (flags & mask) == mask
    }

    /// Enables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// This method performs a read-modify-write operation.
    /// That is why it is a method instead of an associated function.
    ///
    /// # Safety
    ///
    /// Non-atomic operations may lead to undefined behavior.
    /// Enabling an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn enable<I: InterruptNumber>(&mut self, source: I) {
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        self.enables[CONTEXT][source / MAX_FLAGS_WORDS].modify(|value| value | mask);
    }

    /// Disables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// This method performs a read-modify-write operation.
    /// That is why it is a method instead of an associated function.
    ///
    /// # Safety
    ///
    /// Non-atomic operations may lead to undefined behavior.
    #[inline]
    pub unsafe fn disable<I: InterruptNumber>(&mut self, source: I) {
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        self.enables[CONTEXT][source / MAX_FLAGS_WORDS].modify(|value| value & !mask);
    }

    /// Gets the priority threshold for for the PLIC context.
    #[inline]
    pub fn get_threshold() -> u32 {
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).states[CONTEXT].threshold.read() }
    }

    /// Sets the priority threshold for for the PLIC context.
    ///
    /// # Safety
    ///
    /// Unmasking an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn set_threshold<P: PriorityLevel>(priority: P) {
        let priority = u32::from(priority.number());
        // SAFETY: atomic write with no side effects
        (*Self::PTR).states[CONTEXT].threshold.write(priority);
    }

    /// Claims the number of a pending interrupt for for the PLIC context.
    #[inline]
    pub fn claim() -> u16 {
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).states[CONTEXT].claim_complete.read() as _ }
    }

    /// Marks a pending interrupt as complete from for the PLIC context.
    #[inline]
    pub fn complete<I: InterruptNumber>(source: I) {
        let source = u32::from(source.number());
        // SAFETY: atomic write with no side effects
        unsafe {
            (*Self::PTR).states[CONTEXT].claim_complete.write(source);
        }
    }
}

/// Helper structure to identify invalid interrupt numbers.
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());

/// Trait for enums of global interrupt numbers handled by the PLIC.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available PLIC global interrupts for a specific device.
/// Each variant must convert to a `u16` of its interrupt number.
///
/// # Note
///
/// Recall that the interrupt number `0` is reserved as "no interrupt".
///
/// # Safety
///
/// This trait must only be implemented on enums of PLIC global interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// The interrupt number must be less than 1_024.
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    fn try_from(value: u16) -> Result<Self, TryFromInterruptError>;
}

/// Helper structure to identify invalid priority numbers.
#[derive(Debug, Copy, Clone)]
pub struct TryFromPriorityError(());

/// Trait for enums of priority levels implemented by the PLIC.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available PLIC priority levels for a specific device.
/// Each variant must convert to a `u16` of its priority level.
///
/// # Note
///
/// Recall that the priority number `0` is reserved as "never interrupt".
///
/// # Safety
///
/// This trait must only be implemented on enums of PLIC priority level. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// All the interrupt numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait PriorityLevel: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: u16;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid priority level.
    fn try_from(value: u16) -> Result<Self, TryFromInterruptError>;
}
