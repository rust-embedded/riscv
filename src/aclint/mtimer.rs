//! Machine-level Timer Device.

pub use super::HartIdNumber;
use crate::common::safe_peripheral;

/// MTIMER peripheral.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MTIMER {
    /// `MTIMECMP` register for HART ID 0.  In multi-HART architectures,
    /// use [`MTIMER::mtimecmp`] for accessing the `MTIMECMP` of other HARTs.
    pub mtimecmp0: MTIMECMP,
    /// The `MTIME` register is shared among all the HARTs.
    pub mtime: MTIME,
}

impl MTIMER {
    /// Creates a new `MTIMER` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `MTIMER` peripheral.
    #[inline]
    pub const unsafe fn new(mtimecmp: usize, mtime: usize) -> Self {
        Self {
            mtimecmp0: MTIMECMP::new(mtimecmp),
            mtime: MTIME::new(mtime),
        }
    }

    /// Sets the Machine Timer Interrupt bit of the `mie` CSR.
    /// This bit must be set for the `MTIMER` to trigger machine timer interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `MTIMER` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable() {
        riscv::register::mie::set_mtimer();
    }

    /// Clears the Machine Timer Interrupt bit of the `mie` CSR.
    /// When cleared, the `MTIMER` cannot trigger machine timer interrupts.
    #[inline]
    pub fn disable() {
        // SAFETY: it is safe to disable interrupts
        unsafe { riscv::register::mie::clear_mtimer() };
    }

    /// Returns the `MTIME` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MTIMER::mtimecmp0`].
    #[inline]
    pub fn mtimecmp<H: HartIdNumber>(&self, hart_id: H) -> MTIMECMP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MTIMECMP::new(self.mtimecmp0.get_ptr().offset(hart_id.number() as _) as _) }
    }
}

// MTIMECMP register.
safe_peripheral!(MTIMECMP, u64, RW);

// MTIME register.
safe_peripheral!(MTIME, u64, RW);
