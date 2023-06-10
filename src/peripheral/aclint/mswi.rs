pub use super::{HartIdNumber, MSWI};
use crate::peripheral::common::{peripheral_reg, RW};
use crate::register::mie;

impl MSWI {
    /// Creates a new `MSWI` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `MSWI` peripheral.
    pub unsafe fn new(address: usize) -> Self {
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
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MSWI::msip0`].
    #[inline(always)]
    pub fn msip<H: HartIdNumber>(&self, hart_id: H) -> MSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MSIP::from_ptr(self.msip0.get_ptr().offset(hart_id.number() as _)) }
    }
}

peripheral_reg!(MSIP, u32, RW);

impl MSIP {
    /// Returns `true` if a machine software interrupt is pending.
    #[inline(always)]
    pub fn is_pending(self) -> bool {
        self.read() == 1
    }

    /// Writes to the register to trigger a machine software interrupt.
    #[inline(always)]
    pub fn pend(self) {
        self.write(1);
    }

    /// Clears the register to unpend a machine software interrupt.
    #[inline(always)]
    pub fn unpend(self) {
        self.write(0);
    }
}
