//! Asynchronous delay implementation for the (A)CLINT peripheral.

use crate::aclint::mtimer::{MTIME, MTIMECMP, MTIMER};
pub use crate::hal_async::delay::DelayNs;
use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

extern "Rust" {
    /// Returns the `MTIMER` register for the given HART ID.
    /// This is necessary for [`MachineExternal`] to obtain the corresponding `MTIMER` register.
    ///
    /// # Safety
    ///
    /// Do not call this function directly. It is only meant to be called by [`MachineExternal`].
    fn _riscv_peripheral_aclint_mtimer(hart_id: usize) -> MTIMER;

    /// Tries to push a new timer to the timer queue assigned to the given HART ID.
    /// If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
    ///
    /// # Note
    ///
    /// the [`Delay`] reference allows to access the `MTIME` and `MTIMECMP` registers,
    /// as well as handy information such as the HART ID or the clock frequency of the `MTIMER` peripheral.
    ///
    /// # Safety
    ///
    /// Do not call this function directly. It is only meant to be called by [`DelayAsync`].
    fn _riscv_peripheral_push_timer(hart_id: usize, delay: &Delay, t: Timer) -> Result<(), Timer>;

    /// Pops a expired timer from the timer queue assigned to the given HART ID.
    /// If the queue is empty, it returns `Err(None)`.
    /// Alternatively, if the queue is not empty but the earliest timer has not expired yet,
    /// it returns `Err(Some(next_expires))` where `next_expires` is the tick at which this timer expires.
    ///
    /// # Safety
    ///
    /// It is extremely important that this function only returns a timer that has expired.
    /// Otherwise, the timer will be lost and the waker will never be called.
    ///
    /// Do not call this function directly. It is only meant to be called by [`MachineExternal`] and [`DelayAsync`].
    fn _riscv_peripheral_pop_timer(hart_id: usize, current_tick: u64)
        -> Result<Timer, Option<u64>>;
}

/// Machine-level timer interrupt handler.
/// This handler is triggered whenever the `MTIME` register reaches the value of the `MTIMECMP` register.
#[no_mangle]
#[allow(non_snake_case)]
fn MachineExternal() {
    let hart_id = riscv::register::mhartid::read();
    let mtimer = unsafe { _riscv_peripheral_aclint_mtimer(hart_id) };
    let (mtime, mtimercmp) = (mtimer.mtime, mtimer.mtimecmp_mhartid());
    schedule_machine_external(hart_id, mtime, mtimercmp);
}

fn schedule_machine_external(hart_id: usize, mtime: MTIME, mtimercmp: MTIMECMP) {
    unsafe { riscv::register::mie::clear_mtimer() }; // disable machine timer interrupts to avoid reentrancy
    loop {
        let current_tick = mtime.read();
        let timer = unsafe { _riscv_peripheral_pop_timer(hart_id, current_tick) };
        match timer {
            Ok(timer) => {
                debug_assert!(timer.expires() <= current_tick);
                timer.wake();
            }
            Err(e) => {
                if let Some(next_expires) = e {
                    debug_assert!(next_expires > current_tick);
                    mtimercmp.write(next_expires); // schedule next interrupt at next_expires
                    unsafe { riscv::register::mie::set_mtimer() }; // enable machine timer interrupts again
                } else {
                    mtimercmp.write(u64::MAX); // write max to clear and "disable" the interrupt
                }
                break;
            }
        }
    }
}

/// Asynchronous delay implementation for (A)CLINT peripherals.
#[derive(Clone)]
pub struct Delay {
    mtime: MTIME,
    hart_id: usize,
    mtimecmp: MTIMECMP,
    freq: usize,
}

impl Delay {
    /// Creates a new `Delay` instance.
    #[inline]
    pub fn new<H: riscv_pac::HartIdNumber>(mtimer: MTIMER, hart_id: H, freq: usize) -> Self {
        Self {
            mtime: mtimer.mtime,
            hart_id: hart_id.number() as _,
            mtimecmp: mtimer.mtimecmp(hart_id),
            freq,
        }
    }

    /// Creates a new `Delay` instance for the current HART.
    /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
    ///
    /// # Note
    ///
    /// This function can only be used in M-mode. For S-mode, use [`Delay::new_mhartid`] instead.
    #[inline]
    pub fn new_mhartid(mtimer: MTIMER, freq: usize) -> Self {
        let hart_id = riscv::register::mhartid::read();
        Self {
            mtime: mtimer.mtime,
            hart_id,
            mtimecmp: mtimer.mtimecmp_mhartid(),
            freq,
        }
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

    /// Returns the `MTIMECMP` register.
    #[inline]
    pub const fn get_mtimecmp(&self) -> MTIMECMP {
        self.mtimecmp
    }

    /// Returns the hart ID.
    #[inline]
    pub const fn get_hart_id(&self) -> usize {
        self.hart_id
    }
}

/// Timer queue entry.
#[derive(Debug)]
pub struct Timer {
    expires: u64,
    waker: Waker,
}

impl Timer {
    /// Creates a new timer queue entry.
    #[inline]
    pub fn new(expires: u64, waker: Waker) -> Self {
        Self { expires, waker }
    }

    /// Returns the tick at which the timer expires.
    #[inline]
    pub const fn expires(&self) -> u64 {
        self.expires
    }

    /// Wakes the waker associated with this timer.
    #[inline]
    pub fn wake(&self) {
        self.waker.wake_by_ref();
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires
    }
}

impl Eq for Timer {}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.expires.cmp(&other.expires)
    }
}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.expires.cmp(&other.expires))
    }
}

struct DelayAsync {
    delay: Delay,
    expires: u64,
    pushed: bool,
}

impl DelayAsync {
    pub fn new(delay: Delay, n_ticks: u64) -> Self {
        let t0 = delay.mtime.read();
        let expires = t0.wrapping_add(n_ticks);
        Self {
            delay,
            expires,
            pushed: false,
        }
    }
}

impl Future for DelayAsync {
    type Output = ();

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.delay.mtime.read() < self.expires {
            if !self.pushed {
                // we only push the timer to the queue the first time we poll
                self.pushed = true;
                let timer = Timer::new(self.expires, cx.waker().clone());
                unsafe {
                    _riscv_peripheral_push_timer(self.delay.hart_id, &self.delay, timer)
                        .expect("timer queue is full");
                };
                // we also need to schedule the interrupt if the timer we just pushed is the earliest one
                schedule_machine_external(
                    self.delay.hart_id,
                    self.delay.mtime,
                    self.delay.mtimecmp,
                );
            }
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl DelayNs for Delay {
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        let n_ticks = ns as u64 * self.get_freq() as u64 / 1_000_000_000;
        DelayAsync::new(self.clone(), n_ticks).await;
    }

    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * self.get_freq() as u64 / 1_000_000;
        DelayAsync::new(self.clone(), n_ticks).await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * self.get_freq() as u64 / 1_000;
        DelayAsync::new(self.clone(), n_ticks).await;
    }
}
