//! Devices for the Core Local Interruptor (CLINT) and Advanced CLINT (ACLINT) peripherals.
//!
//! CLINT pecification: <https://github.com/pulp-platform/clint>
//! ACLINT Specification: <https://chromitem-soc.readthedocs.io/en/latest/clint.html>

pub mod mswi;
pub mod mtimer;

#[cfg(feature = "clint")]
pub use super::CLINT;

#[cfg(feature = "clint")]
impl CLINT {
    pub const fn new(address: usize) -> Self {
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
