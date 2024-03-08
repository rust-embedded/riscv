//! Core-Local Interrupt Controller (CLIC) peripheral.
//!
//! Specification: <https://github.com/riscv/riscv-fast-interrupt/blob/master/clic.adoc>

pub mod intattr;
pub mod intctl;
pub mod intie;
pub mod intip;
pub mod inttrig;
#[cfg(feature = "clic-smclic")]
pub mod smclicconfig;

pub use riscv_pac::{HartIdNumber, InterruptNumber, PriorityNumber}; // re-export useful riscv-pac traits

/// Trait for a CLIC peripheral.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with a CLIC peripheral.
/// * The CLIC peripheral base address `BASE` must be valid for the target device.
pub unsafe trait Clic: Copy {
    /// Base address of the CLIC peripheral.
    const BASE: usize;
}

/// Core-Local Interrupt Controller (CLIC) peripheral.
///
/// The RISC-V standard does not specify a fixed location for the CLIC.
/// Thus, each platform must specify the base address of the CLIC on the platform.
/// The base address, as well as all the associated types, are defined in the [`Clic`] trait.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIC<P: Clic> {
    _marker: core::marker::PhantomData<P>,
}

impl<P: Clic> CLIC<P> {
    #[cfg(feature = "clic-smclic")]
    const SMCLICCONFIG_OFFSET: usize = 0x0;

    const INTTRIG_OFFSET: usize = 0x40;
    const INTTRIG_SEPARATION: usize = 0x4;

    const INT_OFFSET: usize = 0x1000;
    const INT_SEPARATION: usize = 0x4;

    /// Returns the smclicconfig register of the CLIC.
    #[inline]
    #[cfg(feature = "clic-smclic")]
    pub fn smclicconfig() -> smclicconfig::SMCLICCONFIG {
        // SAFETY: valid address
        unsafe { smclicconfig::SMCLICCONFIG::new(P::BASE + Self::SMCLICCONFIG_OFFSET) }
    }

    /// Returns the clicinttrig register for a given interrupt number.
    #[inline]
    pub fn inttrig<I: InterruptNumber>(int_nr: I) -> inttrig::INTTRIG {
        let addr =
            P::BASE + Self::INTTRIG_OFFSET + int_nr.number() as usize * Self::INTTRIG_SEPARATION;
        // SAFETY: valid address
        unsafe { inttrig::INTTRIG::new(addr) }
    }

    /// Returns the interrupts pending register of a given interrupt number.
    #[inline]
    pub fn ip<I: InterruptNumber>(int_nr: I) -> intip::INTIP {
        let addr = P::BASE + Self::INT_OFFSET + int_nr.number() as usize * Self::INT_SEPARATION;
        // SAFETY: valid address
        unsafe { intip::INTIP::new(addr) }
    }

    /// Returns the interrupts enable register of a given interrupt number.
    #[inline]
    pub fn ie<I: InterruptNumber>(int_nr: I) -> intie::INTIE {
        let addr = P::BASE + Self::INT_OFFSET + int_nr.number() as usize * Self::INT_SEPARATION;
        // SAFETY: valid interrupt_number
        unsafe { intie::INTIE::new(addr) }
    }

    /// Returns the attribute register of a given interrupt number.
    #[inline]
    pub fn attr<I: InterruptNumber>(int_nr: I) -> intattr::INTATTR {
        let addr = P::BASE + Self::INT_OFFSET + int_nr.number() as usize * Self::INT_SEPARATION;
        // SAFETY: valid address
        unsafe { intattr::INTATTR::new(addr) }
    }

    /// Returns the control register of this interrupt.
    #[inline]
    pub fn ctl<I: InterruptNumber>(int_nr: I) -> intctl::INTCTL {
        let addr = P::BASE + Self::INT_OFFSET + int_nr.number() as usize * Self::INT_SEPARATION;
        // SAFETY: valid address
        unsafe { intctl::INTCTL::new(addr) }
    }
}
