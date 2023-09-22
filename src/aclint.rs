//! Devices for the Core Local Interruptor (CLINT) and Advanced CLINT (ACLINT) peripherals.
//!
//! CLINT pecification: <https://github.com/pulp-platform/clint>
//! ACLINT Specification: <https://chromitem-soc.readthedocs.io/en/latest/clint.html>

pub mod mswi;
pub mod mtimer;
pub mod sswi;

/// Trait for enums of HART IDs in (A)CLINT peripherals.
///
/// # Note
///
/// If your target only has one HART (HART ID 0), you don't need to implement this trait.
/// Instead, you can access directly to the base registers through the `(A)CLINT` structs.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with a PLIC peripheral.
/// * This trait must only be implemented on enums of HART IDs.
/// * Each enum variant must represent a distinct value (no duplicates are permitted).
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the HART ID numbers must be less than or equal to `MAX_HART_ID_NUMBER`.
/// * `MAX_HART_ID_NUMBER` must coincide with the highest allowed HART ID number.
pub unsafe trait HartIdNumber: Copy {
    /// Highest number assigned to a HART ID.
    const MAX_HART_ID_NUMBER: u16;

    /// Converts a HART Id to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid HART ID.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}

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

    /// Enables machine software interrupts to let the `MSWI` peripheral trigger interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `MSWI` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable_mswi() {
        mswi::MSWI::enable();
    }

    /// Disables machine software interrupts to prevent the `MSWI` peripheral from triggering interrupts.
    #[inline]
    pub fn disable_mswi() {
        mswi::MSWI::disable();
    }

    /// Enables machine timer interrupts to let the `MTIMER` peripheral trigger interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `MTIMER` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable_mtimer() {
        mtimer::MTIMER::enable();
    }

    /// Disables machine timer interrupts to prevent the `MTIMER` peripheral from triggering interrupts.
    #[inline]
    pub fn disable_mtimer() {
        mtimer::MTIMER::disable();
    }

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

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub(crate) enum HartId {
        H0 = 0,
        H1 = 1,
        H2 = 2,
    }

    unsafe impl HartIdNumber for HartId {
        const MAX_HART_ID_NUMBER: u16 = 2;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self, u16> {
            if number > Self::MAX_HART_ID_NUMBER {
                Err(number)
            } else {
                // SAFETY: valid context number
                Ok(unsafe { core::mem::transmute(number) })
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

        assert_eq!(HartId::from_number(3), Err(3));
    }

    #[allow(dead_code)]
    #[test]
    fn check_clint() {
        // Call CLINT macro with a base address and a list of mtimecmps for easing access to per-HART mtimecmp regs.
        crate::clint_codegen!(
            base 0x0200_0000,
            mtimecmps [mtimecmp0=(HartId::H0,"`H0`"), mtimecmp1=(HartId::H1,"`H1`"), mtimecmp2=(HartId::H2,"`H2`")],
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
        assert_eq!(mtimecmp1.get_ptr() as usize, 0x0200_4000 + 1 * 8); // 8 bytes per register
        assert_eq!(mtimecmp2.get_ptr() as usize, 0x0200_4000 + 2 * 8);

        // Check that the mtimecmpX functions are equivalent to the mtimer.mtimecmp(X) function.
        let mtimecmp0 = CLINT::mtimecmp0();
        let mtimecmp1 = CLINT::mtimecmp1();
        let mtimecmp2 = CLINT::mtimecmp2();

        assert_eq!(
            mtimecmp0.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H0).get_ptr() as usize
        );
        assert_eq!(
            mtimecmp1.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H1).get_ptr() as usize
        );
        assert_eq!(
            mtimecmp2.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H2).get_ptr() as usize
        );
    }
}
