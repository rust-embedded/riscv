//! RISC-V peripherals
use core::marker::PhantomData;

// Platform-Level Interrupt Controller
#[cfg(feature = "plic")]
pub mod plic;

/// Interface for a context of the PLIC peripheral.
///
/// # Note
///
/// The RISC-V standard does not specify a fixed location for the PLIC.
/// Thus, we use const generics to map a PLIC to the desired memory location.
/// Each platform must specify the base address of the PLIC on the platform.
///
/// The PLIC standard allows up to 15_872 different contexts for interfacing the PLIC.
/// Usually, each HART uses a dedicated context. In this way, they do not interfere
/// with each other when attending to external interruptions.
///
/// Each platform must specify a type alias for every context of its PLIC.
/// In single core platforms, there is usually just one context (context 0).
/// In this case, PACs can define their PLIC as follows:
/// ```
/// pub type PLIC = riscv::peripheral::PLIC<ADDRESS, 0>;
/// ```
/// where `ADDRESS` is the base address of the PLIC peripheral in the target platform.
///
/// For multicore architectures with more than one context, these can be defined as follows:
///  ```
/// pub type PLIC0 = riscv::peripheral::PLIC<ADDRESS, 0>;
/// pub type PLIC1 = riscv::peripheral::PLIC<ADDRESS, 1>;
/// ```
/// In this way, each HART can own a `PLIC` structure that points to a dedicated context.
#[allow(clippy::upper_case_acronyms)]
#[cfg(feature = "plic")]
#[derive(Default)]
pub struct PLIC<const BASE: usize, const CONTEXT: usize> {
    _marker: PhantomData<*const ()>,
}

#[cfg(feature = "plic")]
impl<const BASE: usize, const CONTEXT: usize> PLIC<BASE, CONTEXT> {
    /// Pointer to the register block
    pub const PTR: *const self::plic::RegisterBlock = BASE as *const _;

    /// Creates a new interface for the PLIC peripheral. PACs can use this
    /// function to add a PLIC interface to their `Peripherals` struct.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
