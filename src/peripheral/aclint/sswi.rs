use crate::peripheral::common::{peripheral_reg, Reg, WARL};
use crate::register::mie;

/// Supervisor-level Software Interrupt Device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SSWI {
    /// [`SETSSIP`] register for HART ID 0.  In multi-HART architectures,
    /// use [`SSWI::setssip`] for accessing the `SETSSIP` of other HARTs.
    pub setssip0: SETSSIP,
}

impl SSWI {
    pub const fn new(address: usize) -> Self {
        Self {
            setssip0: SETSSIP::new(address),
        }
    }

    /// Sets the Supervisor Software Interrupt bit of the [`crate::register::mie`] CSR.
    /// This bit must be set for the `SSWI` to trigger supervisor software interrupts.
    #[inline(always)]
    pub unsafe fn enable() {
        mie::set_ssoft();
    }

    /// Clears the Supervisor Software Interrupt bit of the [`crate::register::mie`] CSR.
    /// When cleared, the `SSWI` cannot trigger supervisor software interrupts.
    #[inline(always)]
    pub unsafe fn disable() {
        mie::clear_ssoft();
    }

    /// Returns the `SETSSIP` register for the HART which ID is `hart_id`.
    ///
    /// # Safety
    ///
    /// `hart_id` must be valid for the target.
    /// Otherwise, the resulting `SETSSIP` register will point to a reserved memory region.
    pub unsafe fn setssip(&self, hart_id: u16) -> SETSSIP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        SETSSIP::from_ptr(self.setssip0.ptr.offset(hart_id as _))
    }
}

peripheral_reg!(SETSSIP, u32, WARL);

impl SETSSIP {
    /// Returns `true` if a supervisor software interrupt is pending.
    pub unsafe fn is_pending(self) -> bool {
        self.register.read() == 1
    }

    /// Writes to the register to trigger a supervisor software interrupt.
    pub unsafe fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a supervisor software interrupt.
    pub unsafe fn unpend(self) {
        self.register.write(0);
    }
}
