use crate::peripheral::common::{peripheral_reg, Reg, WARL};
use crate::register::mie;

/// Machine-level Software Interrupt Device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MSWI {
    /// [`MSIP`] register for HART ID 0.  In multi-HART architectures,
    /// use [`MSWI::msip`] for accessing the `MSIP` of other HARTs.
    pub msip0: MSIP,
}

impl MSWI {
    pub const fn new(address: usize) -> Self {
        Self {
            msip0: MSIP::new(address),
        }
    }
    /// Sets the Machine Software Interrupt bit of the [`crate::register::mie`] CSR.
    /// This bit must be set for the `MSWI` to trigger machine software interrupts.
    #[inline(always)]
    pub unsafe fn enable() {
        mie::set_msoft();
    }

    /// Clears the Machine Software Interrupt bit of the [`crate::register::mie`] CSR.
    /// When cleared, the `MSWI` cannot trigger machine software interrupts.
    #[inline(always)]
    pub unsafe fn disable() {
        mie::clear_msoft();
    }

    /// Returns the `MSIP` register for the HART which ID is `hart_id`.
    ///
    /// # Safety
    ///
    /// `hart_id` must be valid for the target.
    /// Otherwise, the resulting `MSIP` register will point to a reserved memory region.
    pub unsafe fn msip(&self, hart_id: u16) -> MSIP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        MSIP::from_ptr(self.msip0.ptr.offset(hart_id as _))
    }
}

peripheral_reg!(MSIP, u32, WARL);

impl MSIP {
    /// Returns `true` if a machine software interrupt is pending.
    pub unsafe fn is_pending(self) -> bool {
        self.register.read() == 1
    }

    /// Writes to the register to trigger a machine software interrupt.
    pub unsafe fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a machine software interrupt.
    pub unsafe fn unpend(self) {
        self.register.write(0);
    }
}
