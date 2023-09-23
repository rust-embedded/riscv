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

    /// Returns `true` if a machine external interrupt is pending.
    #[inline]
    pub fn is_interrupting() -> bool {
        riscv::register::mip::read().mext()
    }

    /// Returns true if Machine External Interrupts are enabled.
    #[inline]
    pub fn is_enabled() -> bool {
        riscv::register::mie::read().mext()
    }

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
        // SAFETY: valid address
        unsafe { priorities::PRIORITIES::new(P::BASE + Self::PRIORITIES_OFFSET) }
    }

    /// Returns the pendings register of the PLIC.
    /// This register allows to check if a particular interrupt source is pending.
    #[inline]
    pub fn pendings() -> pendings::PENDINGS {
        // SAFETY: valid address
        unsafe { pendings::PENDINGS::new(P::BASE + Self::PENDINGS_OFFSET) }
    }

    /// Returns a proxy to access to all the PLIC registers of a given context.
    #[inline]
    pub fn ctx<C: ContextNumber>(context: C) -> CTX<P> {
        // SAFETY: valid context number
        unsafe { CTX::new(context.number()) }
    }
}

/// PLIC context proxy. It provides access to the PLIC registers of a given context.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CTX<P: Plic> {
    context: usize,
    _marker: core::marker::PhantomData<P>,
}

impl<P: Plic> CTX<P> {
    const ENABLES_OFFSET: usize = 0x2000;
    const ENABLES_SEPARATION: usize = 0x80;

    const THRESHOLDS_OFFSET: usize = 0x20_0000;
    const THRESHOLDS_SEPARATION: usize = 0x1000;

    const CLAIMS_OFFSET: usize = 0x20_0004;
    const CLAIMS_SEPARATION: usize = 0x1000;

    /// Creates a new PLIC context proxy
    ///
    /// # Safety
    ///
    /// The context number must be valid for the target device.
    #[inline]
    pub(crate) unsafe fn new(context: u16) -> Self {
        Self {
            context: context as _,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the context number of this proxy.
    #[inline]
    pub const fn context(self) -> u16 {
        self.context as _
    }

    /// Returns the interrupts enable register of the context.
    #[inline]
    pub const fn enables(self) -> enables::ENABLES {
        let addr = P::BASE + Self::ENABLES_OFFSET + self.context * Self::ENABLES_SEPARATION;
        // SAFETY: valid address
        unsafe { enables::ENABLES::new(addr) }
    }

    /// Returns the interrupt threshold register of the context.
    #[inline]
    pub const fn threshold(self) -> threshold::THRESHOLD {
        let addr = P::BASE + Self::THRESHOLDS_OFFSET + self.context * Self::THRESHOLDS_SEPARATION;
        // SAFETY: valid address
        unsafe { threshold::THRESHOLD::new(addr) }
    }

    /// Returns the interrupt claim/complete register of the context.
    #[inline]
    pub const fn claim(self) -> claim::CLAIM {
        let addr = P::BASE + Self::CLAIMS_OFFSET + self.context * Self::CLAIMS_SEPARATION;
        // SAFETY: valid address
        unsafe { claim::CLAIM::new(addr) }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{ContextNumber, InterruptNumber, PriorityNumber};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(crate) enum Interrupt {
        I1 = 1,
        I2 = 2,
        I3 = 3,
        I4 = 4,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u8)]
    pub(crate) enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(crate) enum Context {
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

    #[allow(dead_code)]
    #[test]
    fn check_plic() {
        crate::plic_codegen!(
            base 0x0C00_0000,
            ctxs [ctx0 = (Context::C0, "`C0`"), ctx1 = (Context::C1, "`C1`"), ctx2 = (Context::C2, "`C2`")],
        );

        let priorities = PLIC::priorities();
        let pendings = PLIC::pendings();

        assert_eq!(priorities.address(), 0x0C00_0000);
        assert_eq!(pendings.address(), 0x0C00_1000);

        for i in 0..=Context::MAX_CONTEXT_NUMBER {
            let context = Context::from_number(i).unwrap();
            let ctx = PLIC::ctx(context);

            assert_eq!(
                ctx.enables().address(),
                0x0C00_0000 + 0x2000 + i as usize * 0x80
            );
            assert_eq!(
                ctx.threshold().get_ptr() as usize,
                0x0C00_0000 + 0x20_0000 + i as usize * 0x1000
            );
            assert_eq!(
                ctx.claim().get_ptr() as usize,
                0x0C00_0000 + 0x20_0004 + i as usize * 0x1000
            );
        }

        let ctx0 = PLIC::ctx0();
        let ctx_0_ = PLIC::ctx(Context::C0);
        assert_eq!(ctx0.enables().address(), ctx_0_.enables().address());
        assert_eq!(ctx0.threshold().get_ptr(), ctx_0_.threshold().get_ptr());
        assert_eq!(ctx0.claim().get_ptr(), ctx_0_.claim().get_ptr());

        let ctx1 = PLIC::ctx1();
        let ctx_1_ = PLIC::ctx(Context::C1);
        assert_eq!(ctx1.enables().address(), ctx_1_.enables().address());
        assert_eq!(ctx1.threshold().get_ptr(), ctx_1_.threshold().get_ptr());
        assert_eq!(ctx1.claim().get_ptr(), ctx_1_.claim().get_ptr());

        let ctx2 = PLIC::ctx2();
        let ctx_2_ = PLIC::ctx(Context::C2);
        assert_eq!(ctx2.enables().address(), ctx_2_.enables().address());
        assert_eq!(ctx2.threshold().get_ptr(), ctx_2_.threshold().get_ptr());
        assert_eq!(ctx2.claim().get_ptr(), ctx_2_.claim().get_ptr());
    }
}
