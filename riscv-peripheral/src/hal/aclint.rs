//! Delay trait implementation for (A)CLINT peripherals

use crate::aclint::mtimer::MTIME;
pub use crate::hal::delay::DelayNs;

/// Delay implementation for (A)CLINT peripherals.
pub struct Delay {
    mtime: MTIME,
    freq: usize,
}

impl Delay {
    /// Creates a new `Delay` instance.
    #[inline]
    pub const fn new(mtime: MTIME, freq: usize) -> Self {
        Self { mtime, freq }
    }

    /// Returns the frequency of the `MTIME` register.
    #[inline]
    pub const fn get_freq(&self) -> usize {
        self.freq
    }

    /// Sets the frequency of the `MTIME` register.
    #[inline]
    pub fn set_freq(&mut self, freq: usize) {
        self.freq = freq;
    }

    /// Returns the `MTIME` register.
    #[inline]
    pub const fn get_mtime(&self) -> MTIME {
        self.mtime
    }
}

impl DelayNs for Delay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t0 = self.mtime.read();
        let ns_64: u64 = ns.into();
        let n_ticks = ns_64 * self.freq as u64 / 1_000_000_000;
        while self.mtime.read().wrapping_sub(t0) < n_ticks {}
    }
}
