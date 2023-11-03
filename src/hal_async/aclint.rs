//! Asynchronous delay implementation for the (A)CLINT peripheral.

use crate::aclint::mtimer::MTIME;
pub use crate::hal::aclint::Delay;
pub use crate::hal_async::delay::DelayUs;
use core::{
    cmp::Ordering,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

enum DelayAsyncState {
    WaitOverflow(u64),
    Wait(u64),
    Ready,
}

struct FSMDelay {
    mtime: MTIME,
    state: DelayAsyncState,
}

impl FSMDelay {
    pub fn new(n_ticks: u64, mtime: MTIME) -> Self {
        let t_from = mtime.read();
        let t_to = t_from.wrapping_add(n_ticks);

        let state = match t_to.cmp(&t_from) {
            Ordering::Less => DelayAsyncState::WaitOverflow(t_to),
            Ordering::Greater => DelayAsyncState::Wait(t_to),
            Ordering::Equal => DelayAsyncState::Ready,
        };

        Self { mtime, state }
    }
}

impl Future for FSMDelay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            DelayAsyncState::WaitOverflow(t_to) => match t_to.cmp(&self.mtime.read()) {
                Ordering::Less => Poll::Pending,
                Ordering::Greater => {
                    self.state = DelayAsyncState::Wait(t_to);
                    Poll::Pending
                }
                Ordering::Equal => {
                    self.state = DelayAsyncState::Ready;
                    Poll::Ready(())
                }
            },
            DelayAsyncState::Wait(t_to) => {
                if self.mtime.read() < t_to {
                    Poll::Pending
                } else {
                    self.state = DelayAsyncState::Ready;
                    Poll::Ready(())
                }
            }
            DelayAsyncState::Ready => Poll::Ready(()),
        }
    }
}

impl DelayUs for Delay {
    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * self.get_freq() as u64 / 1_000_000;
        let state = FSMDelay::new(n_ticks, self.get_mtime());
        state.await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * self.get_freq() as u64 / 1_000;
        let state = FSMDelay::new(n_ticks, self.get_mtime());
        state.await;
    }
}
