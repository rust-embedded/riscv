use crate::peripheral::common::{peripheral_reg, Reg, RW};

/// Machine-level Timer Device (MTIMER).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MTIMER {
    /// [`MTIMECMP`] register for HART ID 0.  In multi-HART architectures,
    /// use [`MTIMER::mtimecmp`] for accessing the MTIMECMP of other HARTs.
    pub mtimecmp0: MTIMECMP,
    /// The MTIME register is shared among all the HARTS.
    pub mtime: MTIME,
}

impl MTIMER {
    pub const fn new(mtimecmp: usize, mtime: usize) -> Self {
        Self {
            mtimecmp0: MTIMECMP::new(mtimecmp),
            mtime: MTIME::new(mtime),
        }
    }

    pub unsafe fn mtimecmp(&self, hart_id: u16) -> MTIMECMP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        MTIMECMP::new(self.mtimecmp0.ptr.offset(hart_id as _) as _)
    }
}

// MTIMECMP register.
peripheral_reg!(MTIMECMP, u64, RW);

// MTIME register.
peripheral_reg!(MTIME, u64, RW);
