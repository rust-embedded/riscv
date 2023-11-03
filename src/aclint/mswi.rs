//! Machine-level Software Interrupt Device.

pub use super::HartIdNumber;
use crate::common::unsafe_peripheral;

/// MSWI peripheral.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MSWI {
    /// `MSIP` register for HART ID 0.  In multi-HART architectures,
    /// use [`MSWI::msip`] for accessing the `MSIP` of other HARTs.
    pub msip0: MSIP,
}

impl MSWI {
    /// Creates a new `MSWI` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `MSWI` peripheral.
    #[inline]
    pub const unsafe fn new(address: usize) -> Self {
        Self {
            msip0: MSIP::new(address),
        }
    }

    /// Returns the `MSIP` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MSWI::msip0`].
    #[inline]
    pub fn msip<H: HartIdNumber>(&self, hart_id: H) -> MSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MSIP::new(self.msip0.get_ptr().offset(hart_id.number() as _) as _) }
    }
}

unsafe_peripheral!(MSIP, u32, RW);

impl MSIP {
    /// Returns `true` if a machine software interrupt is pending.
    #[inline]
    pub fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    /// Writes to the register to trigger a machine software interrupt.
    #[inline]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a machine software interrupt.
    #[inline]
    pub fn unpend(self) {
        self.register.write(0);
    }
}

#[cfg(test)]
mod test {
    use super::super::test::HartId;
    use super::*;

    #[test]
    fn test_mswi() {
        // slice to emulate the interrupt pendings register
        let raw_reg = [0u32; HartId::MAX_HART_ID_NUMBER as usize + 1];
        // SAFETY: valid memory address
        let mswi = unsafe { MSWI::new(raw_reg.as_ptr() as _) };

        for i in 0..=HartId::MAX_HART_ID_NUMBER {
            let hart_id = HartId::from_number(i).unwrap();
            let msip = mswi.msip(hart_id);
            assert!(!msip.is_pending());
            assert_eq!(raw_reg[i as usize], 0);
            msip.pend();
            assert!(msip.is_pending());
            assert_ne!(raw_reg[i as usize], 0);
            msip.unpend();
            assert!(!msip.is_pending());
            assert_eq!(raw_reg[i as usize], 0);
        }
    }
}
