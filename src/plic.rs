//! Platform-Level Interrupt Controller (PLIC) peripheral.
//!
//! Specification: <https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

pub mod claim;
pub mod enables;
pub mod pendings;
pub mod priorities;
pub mod threshold;

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
/// * This trait must only be implemented on a PAC of a target with a PLIC peripheral.
/// * This trait must only be implemented on enums of external interrupts.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// * `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
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
/// * This trait must only be implemented on a PAC of a target with a PLIC peripheral.
/// * This trait must only be implemented on enums of priority levels.
/// * Each enum variant must represent a distinct value (no duplicates are permitted).
/// * Each enum variant must always return the same value (do not change at runtime).
/// * There must be a valid priority number set to 0 (i.e., never interrupt).
/// * All the priority level numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// * `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: u8;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u8;

    /// Tries to convert a number to a valid priority level.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u8) -> Result<Self, u8>;
}

/// Trait for enums of PLIC contexts.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available contexts for a specific device.
/// Each variant must convert to a `u16` of its context number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with a PLIC peripheral.
/// * This trait must only be implemented on enums of contexts.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each anum variant must always return the same value (do not change at runtime).
/// * All the context numbers must be less than or equal to `MAX_CONTEXT_NUMBER`.
/// * `MAX_CONTEXT_NUMBER` must coincide with the highest allowed context number.
pub unsafe trait ContextNumber: Copy {
    /// Highest number assigned to a context.
    const MAX_CONTEXT_NUMBER: u16;

    /// Converts an context to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid context.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}

/// Trait for a PLIC peripheral.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with a PLIC peripheral.
/// * The PLIC peripheral base address `BASE` must be valid for the target device.
pub unsafe trait Plic: Copy {
    /// Base address of the PLIC peripheral.
    const BASE: usize;
}

/// Platform-Level Interrupt Controler (PLIC) peripheral.
///
/// The RISC-V standard does not specify a fixed location for the PLIC.
/// Thus, each platform must specify the base address of the PLIC on the platform.
/// The base address, as well as all the associated types, are defined in the [`Plic`] trait.
///
/// The PLIC standard allows up to 15_872 different contexts for interfacing the PLIC.
/// Each context has an assigned index starting from 0 to up to 15_871.
/// Usually, each HART uses a dedicated context. In this way, they do not interfere
/// with each other when attending to external interruptions.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PLIC<P: Plic> {
    _marker: core::marker::PhantomData<P>,
}

impl<P: Plic> PLIC<P> {
    const PRIORITIES_OFFSET: usize = 0;

    const PENDINGS_OFFSET: usize = 0x1000;

    const ENABLES_OFFSET: usize = 0x2000;
    const ENABLES_SEPARATION: usize = 0x80;

    const THRESHOLDS_OFFSET: usize = 0x20_0000;
    const THRESHOLDS_SEPARATION: usize = 0x1000;

    const CLAIMS_OFFSET: usize = 0x20_0004;
    const CLAIMS_SEPARATION: usize = 0x1000;

    /// Sets the Machine External Interrupt bit of the `mie` CSR.
    /// This bit must be set for the PLIC to trigger machine external interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `PLIC` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable() {
        riscv::register::mie::set_mext();
    }

    /// Clears the Machine External Interrupt bit of the `mie` CSR.
    /// When cleared, the PLIC does not trigger machine external interrupts.
    #[inline]
    pub fn disable() {
        // SAFETY: it is safe to disable interrupts
        unsafe { riscv::register::mie::clear_mext() };
    }

    /// Returns the priorities register of the PLIC.
    /// This register allows to set the priority level of each interrupt source.
    /// The priority level of each interrupt source is shared among all the contexts.
    #[inline]
    pub fn priorities() -> priorities::PRIORITIES {
        unsafe { priorities::PRIORITIES::new(P::BASE + Self::PRIORITIES_OFFSET) }
    }

    /// Returns the pendings register of the PLIC.
    /// This register allows to check if an interrupt source is pending.
    /// This register is shared among all the contexts.
    #[inline]
    pub fn pendings() -> pendings::PENDINGS {
        unsafe { pendings::PENDINGS::new(P::BASE + Self::PENDINGS_OFFSET) }
    }

    /// Returns the interrupt enable register assigned to a given context.
    /// This register allows to enable/disable interrupt sources for a given context.
    /// Each context has its own enable register.
    #[inline]
    pub fn enables<C: ContextNumber>(context: C) -> enables::ENABLES {
        let context = context.number() as usize;
        let addr = P::BASE + Self::ENABLES_OFFSET + context * Self::ENABLES_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { enables::ENABLES::new(addr) }
    }

    /// Returns the interrupt threshold register assigned to a given context.
    /// This register allows to set the priority threshold level for a given context.
    /// Each context has its own threshold register.
    #[inline]
    pub fn threshold<C: ContextNumber>(context: C) -> threshold::THRESHOLD {
        let context = context.number() as usize;
        let addr = P::BASE + Self::THRESHOLDS_OFFSET + context * Self::THRESHOLDS_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { threshold::THRESHOLD::new(addr) }
    }

    /// Returns the interrupt claim/complete register assigned to a given context.
    /// This register allows to claim and complete interrupts for a given context.
    /// Each context has its own claim/complete register.
    #[inline]
    pub fn claim<C: ContextNumber>(context: C) -> claim::CLAIM {
        let context = context.number() as usize;
        let addr = P::BASE + Self::CLAIMS_OFFSET + context * Self::CLAIMS_SEPARATION;
        // SAFETY: context is a valid index
        unsafe { claim::CLAIM::new(addr) }
    }
}

#[cfg(test)]
pub(self) mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(super) enum Interrupt {
        I1 = 1,
        I2 = 2,
        I3 = 3,
        I4 = 4,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u8)]
    pub(super) enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(super) enum Context {
        C0 = 0,
        C1 = 1,
        C2 = 2,
    }

    unsafe impl InterruptNumber for Interrupt {
        const MAX_INTERRUPT_NUMBER: u16 = 4;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self, u16> {
            if number > Self::MAX_INTERRUPT_NUMBER || number == 0 {
                Err(number)
            } else {
                // SAFETY: valid interrupt number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    unsafe impl PriorityNumber for Priority {
        const MAX_PRIORITY_NUMBER: u8 = 3;

        #[inline]
        fn number(self) -> u8 {
            self as _
        }

        #[inline]
        fn from_number(number: u8) -> Result<Self, u8> {
            if number > Self::MAX_PRIORITY_NUMBER {
                Err(number)
            } else {
                // SAFETY: valid priority number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    unsafe impl ContextNumber for Context {
        const MAX_CONTEXT_NUMBER: u16 = 2;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self, u16> {
            if number > Self::MAX_CONTEXT_NUMBER {
                Err(number)
            } else {
                // SAFETY: valid context number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    #[test]
    fn check_interrupt_enum() {
        assert_eq!(Interrupt::I1.number(), 1);
        assert_eq!(Interrupt::I2.number(), 2);
        assert_eq!(Interrupt::I3.number(), 3);
        assert_eq!(Interrupt::I4.number(), 4);

        assert_eq!(Interrupt::from_number(1), Ok(Interrupt::I1));
        assert_eq!(Interrupt::from_number(2), Ok(Interrupt::I2));
        assert_eq!(Interrupt::from_number(3), Ok(Interrupt::I3));
        assert_eq!(Interrupt::from_number(4), Ok(Interrupt::I4));

        assert_eq!(Interrupt::from_number(0), Err(0));
        assert_eq!(Interrupt::from_number(5), Err(5));
    }

    #[test]
    fn check_priority_enum() {
        assert_eq!(Priority::P0.number(), 0);
        assert_eq!(Priority::P1.number(), 1);
        assert_eq!(Priority::P2.number(), 2);
        assert_eq!(Priority::P3.number(), 3);

        assert_eq!(Priority::from_number(0), Ok(Priority::P0));
        assert_eq!(Priority::from_number(1), Ok(Priority::P1));
        assert_eq!(Priority::from_number(2), Ok(Priority::P2));
        assert_eq!(Priority::from_number(3), Ok(Priority::P3));

        assert_eq!(Priority::from_number(4), Err(4));
    }

    #[test]
    fn check_context_enum() {
        assert_eq!(Context::C0.number(), 0);
        assert_eq!(Context::C1.number(), 1);
        assert_eq!(Context::C2.number(), 2);

        assert_eq!(Context::from_number(0), Ok(Context::C0));
        assert_eq!(Context::from_number(1), Ok(Context::C1));
        assert_eq!(Context::from_number(2), Ok(Context::C2));

        assert_eq!(Context::from_number(3), Err(3));
    }
}
