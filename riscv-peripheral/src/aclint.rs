//! Devices for the Core Local Interruptor (CLINT) and Advanced CLINT (ACLINT) peripherals.
//!
//! CLINT pecification: <https://github.com/pulp-platform/clint>
//! ACLINT Specification: <https://chromitem-soc.readthedocs.io/en/latest/clint.html>

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
pub unsafe trait Clint: Copy {
    /// Base address of the CLINT peripheral.
    const BASE: usize;
}

/// Interface for a CLINT peripheral.
///
/// The RISC-V standard does not specify a fixed location for the CLINT.
/// Thus, each platform must specify the base address of the CLINT on the platform.
/// The base address, as well as all the associated types, are defined in the [`Clint`] trait.
///
/// The CLINT standard allows up to 4_095 different HARTs connected to the CLINT.
/// Each HART has an assigned index starting from 0 to up to 4_094.
/// In this way, each HART's timer and software interrupts can be independently configured.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLINT<C: Clint> {
    _marker: core::marker::PhantomData<C>,
}

impl<C: Clint> CLINT<C> {
    const MTIMECMP_OFFSET: usize = 0x4000;

    const MTIME_OFFSET: usize = 0xBFF8;

    /// Returns the `MSWI` peripheral.
    #[inline]
    pub const fn mswi() -> mswi::MSWI {
        // SAFETY: valid base address
        unsafe { mswi::MSWI::new(C::BASE) }
    }

    /// Returns the `MTIMER` peripheral.
    #[inline]
    pub const fn mtimer() -> mtimer::MTIMER {
        // SAFETY: valid base address
        unsafe {
            mtimer::MTIMER::new(
                C::BASE + Self::MTIMECMP_OFFSET,
                C::BASE + Self::MTIME_OFFSET,
            )
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::HartIdNumber;
    use riscv_pac::result::{Error, Result};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(usize)]
    pub(crate) enum HartId {
        H0 = 0,
        H1 = 1,
        H2 = 2,
    }

    unsafe impl HartIdNumber for HartId {
        const MAX_HART_ID_NUMBER: usize = Self::H2 as usize;

        #[inline]
        fn number(self) -> usize {
            self as _
        }

        #[inline]
        fn from_number(number: usize) -> Result<Self> {
            if number > Self::MAX_HART_ID_NUMBER {
                Err(Error::InvalidVariant(number))
            } else {
                // SAFETY: valid context number
                Ok(unsafe { core::mem::transmute::<usize, HartId>(number) })
            }
        }
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
            mtimecmps [mtimecmp0=(HartId::H0,"`H0`"), mtimecmp1=(HartId::H1,"`H1`"), mtimecmp2=(HartId::H2,"`H2`")],
            msips [msip0=(HartId::H0,"`H0`"), msip1=(HartId::H1,"`H1`"), msip2=(HartId::H2,"`H2`")],
        );

        let mswi = CLINT::mswi();
        let mtimer = CLINT::mtimer();

        assert_eq!(mswi.msip0.get_ptr() as usize, 0x0200_0000);
        assert_eq!(mtimer.mtimecmp0.get_ptr() as usize, 0x0200_4000);
        assert_eq!(mtimer.mtime.get_ptr() as usize, 0x0200_bff8);

        let mtimecmp0 = mtimer.mtimecmp(HartId::H0);
        let mtimecmp1 = mtimer.mtimecmp(HartId::H1);
        let mtimecmp2 = mtimer.mtimecmp(HartId::H2);

        assert_eq!(mtimecmp0.get_ptr() as usize, 0x0200_4000);
        assert_eq!(mtimecmp1.get_ptr() as usize, 0x0200_4000 + 8); // 8 bytes per register
        assert_eq!(mtimecmp2.get_ptr() as usize, 0x0200_4000 + 2 * 8);

        assert_eq!(CLINT::mtime(), mtimer.mtime);
        assert_eq!(CLINT::mtimecmp0(), mtimer.mtimecmp(HartId::H0));
        assert_eq!(CLINT::mtimecmp1(), mtimer.mtimecmp(HartId::H1));
        assert_eq!(CLINT::mtimecmp2(), mtimer.mtimecmp(HartId::H2));

        assert_eq!(CLINT::msip0(), mswi.msip(HartId::H0));
        assert_eq!(CLINT::msip1(), mswi.msip(HartId::H1));
        assert_eq!(CLINT::msip2(), mswi.msip(HartId::H2));
    }
}
