//! Delay trait implementation for (A)CLINT peripherals

use crate::aclint::mtimer::{Mtimer, MTIMER};
use crate::hal::delay::DelayNs;

impl<M: Mtimer> DelayNs for MTIMER<M> {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t0 = self.mtime().read();
        let ns_64: u64 = ns.into();
        let n_ticks = ns_64 * M::MTIME_FREQ as u64 / 1_000_000_000;
        while self.mtime().read().wrapping_sub(t0) < n_ticks {}
    }
}
