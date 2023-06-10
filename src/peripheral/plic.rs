//! Platform-Level Interrupt Controller (PLIC) peripheral.
//!
//! Specification: <https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

use crate::peripheral::common::{peripheral_reg, RW};
use crate::peripheral::PLIC;
use crate::register::mie;

pub mod enables;
pub mod pendings;
pub mod priorities;

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
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u16) -> Result<Self, u16>;
}

/// Trait for enums of priority levels.
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
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_CONTEXT_NUMBER: u8;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u8;

    /// Tries to convert a number to a valid priority level.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u8) -> Result<Self, u8>;
}

/// Trait for enums of PLIC contexts.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available contexts for a specific device.
/// Each variant must convert to a `u16` of its context number.
///
/// # Note
///
/// If your target only has one context (context 0), you don't need to implement this trait.
/// Instead, you can access directly to the PLIC registers of the [`PLIC`] struct.
///
/// # Safety
///
/// This trait must only be implemented on enums of contexts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// All the context numbers must be less than or equal to `MAX_CONTEXT_NUMBER`.
/// `MAX_CONTEXT_NUMBER` must coincide with the highest allowed context number.
pub unsafe trait ContextNumber: Copy {
    /// Highest number assigned to a context.
    const MAX_CONTEXT_NUMBER: u16;

    /// Converts an context to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a validcontext.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u16) -> Result<Self, u16>;
}

impl PLIC {
    /// Maximum number of interrupt sources supported by the PLIC standard.
    const MAX_SOURCES: u32 = 1_024;
    /// Maximum number of words needed to represent interrupts with flags.
    const MAX_FLAGS_WORDS: u32 = Self::MAX_SOURCES / u32::BITS;
    /// Separation between enables registers for different contexts.
    const ENABLES_SEPARATION: usize = 0x80;
    /// Separation between threshold registers for different contexts.
    const THRESHOLDS_SEPARATION: usize = 0x1000;
    /// Separation between claim/complete registers for different contexts.
    const CLAIMS_SEPARATION: usize = 0x1000;

    /// Creates a new instance of the PLIC peripheral.
    ///
    /// # Safety
    ///
    /// The base address must be valid for the target device.
    #[inline(always)]
    pub unsafe fn new(base: usize) -> Self {
        Self {
            priorities: PRIORITIES::new(base),
            pendings: PENDINGS::new(base + 0x1000),
            enables0: ENABLES::new(base + 0x2000),
            threshold0: THRESHOLD::new(base + 0x20_0000),
            claim0: CLAIM::new(base + 0x20_0004),
        }
    }

    /// Sets the Machine External Interrupt bit of the [`crate::register::mie`] CSR.
    /// This bit must be set for the PLIC to trigger machine external interrupts.
    #[inline(always)]
    pub unsafe fn enable() {
        mie::set_mext();
    }

    /// Clears the Machine External Interrupt bit of the [`crate::register::mie`] CSR.
    /// When cleared, the PLIC does not trigger machine external interrupts.
    #[inline(always)]
    pub unsafe fn disable() {
        mie::clear_mext();
    }

    /// Returns the interrupt enable register assigned to a given context.
    ///
    /// # Note
    ///
    /// For context 0, you can simply use [`PLIC::enables0`].
    #[inline(always)]
    pub fn enables<C: ContextNumber>(&self, context: C) -> ENABLES {
        let context = context.number() as usize;
        let addr = self.enables0.base.get_ptr() as usize + context * Self::ENABLES_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { ENABLES::new(addr) }
    }

    /// Returns the interrupt threshold register assigned to a given context.
    ///
    /// # Note
    ///
    /// For context 0, you can simply use [`PLIC::threshold0`].
    #[inline(always)]
    pub fn threshold<C: ContextNumber>(&self, context: C) -> THRESHOLD {
        let context = context.number() as usize;
        let addr = self.threshold0.get_ptr() as usize + context * Self::THRESHOLDS_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { THRESHOLD::new(addr) }
    }

    /// Returns the interrupt claim/complete register assigned to a given context.
    ///
    /// # Note
    ///
    /// For context 0, you can simply use [`PLIC::claim0`].
    #[inline(always)]
    pub unsafe fn claim<C: ContextNumber>(&self, context: C) -> CLAIM {
        let context = context.number() as usize;
        let addr = self.claim0.get_ptr() as usize + context * Self::CLAIMS_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { CLAIM::new(addr) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PRIORITIES {
    priority0: priorities::PRIORITY,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PENDINGS {
    base: pendings::PENDING,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ENABLES {
    base: enables::ENABLE,
}

peripheral_reg!(THRESHOLD, u32, RW);

impl THRESHOLD {
    /// Returns the priority threshold level.
    #[inline(always)]
    pub fn get_threshold<P: PriorityNumber>(&self) -> P {
        P::try_from(self.read() as _).unwrap()
    }

    /// Sets the priority threshold level.
    ///
    /// # Safety
    ///
    /// Changing the priority threshold can break priority-based critical sections.
    #[inline(always)]
    pub unsafe fn set_threshold<P: PriorityNumber>(&mut self, threshold: P) {
        self.write(threshold.number() as _)
    }
}

peripheral_reg!(CLAIM, u32, RW);

impl CLAIM {
    /// Claims the number of a pending interrupt for for the PLIC context.
    /// If no interrupt is pending for this context, it returns [`None`].
    #[inline(always)]
    pub fn claim<I: InterruptNumber>(self) -> Option<I> {
        match self.read() {
            0 => None,
            i => Some(I::try_from(i as _).unwrap()),
        }
    }

    /// Marks a pending interrupt as complete from for the PLIC context.
    ///
    /// # Note
    ///
    /// If the source ID does not match an interrupt source that is
    /// currently enabled for the target, the completion is silently ignored.
    #[inline(always)]
    pub fn complete<I: InterruptNumber>(self, source: I) {
        self.write(source.number() as _)
    }
}
