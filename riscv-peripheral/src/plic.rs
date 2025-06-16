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

use riscv::register::{mhartid, mie, mip};

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

    /// Creates a new `PLIC` peripheral.
    #[inline]
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns `true` if a machine external interrupt is pending.
    #[inline]
    pub fn is_interrupting(self) -> bool {
        mip::read().mext()
    }

    /// Returns true if machine external interrupts are enabled.
    #[inline]
    pub fn is_enabled(self) -> bool {
        mie::read().mext()
    }

    /// Enables machine external interrupts to allow the PLIC to trigger interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `PLIC` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(self) {
        unsafe { mie::set_mext() };
    }

    /// Disables machine external interrupts to prevent the PLIC from triggering interrupts.
    #[inline]
    pub fn disable(self) {
        // SAFETY: it is safe to disable interrupts
        unsafe { mie::clear_mext() };
    }

    /// Returns the [`PRIORITIES`](priorities::PRIORITIES) register of the PLIC.
    ///
    /// This register allows to set the priority level of each interrupt source.
    /// The priority level of each interrupt source is shared among all the contexts.
    #[inline]
    pub const fn priorities(self) -> priorities::PRIORITIES {
        // SAFETY: valid address
        unsafe { priorities::PRIORITIES::new(P::BASE + Self::PRIORITIES_OFFSET) }
    }

    /// Returns the [`PENDINGS`](pendings::PENDINGS) register of the PLIC.
    ///
    /// This register allows to check if a particular interrupt source is pending.
    #[inline]
    pub const fn pendings(self) -> pendings::PENDINGS {
        // SAFETY: valid address
        unsafe { pendings::PENDINGS::new(P::BASE + Self::PENDINGS_OFFSET) }
    }

    /// Returns a proxy to access to all the PLIC registers of a given HART context.
    #[inline]
    pub fn ctx<H: HartIdNumber>(self, hart_id: H) -> CTX<P> {
        // SAFETY: valid context number
        unsafe { CTX::new(hart_id.number() as _) }
    }

    /// Returns the PLIC HART context for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the [`mhartid`] CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`PLIC::ctx`] instead.
    #[inline]
    pub fn ctx_mhartid(self) -> CTX<P> {
        let hart_id = mhartid::read();
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
    use crate::test::HartId;
    use riscv_pac::HartIdNumber;

    #[allow(dead_code)]
    #[test]
    fn check_plic() {
        crate::plic_codegen!(
            PLIC,
            base 0x0C00_0000,
            harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2]
        );

        let plic = PLIC::new();
        let priorities = plic.priorities();
        let pendings = plic.pendings();

        assert_eq!(priorities.address(), 0x0C00_0000);
        assert_eq!(pendings.address(), 0x0C00_1000);

        for i in 0..=HartId::MAX_HART_ID_NUMBER {
            let hart_id = HartId::from_number(i).unwrap();

            let ctx = plic.ctx(hart_id);

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

        assert_eq!(plic.ctx0(), plic.ctx(HartId::H0));
        assert_eq!(plic.ctx1(), plic.ctx(HartId::H1));
        assert_eq!(plic.ctx2(), plic.ctx(HartId::H2));
    }
}
