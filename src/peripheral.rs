//! RISC-V peripherals
use core::marker::PhantomData;
use core::ops;

// Platform-Level Interrupt Controller
pub mod plic;

/// PLIC peripheral.
///
/// # Note
///
/// The RISC-V standard does not specify a fixed location for the PLIC.
/// Thus, we use const generics to map a PLIC to the desired memory location.
/// Each platform must create a `PLIC<BASE>` struct where `BASE` refers to
/// the base address of the PLIC on the platform.
#[allow(clippy::upper_case_acronyms)]
pub struct PLIC<const BASE: usize> {
    _marker: PhantomData<*const ()>,
}

unsafe impl<const BASE: usize> Send for PLIC<BASE> {}

impl<const BASE: usize> PLIC<BASE> {
    /// Pointer to the register block
    pub const PTR: *const self::plic::RegisterBlock = BASE as *const _;
}

impl<const BASE: usize> ops::Deref for PLIC<BASE> {
    type Target = self::plic::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
