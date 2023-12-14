//! Asynchronous delay implementation for the (A)CLINT peripheral.

use crate::aclint::mtimer::MTIME;
pub use crate::hal::aclint::Delay;
pub use crate::hal_async::delay::DelayNs;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

struct DelayAsync {
    mtime: MTIME,
    t0: u64,
    n_ticks: u64,
    waker: Option<Waker>,
}

impl DelayAsync {
    pub fn new(mtime: MTIME, n_ticks: u64) -> Self {
        let t0 = mtime.read();
        Self {
            mtime,
            t0,
            n_ticks,
            waker: None,
        }
    }
}

impl Future for DelayAsync {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.mtime.read().wrapping_sub(self.t0) < self.n_ticks {
            true => {
                self.get_mut().waker = Some(cx.waker().clone());
                Poll::Pending
            }
            false => {
                if let Some(waker) = self.get_mut().waker.take() {
                    waker.wake();
                } else {
                    // corner case: delay expired before polling for the first time
                    cx.waker().wake_by_ref();
                };
                Poll::Ready(())
            }
        }
    }
}

impl DelayNs for Delay {
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        let n_ticks = ns as u64 * self.get_freq() as u64 / 1_000_000_000;
        DelayAsync::new(self.get_mtime(), n_ticks).await;
    }

    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * self.get_freq() as u64 / 1_000_000;
        DelayAsync::new(self.get_mtime(), n_ticks).await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * self.get_freq() as u64 / 1_000;
        DelayAsync::new(self.get_mtime(), n_ticks).await;
    }
}
