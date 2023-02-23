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

impl<const BASE: usize> PLIC<BASE> {
    /// Returns the priority level number associated to a given interrupt source.
    #[inline]
    pub fn get_priority<I: InterruptNumber>(source: I) -> u16 {
        let source = usize::from(source.number());
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).priority[source].read() as _ }
    }

    /// Sets the priority level of a given interrupt source.
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

    /// Checks if an interrupt source is enabled for a given context.
    #[inline]
    pub fn is_enabled<C: ContextNumber, I: InterruptNumber>(context: C, source: I) -> bool {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        // SAFETY: atomic read with no side effects
        let flags = unsafe { (*Self::PTR).enables[context][source / MAX_FLAGS_WORDS].read() };
        (flags & mask) == mask
    }

    /// Enables an interrupt source for a given context.
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
    pub unsafe fn enable<C: ContextNumber, I: InterruptNumber>(&mut self, context: C, source: I) {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        self.enables[context][source / MAX_FLAGS_WORDS].modify(|value| value | mask);
    }

    /// Disables an interrupt source for a given context.
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
    pub unsafe fn disable<C: ContextNumber, I: InterruptNumber>(&mut self, context: C, source: I) {
        let context = usize::from(context.number());
        let source = usize::from(source.number());
        let mask: u32 = 1 << (source % MAX_FLAGS_WORDS);
        self.enables[context][source / MAX_FLAGS_WORDS].modify(|value| value & !mask);
    }

    /// Gets the priority threshold for a given context.
    #[inline]
    pub fn get_threshold<C: ContextNumber>(context: C) -> u32 {
        let context = usize::from(context.number());
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).states[context].threshold.read() }
    }

    /// Sets the priority threshold for a given context.
    ///
    /// # Safety
    ///
    /// Unmasking an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn set_threshold<C: ContextNumber, P: PriorityLevel>(context: C, priority: P) {
        let context = usize::from(context.number());
        let priority = u32::from(priority.number());
        // SAFETY: atomic write with no side effects
        (*Self::PTR).states[context].threshold.write(priority);
    }

    /// Claims the number of a pending interrupt for a given context.
    #[inline]
    pub fn claim<C: ContextNumber>(context: C) -> u16 {
        let context = usize::from(context.number());
        // SAFETY: atomic read with no side effects
        unsafe { (*Self::PTR).states[context].claim_complete.read() as _ }
    }

    /// Marks a pending interrupt as complete from a given context.
    #[inline]
    pub fn complete<C: ContextNumber, I: InterruptNumber>(context: C, source: I) {
        let context = usize::from(context.number());
        let source = u32::from(source.number());
        // SAFETY: atomic write with no side effects
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
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    fn number(self) -> u16;
}

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
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait PriorityLevel: Copy {
    fn number(self) -> u16;
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
