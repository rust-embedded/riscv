//! Devices for the Core Local Interruptor (CLINT) and Advanced CLINT (ACLINT) peripherals.
//!
//! CLINT pecification: <https://github.com/pulp-platform/clint>
//! ACLINT Specification: <https://chromitem-soc.readthedocs.io/en/latest/clint.html>

pub mod mswi;
pub mod mtimer;

#[cfg(feature = "clint")]
pub use super::CLINT;

/// Trait for enums of HART IDs in (A)CLINT peripherals.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available HARTs for a specific device.
/// Each variant must convert to a `u16` of its HART ID.
///
/// # Note
///
/// If your target only has one HART (HART ID 0), you don't need to implement this trait.
/// Instead, you can access directly to the base registers through the `(A)CLINT` structs.
///
/// # Safety
///
/// This trait must only be implemented on enums of HART IDs. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// All the HART ID numbers must be less than or equal to `MAX_HART_ID_NUMBER`.
/// `MAX_HART_ID_NUMBER` must coincide with the highest allowed HART ID number.
pub unsafe trait HartIdNumber: Copy {
    /// Highest number assigned to a HART ID.
    const MAX_HART_ID_NUMBER: u16;

    /// Converts a HART Id to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid HART ID.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u16) -> Result<Self, u16>;
}

#[cfg(feature = "clint")]
impl CLINT {
    /// Creates a new `CLINT` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid CLINT peripheral.
    pub unsafe fn new(address: usize) -> Self {
        Self {
            mswi: MSWI::new(address),
            mtimer: MTIMER::new(address + 0x4000, address + 0xBFF8),
        }
    }
}

/// Machine-level Software Interrupt Device.
///
/// # Note
///
/// You need to activate the `aclint` or `clint` features to use this device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MSWI {
    /// `MSIP` register for HART ID 0.  In multi-HART architectures,
    /// use [`MSWI::msip`] for accessing the `MSIP` of other HARTs.
    pub msip0: mswi::MSIP,
}

/// Machine-level Timer Device.
///
/// # Note
///
/// You need to activate the `aclint` or `clint` features to use this device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MTIMER {
    /// `MTIMECMP` register for HART ID 0.  In multi-HART architectures,
    /// use [`MTIMER::mtimecmp`] for accessing the `MTIMECMP` of other HARTs.
    pub mtimecmp0: mtimer::MTIMECMP,
    /// The `MTIME` register is shared among all the HARTs.
    pub mtime: mtimer::MTIME,
}

#[cfg(feature = "aclint")]
pub mod sswi;

/// Supervisor-level Software Interrupt Device.
///
/// # Note
///
/// You need to activate the `aclint` feature to use this device.
#[cfg(feature = "aclint")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SSWI {
    /// `SETSSIP` register for HART ID 0.  In multi-HART architectures,
    /// use [`SSWI::setssip`] for accessing the `SETSSIP` of other HARTs.
    pub setssip0: sswi::SETSSIP,
}
