//! Platform-Level Interrupt Controller (PLIC) peripheral.
//!
//! Specification: https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc

use super::PLIC;
use volatile_register::{RO, RW};

/// Maximum number of interrupt sources supported by the PLIC standard
const MAX_SOURCES: usize = 1_024;
/// Maximum number of words needed to represent interrupts with flags
const MAX_FLAGS_WORDS: usize = MAX_SOURCES / (u32::BITS as usize);
/// Maximum number of contexts supported by the PLIC standard
const MAX_CONTEXTS: usize = 15_872;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    pub priority: [RW<u32>; MAX_SOURCES],    // Offset: 0x0000_0000
    pub pending: [RO<u32>; MAX_FLAGS_WORDS], // Offset: 0x0000_1000
    _reserved1: [u32; 0x03e0],               // Offset: 0x0000_1080
    pub enables: [ContextEnable; MAX_CONTEXTS], // Offset: 0x0000_2000
    _reserved2: [u32; 0x3800],               // Offset: 0x001F_2000
    pub states: [ContextState; MAX_CONTEXTS], // Offset: 0x0020_0000; Total size: 0x0400_0000
}

/// Interrupt enable for a given context.
pub type ContextEnable = [RW<u32>; MAX_SOURCES / MAX_FLAGS_WORDS]; // Total size: 0x0000_0080

/// State of a single context
#[repr(C)]
pub struct ContextState {
    pub threshold: RW<u32>,      // Offset: 0x0000_0000
    pub claim_complete: RW<u32>, // Offset: 0x0000_0004
    _reserved: [u32; 0x3fe],     // Offset: 0x0000_0008; Total size: 0x0000_1000
}

impl<const BASE: usize> PLIC<BASE> {
    /// Returns the priority level number associated to a given interrupt source.
    #[inline]
    pub fn get_priority<I: InterruptNumber>(source: I) -> u32 {
        let source = usize::from(source.number());
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::PTR).priority[source].read() }
    }

    /// Sets the priority level of a given interrupt source.
    #[inline]
    pub fn set_priority<I: InterruptNumber, P: PriorityLevel>(source: I, priority: P) {
        let source = usize::from(source.number());
        // NOTE(unsafe) atomic write with no side effects
        unsafe {
            (*Self::PTR).priority[source].write(priority.number());
        }
    }

    /// Checks if an interrupt triggered by a given source is pending.
    #[inline]
    pub fn is_pending<I: InterruptNumber>(source: I) -> bool {
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // NOTE(unsafe) atomic read with no side effects
        let flags = unsafe { (*Self::PTR).pending[source / MAX_FLAGS_WORDS].read() };
        (flags & mask) == mask
    }

    /// Checks if an interrupt source is enabled for a given context.
    #[inline]
    pub fn is_enabled<C: ContextNumber, I: InterruptNumber>(context: C, source: I) -> bool {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // NOTE(unsafe) atomic read with no side effects
        let flags = unsafe { (*Self::PTR).enables[context][source / MAX_FLAGS_WORDS].read() };
        (flags & mask) == mask
    }

    /// Enables an interrupt source for a given context.
    #[inline]
    pub fn enable<C: ContextNumber, I: InterruptNumber>(context: C, source: I) {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // NOTE(unsafe) atomic write with no side effects
        unsafe {
            (*Self::PTR).enables[context][source / MAX_FLAGS_WORDS].modify(|value| value | mask);
        }
    }

    /// Disables an interrupt source for a given context.
    #[inline]
    pub fn disable<C: ContextNumber, I: InterruptNumber>(context: C, source: I) {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // NOTE(unsafe) atomic write with no side effects
        unsafe {
            (*Self::PTR).enables[context][source / MAX_FLAGS_WORDS].modify(|value| value & !mask);
        }
    }

    /// Gets the priority threshold for a given context.
    #[inline]
    pub fn get_threshold<C: ContextNumber>(context: C) -> u32 {
        let context = usize::from(context.number());
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::PTR).states[context].threshold.read() }
    }

    /// Sets the priority threshold for a given context.
    #[inline]
    pub fn set_threshold<C: ContextNumber, P: PriorityLevel>(context: C, priority: P) {
        let context = usize::from(context.number());
        let priority = priority.number();
        // NOTE(unsafe) atomic write with no side effects
        unsafe { (*Self::PTR).states[context].threshold.write(priority) }
    }

    /// Claims the number of a pending interrupt for a given context.
    #[inline]
    pub fn claim<C: ContextNumber>(context: C) -> u16 {
        let context = usize::from(context.number());
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::PTR).states[context].claim_complete.read() as _ }
    }

    /// Marks a pending interrupt as complete from a given context.
    #[inline]
    pub fn complete<C: ContextNumber, I: InterruptNumber>(context: C, source: I) {
        let context = usize::from(context.number());
        let source = u32::from(source.number());
        // NOTE(unsafe) atomic write with no side effects
        unsafe {
            (*Self::PTR).states[context].claim_complete.write(source);
        }
    }
}

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
/// Interrupt number 0 is reserved for "no interrupt"
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    fn number(self) -> u16;
}

/// Trait for enums of priority levels implemented by the PLIC.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available PLIC priority levels for a specific device.
/// Each variant must convert to a `u32` of its priority level.
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
/// Priority number 0 is reserved for "no priority (i.e., disabled)".
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait PriorityLevel: Copy {
    fn number(self) -> u32;
}

/// Trait for enums of contexts implemented by the PLIC.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available PLIC contexs for a specific device.
/// Each variant must convert to a `u16` of its context.
///
/// # Safety
///
/// This trait must only be implemented on enums of PLIC contexts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// The context number must be less than 15_872.
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait ContextNumber: Copy {
    fn number(self) -> u16;
}
