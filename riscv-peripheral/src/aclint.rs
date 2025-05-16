//! Devices for the Core Local Interruptor (CLINT) and Advanced CLINT (ACLINT) peripherals.
//!
//! CLINT pecification: <https://github.com/pulp-platform/clint>
//! ACLINT Specification: <https://github.com/riscvarchive/riscv-aclint/blob/main/riscv-aclint.adoc>

pub mod mswi;
pub mod mtimer;
pub mod sswi;

pub use riscv_pac::HartIdNumber; // re-export useful riscv-pac traits

/// Trait for a CLINT peripheral.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with a CLINT peripheral.
/// * The CLINT peripheral base address `BASE` must be valid for the target device.
/// * The CLINT peripheral clock frequency `MTIME_FREQ` must be valid for the target device.
pub unsafe trait Clint: Copy {
    /// Base address of the CLINT peripheral.
    const BASE: usize;
    /// Clock frequency of the CLINT's [`MTIME`](mtimer::MTIME) register.
    const MTIME_FREQ: usize;
}

/// Interface for a CLINT peripheral.
///
/// The RISC-V standard does not specify a fixed location for the CLINT.
/// Thus, each platform must specify the base address of the CLINT on the platform.
/// The base address and clock frequency are defined in the [`Clint`] trait.
///
/// The CLINT standard allows up to 4_095 different HARTs connected to the CLINT.
/// Each HART has an assigned index starting from 0 to up to 4_094.
/// In this way, each HART's timer and software interrupts can be independently configured.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLINT<C> {
    _marker: core::marker::PhantomData<C>,
}

impl<C: Clint> CLINT<C> {
    #[inline]
    /// Creates a new `CLINT` peripheral.
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the [`MSWI`](mswi::MSWI) device.
    #[inline]
    pub const fn mswi(self) -> mswi::MSWI<C> {
        mswi::MSWI::new()
    }

    /// Returns the [`MTIMER`](mtimer::MTIMER) device.
    #[inline]
    pub const fn mtimer(self) -> mtimer::MTIMER<C> {
        mtimer::MTIMER::new()
    }

    /// Returns `true` if a machine timer **OR** software interrupt is pending.
    #[inline]
    pub fn is_interrupting(self) -> bool {
        self.mswi().is_interrupting() || self.mtimer().is_interrupting()
    }

    /// Returns `true` if machine timer **OR** software interrupts are enabled.
    #[inline]
    pub fn is_enabled(self) -> bool {
        self.mswi().is_enabled() || self.mtimer().is_enabled()
    }

    /// Enables machine timer **AND** software interrupts to allow the CLINT to trigger interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `CLINT` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(self) {
        self.mswi().enable();
        self.mtimer().enable();
    }

    /// Disables machine timer **AND** software interrupts to prevent the CLINT from triggering interrupts.
    #[inline]
    pub fn disable(self) {
        self.mswi().disable();
        self.mtimer().disable();
    }
}

#[cfg(test)]
pub(crate) mod test {
    use riscv_pac::{result::Error, HartIdNumber};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[riscv::pac_enum(unsafe HartIdNumber)]
    pub(super) enum HartId {
        H0 = 0,
        H1 = 1,
        H2 = 2,
    }

    #[test]
    fn check_hart_id_enum() {
        assert_eq!(HartId::H0.number(), 0);
        assert_eq!(HartId::H1.number(), 1);
        assert_eq!(HartId::H2.number(), 2);

        assert_eq!(HartId::from_number(0), Ok(HartId::H0));
        assert_eq!(HartId::from_number(1), Ok(HartId::H1));
        assert_eq!(HartId::from_number(2), Ok(HartId::H2));

        assert_eq!(HartId::from_number(3), Err(Error::InvalidVariant(3)));
    }

    #[allow(dead_code)]
    #[test]
    fn check_clint() {
        // Call CLINT macro with a base address and a list of mtimecmps for easing access to per-HART mtimecmp regs.
        crate::clint_codegen!(
            base 0x0200_0000,
            mtime_freq 32_768,
            harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2],
        );

        let clint = CLINT::new();

        let mswi = clint.mswi();
        let mtimer = clint.mtimer();

        let msip0 = mswi.msip(HartId::H0);
        let msip1 = mswi.msip(HartId::H1);
        let msip2 = mswi.msip(HartId::H2);

        assert_eq!(msip0.get_ptr() as usize, 0x0200_0000);
        assert_eq!(msip1.get_ptr() as usize, 0x0200_0000 + 4); // 4 bytes per register
        assert_eq!(msip2.get_ptr() as usize, 0x0200_0000 + 2 * 4);

        let mtimecmp0 = mtimer.mtimecmp(HartId::H0);
        let mtimecmp1 = mtimer.mtimecmp(HartId::H1);
        let mtimecmp2 = mtimer.mtimecmp(HartId::H2);

        assert_eq!(mtimecmp0.get_ptr() as usize, 0x0200_4000);
        assert_eq!(mtimecmp1.get_ptr() as usize, 0x0200_4000 + 8); // 8 bytes per register
        assert_eq!(mtimecmp2.get_ptr() as usize, 0x0200_4000 + 2 * 8);

        let mtime = mtimer.mtime();
        assert_eq!(mtime.get_ptr() as usize, 0x0200_bff8);

        assert_eq!(clint.mtimecmp0(), mtimer.mtimecmp(HartId::H0));
        assert_eq!(clint.mtimecmp1(), mtimer.mtimecmp(HartId::H1));
        assert_eq!(clint.mtimecmp2(), mtimer.mtimecmp(HartId::H2));

        assert_eq!(clint.msip0(), mswi.msip(HartId::H0));
        assert_eq!(clint.msip1(), mswi.msip(HartId::H1));
        assert_eq!(clint.msip2(), mswi.msip(HartId::H2));
    }
}
