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
    use super::*;

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
}
