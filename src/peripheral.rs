//! RISC-V peripherals

pub mod common;

#[cfg(any(feature = "aclint", feature = "clint"))]
pub mod aclint;

/// Interface for a CLINT peripheral.
///
/// # Note
///
/// You need to set the `clint` feature to enable this peripheral.
///
/// The RISC-V standard does not specify a fixed location for the CLINT.
/// Thus, each platform must specify the base address of the CLINT on the platform.
///
/// The CLINT standard allows up to 4_095 different HARTs connected to the CLINT.
/// Each HART has an assigned index starting from 0 to up to 4_094.
/// In this way, each HART's timer and software interrupts can be independently configured.
#[allow(clippy::upper_case_acronyms)]
#[cfg(feature = "clint")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLINT {
    pub mswi: aclint::MSWI,
    pub mtimer: aclint::MTIMER,
}

// Platform-Level Interrupt Controller
#[cfg(feature = "plic")]
pub mod plic;

/// Interface for a context of the PLIC peripheral.
///
/// # Note
///
/// This structure requires the `plic` feature.
///
/// The RISC-V standard does not specify a fixed location for the PLIC.
/// Thus, each platform must specify the base address of the PLIC on the platform.
///
/// The PLIC standard allows up to 15_872 different contexts for interfacing the PLIC.
/// Each context has an assigned index starting from 0 to up to 15_871.
/// Usually, each HART uses a dedicated context. In this way, they do not interfere
/// with each other when attending to external interruptions.
#[allow(clippy::upper_case_acronyms)]
#[cfg(feature = "plic")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PLIC {
    /// Interrupts priorities register.
    pub priorities: plic::PRIORITIES,
    /// Interrupts pending register.
    pub pendings: plic::PENDINGS,
    /// Interrupt enables register for PLIC context 0.
    /// To access the enables register of other contexts, use [`PLIC::enables`].
    pub enables0: plic::ENABLES,
    /// Priority threshold register for PLIC context 0.
    /// To access the threshold register of other contexts, use [`PLIC::threshold`].
    pub threshold0: plic::THRESHOLD,
    /// Interrupt claim/complete register for PLIC context 0.
    /// To access the claim/complete register of other contexts, use [`PLIC::claim`].
    pub claim0: plic::CLAIM,
}
