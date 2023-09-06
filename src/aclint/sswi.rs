pub use super::HartIdNumber;
use crate::common::unsafe_peripheral;

/// Supervisor-level Software Interrupt Device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SSWI {
    /// `SETSSIP` register for HART ID 0.  In multi-HART architectures,
    /// use [`SSWI::setssip`] for accessing the `SETSSIP` of other HARTs.
    pub setssip0: SETSSIP,
}

impl SSWI {
    /// Creates a new `SSWI` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `SSWI` peripheral.
    #[inline]
    pub const unsafe fn new(address: usize) -> Self {
        Self {
            setssip0: SETSSIP::new(address),
        }
    }

    /// Sets the Supervisor Software Interrupt bit of the `mie` CSR.
    /// This bit must be set for the `SSWI` to trigger supervisor software interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `SSWI` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable() {
        riscv::register::mie::set_ssoft();
    }

    /// Clears the Supervisor Software Interrupt bit of the `mie` CSR.
    /// When cleared, the `SSWI` cannot trigger supervisor software interrupts.
    #[inline]
    pub fn disable() {
        // SAFETY: it is safe to disable interrupts
        unsafe { riscv::register::mie::clear_ssoft() };
    }

    /// Returns the `SETSSIP` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`SSWI::setssip0`].
    #[inline]
    pub fn setssip<H: HartIdNumber>(&self, hart_id: H) -> SETSSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { SETSSIP::new(self.setssip0.get_ptr().offset(hart_id.number() as _) as _) }
    }
}

unsafe_peripheral!(SETSSIP, u32, RW);

impl SETSSIP {
    /// Returns `true` if a supervisor software interrupt is pending.
    #[inline]
    pub fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    /// Writes to the register to trigger a supervisor software interrupt.
    #[inline]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a supervisor software interrupt.
    #[inline]
    pub fn unpend(self) {
        self.register.write(0);
    }
}
