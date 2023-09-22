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

    /// Sets the Machine Software Interrupt bit of the `mie` CSR.
    /// This bit must be set for the `MSWI` to trigger machine software interrupts.
    ///
    /// # Safety
    ///
    /// Enabling the `MSWI` may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable() {
        riscv::register::mie::set_msoft();
    }

    /// Clears the Machine Software Interrupt bit of the `mie` CSR.
    /// When cleared, the `MSWI` cannot trigger machine software interrupts.
    #[inline]
    pub fn disable() {
        // SAFETY: it is safe to disable interrupts
        unsafe { riscv::register::mie::clear_msoft() };
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
