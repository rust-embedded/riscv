pub use super::{HartIdNumber, MTIMER};
use crate::peripheral::common::{peripheral_reg, RW};
use crate::register::mie;

impl MTIMER {
    /// Creates a new `MTIMER` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `MTIMER` peripheral.
    pub unsafe fn new(mtimecmp: usize, mtime: usize) -> Self {
        Self {
            mtimecmp0: MTIMECMP::new(mtimecmp),
            mtime: MTIME::new(mtime),
        }
    }

    /// Sets the Machine Timer Interrupt bit of the [`crate::register::mie`] CSR.
    /// This bit must be set for the `MTIMER` to trigger machine timer interrupts.
    #[inline(always)]
    pub unsafe fn enable() {
        mie::set_mtimer();
    }

    /// Clears the Machine Timer Interrupt bit of the [`crate::register::mie`] CSR.
    /// When cleared, the `MTIMER` cannot trigger machine timer interrupts.
    #[inline(always)]
    pub unsafe fn disable() {
        mie::clear_mtimer();
    }

    /// Returns the `MTIME` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MTIMER::mtimecmp0`].
    #[inline(always)]
    pub fn mtimecmp<H: HartIdNumber>(&self, hart_id: H) -> MTIMECMP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MTIMECMP::from_ptr(self.mtimecmp0.get_ptr().offset(hart_id.number() as _)) }
    }
}

// MTIMECMP register.
peripheral_reg!(MTIMECMP, u64, RW);

// MTIME register.
peripheral_reg!(MTIME, u64, RW);
