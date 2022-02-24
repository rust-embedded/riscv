use crate::register::mcycle;
use core::convert::Infallible;
use embedded_hal::delay::blocking::DelayUs;

/// Machine mode cycle counter (`mcycle`) as a delay provider
#[derive(Copy, Clone)]
pub struct McycleDelay {
    ticks_second: u32,
}

impl McycleDelay {
    /// Constructs the delay provider
    #[inline(always)]
    pub fn new(ticks_second: u32) -> Self {
        Self { ticks_second }
    }
}

impl DelayUs for McycleDelay {
    type Error = Infallible;

    #[inline]
    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        let t0 = mcycle::read64();
        let clock = ((us as u64) * (self.ticks_second as u64)) / 1_000_000;
        while mcycle::read64().wrapping_sub(t0) <= clock {}
        Ok(())
    }
}
