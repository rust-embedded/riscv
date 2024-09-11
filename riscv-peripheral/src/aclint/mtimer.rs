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
    /// The base addresses must point to valid `MTIMECMP` and `MTIME` peripherals.
    #[inline]
    pub const unsafe fn new(mtimecmp: usize, mtime: usize) -> Self {
        Self {
            mtimecmp0: MTIMECMP::new(mtimecmp),
            mtime: MTIME::new(mtime),
        }
    }

    /// Returns the `MTIMECMP` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MTIMER::mtimecmp0`].
    #[inline]
    pub fn mtimecmp<H: HartIdNumber>(&self, hart_id: H) -> MTIMECMP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MTIMECMP::new(self.mtimecmp0.get_ptr().add(hart_id.number()) as _) }
    }

    /// Returns the `MTIMECMP` register for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`MTIMER::mtimecmp`] instead.
    #[inline]
    pub fn mtimecmp_mhartid(&self) -> MTIMECMP {
        let hart_id = riscv::register::mhartid::read();
        // SAFETY: `hart_id` is valid for the target and is the current hart
        unsafe { MTIMECMP::new(self.mtimecmp0.get_ptr().add(hart_id) as _) }
    }
}

// MTIMECMP register.
safe_peripheral!(MTIMECMP, u64, RW);

// MTIME register.
safe_peripheral!(MTIME, u64, RW);

#[cfg(test)]
mod test {
    use super::super::test::HartId;
    use super::*;

    #[test]
    fn check_mtimer() {
        // slice to emulate the mtimecmp registers
        let raw_mtimecmp = [0u64; HartId::MAX_HART_ID_NUMBER as usize + 1];
        let raw_mtime = 0u64;
        // SAFETY: valid memory addresses
        let mtimer =
            unsafe { MTIMER::new(raw_mtimecmp.as_ptr() as _, &raw_mtime as *const u64 as _) };

        assert_eq!(
            mtimer.mtimecmp(HartId::H0).get_ptr() as usize,
            raw_mtimecmp.as_ptr() as usize
        );
        assert_eq!(mtimer.mtimecmp(HartId::H1).get_ptr() as usize, unsafe {
            raw_mtimecmp.as_ptr().offset(1)
        }
            as usize);
        assert_eq!(mtimer.mtimecmp(HartId::H2).get_ptr() as usize, unsafe {
            raw_mtimecmp.as_ptr().offset(2)
        }
            as usize);
        assert_eq!(
            mtimer.mtime.get_ptr() as usize,
            &raw_mtime as *const u64 as _
        );
    }
}
