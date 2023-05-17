use crate::peripheral::common::{peripheral_reg, Reg, RW};
use crate::register::mie;

/// Machine-level Timer Device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MTIMER {
    /// [`MTIMECMP`] register for HART ID 0.  In multi-HART architectures,
    /// use [`MTIMER::mtimecmp`] for accessing the `MTIMECMP` of other HARTs.
    pub mtimecmp0: MTIMECMP,
    /// The `MTIME` register is shared among all the HARTs.
    pub mtime: MTIME,
}

impl MTIMER {
    pub const fn new(mtimecmp: usize, mtime: usize) -> Self {
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

    /// Returns the `MTIMECMP` register for the HART which ID is `hart_id`.
    ///
    /// # Safety
    ///
    /// `hart_id` must be valid for the target.
    /// Otherwise, the resulting `MTIMECMP` register will point to a reserved memory region.
    pub unsafe fn mtimecmp(&self, hart_id: u16) -> MTIMECMP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        MTIMECMP::from_ptr(self.mtimecmp0.ptr.offset(hart_id as _))
    }
}

// MTIMECMP register.
peripheral_reg!(MTIMECMP, u64, RW);

// MTIME register.
peripheral_reg!(MTIME, u64, RW);
