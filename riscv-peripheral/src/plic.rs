//! Platform-Level Interrupt Controller (PLIC) peripheral.
//!
//! Specification: <https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

pub mod claim;
pub mod enables;
pub mod pendings;
pub mod priorities;
pub mod threshold;

// re-export useful riscv-pac traits
pub use riscv_pac::{HartIdNumber, InterruptNumber, PriorityNumber};

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

    /// Returns a proxy to access to all the PLIC registers of a given HART context.
    #[inline]
    pub fn ctx<H: HartIdNumber>(hart_id: H) -> CTX<P> {
        // SAFETY: valid context number
        unsafe { CTX::new(hart_id.number()) }
    }

    /// Returns the PLIC HART context for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`PLIC::ctx`] instead.
    #[inline]
    pub fn ctx_mhartid() -> CTX<P> {
        let hart_id = riscv::register::mhartid::read();
        // SAFETY: `hart_id` is valid for the target and is the current hart
        unsafe { CTX::new(hart_id as _) }
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
    use riscv_pac::result::{Error, Result};
    use riscv_pac::{ExternalInterruptNumber, HartIdNumber, InterruptNumber, PriorityNumber};

    #[pac_enum(unsafe InterruptNumber)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(crate) enum Interrupt {
        I1 = 1,
        I2 = 2,
        I3 = 3,
        I4 = 4,
    }

    unsafe impl ExternalInterruptNumber for Interrupt {}

    #[pac_enum(unsafe PriorityNumber)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u8)]
    pub(crate) enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[pac_enum(unsafe HartIdNumber)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(crate) enum Context {
        C0 = 0,
        C1 = 1,
        C2 = 2,
    }

    // unsafe impl InterruptNumber for Interrupt {
    //     const MAX_INTERRUPT_NUMBER: u16 = 4;

    //     #[inline]
    //     fn number(self) -> u16 {
    //         self as _
    //     }

        #[inline]
        fn from_number(number: u16) -> Result<Self> {
            if number > Self::MAX_INTERRUPT_NUMBER || number == 0 {
                Err(Error::InvalidVariant(number as usize))
            } else {
                // SAFETY: valid interrupt number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    unsafe impl ExternalInterruptNumber for Interrupt {}

    // unsafe impl PriorityNumber for Priority {
    //     const MAX_PRIORITY_NUMBER: u8 = 3;

    //     #[inline]
    //     fn number(self) -> u8 {
    //         self as _
    //     }

        #[inline]
        fn from_number(number: u8) -> Result<Self> {
            if number > Self::MAX_PRIORITY_NUMBER {
                Err(Error::InvalidVariant(number as usize))
            } else {
                // SAFETY: valid priority number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    // unsafe impl HartIdNumber for Context {
    //     const MAX_HART_ID_NUMBER: u16 = 2;

    //     #[inline]
    //     fn number(self) -> u16 {
    //         self as _
    //     }

        #[inline]
        fn from_number(number: u16) -> Result<Self> {
            if number > Self::MAX_HART_ID_NUMBER {
                Err(Error::InvalidVariant(number as usize))
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

        assert_eq!(Interrupt::from_number(0), Err(Error::InvalidVariant(0)),);
        assert_eq!(Interrupt::from_number(5), Err(Error::InvalidVariant(5)),);
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

        assert_eq!(Priority::from_number(4), Err(Error::InvalidVariant(4)),);
    }

    #[test]
    fn check_context_enum() {
        assert_eq!(Context::C0.number(), 0);
        assert_eq!(Context::C1.number(), 1);
        assert_eq!(Context::C2.number(), 2);

        assert_eq!(Context::from_number(0), Ok(Context::C0));
        assert_eq!(Context::from_number(1), Ok(Context::C1));
        assert_eq!(Context::from_number(2), Ok(Context::C2));

        assert_eq!(Context::from_number(3), Err(Error::InvalidVariant(3)),);
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

        for i in 0..=Context::MAX_HART_ID_NUMBER {
            let context = Context::from_number(i).unwrap();
            let i = i as usize;

            let ctx = PLIC::ctx(context);

            assert_eq!(ctx.enables().address(), 0x0C00_0000 + 0x2000 + i * 0x80);
            assert_eq!(
                ctx.threshold().get_ptr() as usize,
                0x0C00_0000 + 0x20_0000 + i * 0x1000
            );
            assert_eq!(
                ctx.claim().get_ptr() as usize,
                0x0C00_0000 + 0x20_0004 + i * 0x1000
            );
        }

        assert_eq!(PLIC::ctx0(), PLIC::ctx(Context::C0));
        assert_eq!(PLIC::ctx1(), PLIC::ctx(Context::C1));
        assert_eq!(PLIC::ctx2(), PLIC::ctx(Context::C2));
    }
}
