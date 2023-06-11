pub use super::{HartIdNumber, SSWI};
use crate::peripheral::common::{unsafe_peripheral, RW};
use crate::register::mie;

impl SSWI {
    /// Creates a new `SSWI` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `SSWI` peripheral.
    pub unsafe fn new(address: usize) -> Self {
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
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`SSWI::setssip0`].
    #[inline(always)]
    pub fn setssip<H: HartIdNumber>(&self, hart_id: H) -> SETSSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { SETSSIP::new(self.setssip0.get_ptr().offset(hart_id.number() as _) as _) }
    }
}

unsafe_peripheral!(SETSSIP, u32, RW);

impl SETSSIP {
    /// Returns `true` if a supervisor software interrupt is pending.
    #[inline(always)]
    pub fn is_pending(self) -> bool {
        self.register.read() == 1
    }

    /// Writes to the register to trigger a supervisor software interrupt.
    #[inline(always)]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a supervisor software interrupt.
    #[inline(always)]
    pub fn unpend(self) {
        self.register.write(0);
    }
}
