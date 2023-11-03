//! Delay trait implementation for (A)CLINT peripherals

use crate::aclint::mtimer::MTIME;
pub use crate::hal::delay::DelayUs;

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

impl DelayUs for Delay {
    #[inline]
    fn delay_us(&mut self, us: u32) {
        let time_from = self.mtime.read();
        let time_to = time_from.wrapping_add(us as u64 * self.freq as u64 / 1_000_000);

        while time_to < self.mtime.read() {} // wait for overflow
        while time_to > self.mtime.read() {} // wait for time to pass
    }
}
