//! Machine-level Timer Device.

pub use super::{Clint, HartIdNumber};
use crate::common::safe_peripheral;
use riscv::register::{mhartid, mie, mip};

/// Trait for an MTIMER device.
///
/// # Note
///
/// For CLINT peripherals, this trait is automatically implemented.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a target with an MTIMER device.
/// * The `MTIMECMP` registers base address `MTIMECMP_BASE` must be valid for the target.
/// * The `MTIME` registers base address `MTIME_BASE` must be valid for the target.
/// * The `MTIME` clock frequency `MTIME_FREQ` must be valid for the target.
pub unsafe trait Mtimer: Copy {
    /// Base address of the MTIMECMP registers.
    const MTIMECMP_BASE: usize;
    /// Base address of the MTIME register.
    const MTIME_BASE: usize;
    /// Clock frequency of the MTIME register.
    const MTIME_FREQ: usize;
}

// SAFETY: the offset of the MSWI peripheral is fixed in the CLINT peripheral
unsafe impl<C: Clint> Mtimer for C {
    const MTIMECMP_BASE: usize = C::BASE + 0x4000;
    const MTIME_BASE: usize = C::BASE + 0xBFF8;
    const MTIME_FREQ: usize = C::MTIME_FREQ;
}

/// MTIMER device.
///
/// It has a single fixed-frequency monotonic time counter ([`MTIME`])
/// register and a time compare register ([`MTIMECMP`]) for each HART.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MTIMER<M> {
    _marker: core::marker::PhantomData<M>,
}

impl<M: Mtimer> MTIMER<M> {
    /// Creates a new `MTIMER` device.
    #[inline]
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the base address of the `MTIMECMP` registers.
    #[inline]
    const fn mtimecmp_as_ptr(self) -> *const u64 {
        M::MTIMECMP_BASE as *const u64
    }

    /// Returns the clock frequency of the `MTIME` register.
    #[inline]
    pub const fn mtime_freq(self) -> usize {
        M::MTIME_FREQ
    }

    /// Returns `true` if a machine timer interrupt is pending.
    #[inline]
    pub fn is_interrupting(self) -> bool {
        mip::read().mtimer()
    }

    /// Returns `true` if machine timer interrupts are enabled.
    #[inline]
    pub fn is_enabled(self) -> bool {
        mie::read().mtimer()
    }

    /// Enables machine timer interrupts in the current HART.
    ///
    /// # Safety
    ///
    /// Enabling interrupts may break mask-based critical sections.
    #[inline]
    pub unsafe fn enable(self) {
        unsafe { mie::set_mtimer() };
    }

    /// Disables machine timer interrupts in the current HART.
    #[inline]
    pub fn disable(self) {
        // SAFETY: it is safe to disable interrupts
        unsafe { mie::clear_mtimer() };
    }

    /// Returns the `MTIME` register.
    #[inline]
    pub const fn mtime(self) -> MTIME {
        // SAFETY: valid base address
        unsafe { MTIME::new(M::MTIME_BASE) }
    }

    /// Returns the `MTIMECMP` register for the HART which ID is `hart_id`.
    #[inline]
    pub fn mtimecmp<H: HartIdNumber>(self, hart_id: H) -> MTIMECMP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MTIMECMP::new(self.mtimecmp_as_ptr().add(hart_id.number()) as _) }
    }

    /// Returns the `MTIMECMP` register for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the [`mhartid`] CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`MTIMER::mtimecmp`] instead.
    #[inline]
    pub fn mtimecmp_mhartid(self) -> MTIMECMP {
        let hart_id = mhartid::read();
        // SAFETY: `hart_id` is valid for the target and is the current hart
        unsafe { MTIMECMP::new(self.mtimecmp_as_ptr().add(hart_id) as _) }
    }
}

// MTIMECMP register.
safe_peripheral!(MTIMECMP, u64, RW);

// MTIME register.
safe_peripheral!(MTIME, u64, RW);
