//! Advanced Core Local Interruptor (ACLINT) peripheral.
//!
//! Specification: <https://github.com/riscv/riscv-aclint/blob/main/riscv-aclint.adoc>

pub mod mswi;
pub mod mtimer;
pub mod sswi;

pub use mswi::MSWI;
pub use mtimer::MTIMER;
pub use sswi::SSWI;

/// Interface for a CLINT peripheral.
///
/// # Note
///
/// This structure requires the `clint` feature.
///
/// The RISC-V standard does not specify a fixed location for the CLINT.
/// Thus, each platform must specify the base address of the CLINT on the platform.
///
/// The CLINT standard allows up to 4_095 different HARTs connected to the CLINT.
/// Each HART has an assigned index starting from 0 to up to 4_094.
/// In this way, each HART's timer and software interrupts can be independently configured.
#[cfg(feature = "clint")]
pub struct CLINT {
    pub mswi: MSWI,
    pub mtimer: MTIMER,
}

#[cfg(feature = "clint")]
impl CLINT {
    pub const fn new(address: usize) -> Self {
        Self {
            mswi: MSWI::new(address),
            // address offsets: <https://github.com/pulp-platform/clint>
            mtimer: MTIMER::new(address + 0x4000, address + 0xBFF8),
        }
    }
}
