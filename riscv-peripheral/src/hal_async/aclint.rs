//! Asynchronous delay implementation for the (A)CLINT peripheral.
//!
//! # Note
//!
//! The asynchronous delay implementation for the (A)CLINT peripheral relies on the machine-level timer interrupts.
//! Therefore, it needs to schedule the machine-level timer interrupts via the [`MTIMECMP`] register assigned to the current HART.
//! Thus, the [`Delay`] instance must be created on the same HART that is used to call the asynchronous delay methods.
//!
//! # Requirements
//!
//! The following `extern "Rust"` functions must be implemented:
//!
//! - `fn _riscv_peripheral_aclint_mtimer(hart_id: usize) -> MTIMER`: This function returns the `MTIMER` register for the given HART ID.
//! - `fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer>`: This function pushes a new timer to a timer queue assigned to the given HART ID.
//! If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
//! The logic of timer queues are application-specific and are not provided by this crate.
//! - `fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64>`:
//! This function pops all the expired timers from a timer queue assigned to the current HART ID and wakes their associated wakers.
//! The function returns the next [`MTIME`] tick at which the next timer expires. If the queue is empty, it returns `None`.

use crate::aclint::mtimer::{MTIME, MTIMECMP, MTIMER};
pub use crate::hal_async::delay::DelayNs;
use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

extern "Rust" {
    /// Returns the `MTIMER` register for the current HART ID.
    /// This is necessary for [`MachineTimer`] to obtain the corresponding `MTIMER` register.
    ///
    /// # Safety
    ///
    /// Do not call this function directly. It is only meant to be called by [`MachineTimer`].
    fn _riscv_peripheral_aclint_mtimer() -> MTIMER;

    /// Tries to push a new timer to the timer queue assigned to the `MTIMER` register for the current HART ID.
    /// If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
    ///
    /// # Safety
    ///
    /// Do not call this function directly. It is only meant to be called by [`DelayAsync`].
    fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer>;

    /// Pops all the expired timers from the timer queue assigned to the `MTIMER` register for the
    /// current HART ID and wakes their associated wakers. Once it is done, if the queue is empty,
    /// it returns `None`. Alternatively, if the queue is not empty but the earliest timer has not expired
    /// yet, it returns `Some(next_expires)` where `next_expires` is the tick at which this timer expires.
    ///
    /// # Safety
    ///
    /// Do not call this function directly. It is only meant to be called by [`MachineTimer`] and [`DelayAsync`].
    fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64>;
}

/// Machine-level timer interrupt handler. This handler is triggered whenever the `MTIME`
/// register reaches the value of the `MTIMECMP` register of the current HART.
#[no_mangle]
#[allow(non_snake_case)]
fn MachineTimer() {
    // recover the MTIME and MTIMECMP registers for the current HART
    let mtimer = unsafe { _riscv_peripheral_aclint_mtimer() };
    let (mtime, mtimercmp) = (mtimer.mtime, mtimer.mtimecmp_mhartid());
    // schedule the next machine timer interrupt
    schedule_machine_timer(mtime, mtimercmp);
}

/// Schedules the next machine timer interrupt for the given HART ID according to the timer queue.
fn schedule_machine_timer(mtime: MTIME, mtimercmp: MTIMECMP) {
    unsafe { riscv::register::mie::clear_mtimer() }; // disable machine timer interrupts to avoid reentrancy
    let current_tick = mtime.read();
    if let Some(next_expires) = unsafe { _riscv_peripheral_aclint_wake_timers(current_tick) } {
        debug_assert!(next_expires > current_tick);
        mtimercmp.write(next_expires); // schedule next interrupt at next_expires
        unsafe { riscv::register::mie::set_mtimer() }; // enable machine timer interrupts
    }
}

/// Asynchronous delay implementation for (A)CLINT peripherals.
///
/// # Note
///
/// The asynchronous delay implementation for (A)CLINT peripherals relies on the machine-level timer interrupts.
/// Therefore, it needs to schedule the machine-level timer interrupts via the [`MTIMECMP`] register assigned to the current HART.
/// Thus, the [`Delay`] instance must be created on the same HART that is used to call the asynchronous delay methods.
/// Additionally, the rest of the application must not modify the [`MTIMER`] register assigned to the current HART.
#[derive(Clone)]
pub struct Delay {
    freq: usize,
    mtime: MTIME,
    mtimecmp: MTIMECMP,
}

impl Delay {
    /// Creates a new `Delay` instance for the current HART.
    #[inline]
    pub fn new(freq: usize) -> Self {
        let mtimer = unsafe { _riscv_peripheral_aclint_mtimer() };
        let (mtime, mtimecmp) = (mtimer.mtime, mtimer.mtimecmp_mhartid());
        Self {
            freq,
            mtime,
            mtimecmp,
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
}

impl DelayNs for Delay {
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        let n_ticks = ns as u64 * self.get_freq() as u64 / 1_000_000_000;
        DelayAsync::new(self, n_ticks).await;
    }

    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * self.get_freq() as u64 / 1_000_000;
        DelayAsync::new(self, n_ticks).await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * self.get_freq() as u64 / 1_000;
        DelayAsync::new(self, n_ticks).await;
    }
}

/// Timer queue entry.
/// When pushed to the timer queue via the `_riscv_peripheral_aclint_push_timer` function,
/// this entry provides the necessary information to adapt it to the timer queue implementation.
#[derive(Debug)]
pub struct Timer {
    freq: usize,
    mtime: MTIME,
    mtimecmp: MTIMECMP,
    expires: u64,
    waker: Waker,
}

impl Timer {
    /// Creates a new timer queue entry.
    #[inline]
    const fn new(
        freq: usize,
        mtime: MTIME,
        mtimecmp: MTIMECMP,
        expires: u64,
        waker: Waker,
    ) -> Self {
        Self {
            freq,
            mtime,
            mtimecmp,
            expires,
            waker,
        }
    }

    /// Returns the frequency of the [`MTIME`] register associated with this timer.
    #[inline]
    pub const fn freq(&self) -> usize {
        self.freq
    }

    /// Returns the [`MTIME`] register associated with this timer.
    #[inline]
    pub const fn mtime(&self) -> MTIME {
        self.mtime
    }

    /// Returns the [`MTIMECMP`] register associated with this timer.
    #[inline]
    pub const fn mtimecmp(&self) -> MTIMECMP {
        self.mtimecmp
    }

    /// Returns the tick at which the timer expires.
    #[inline]
    pub const fn expires(&self) -> u64 {
        self.expires
    }

    /// Returns the waker associated with this timer.
    #[inline]
    pub fn waker(&self) -> Waker {
        self.waker.clone()
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.expires == other.expires
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

struct DelayAsync<'a> {
    delay: &'a Delay,
    expires: u64,
    pushed: bool,
}

impl<'a> DelayAsync<'a> {
    pub fn new(delay: &'a Delay, n_ticks: u64) -> Self {
        let t0 = delay.mtime.read();
        let expires = t0.wrapping_add(n_ticks);
        Self {
            delay,
            expires,
            pushed: false,
        }
    }
}

impl<'a> Future for DelayAsync<'a> {
    type Output = ();

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.delay.mtime.read() < self.expires {
            if !self.pushed {
                // we only push the timer to the queue the first time we poll
                self.pushed = true;
                let timer = Timer::new(
                    self.delay.freq,
                    self.delay.mtime,
                    self.delay.mtimecmp,
                    self.expires,
                    cx.waker().clone(),
                );
                unsafe {
                    _riscv_peripheral_aclint_push_timer(timer).expect("timer queue is full");
                };
                // we also need to reschedule the machine timer interrupt
                schedule_machine_timer(self.delay.mtime, self.delay.mtimecmp);
            }
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
