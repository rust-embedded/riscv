//! Peripheral definitions for the E310x chip.
//! This is a simple example of how to use the `riscv-peripheral` crate to generate
//! peripheral definitions for a target.

use riscv_pac::result::{Error, Result};
use riscv_pac::{pac_enum, ExternalInterruptNumber};

#[repr(u16)]
#[pac_enum(unsafe HartIdNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HartId {
    H0 = 0,
}

unsafe impl HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: u16 = Self::H0 as u16;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self> {
        if number > Self::MAX_HART_ID_NUMBER {
            Err(Error::InvalidVariant(number as usize))
        } else {
            // SAFETY: valid context number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    WATCHDOG = 1,
    RTC = 2,
    UART0 = 3,
    UART1 = 4,
    QSPI0 = 5,
    QSPI1 = 6,
    QSPI2 = 7,
    GPIO0 = 8,
    GPIO1 = 9,
    GPIO2 = 10,
    GPIO3 = 11,
    GPIO4 = 12,
    GPIO5 = 13,
    GPIO6 = 14,
    GPIO7 = 15,
    GPIO8 = 16,
    GPIO9 = 17,
    GPIO10 = 18,
    GPIO11 = 19,
    GPIO12 = 20,
    GPIO13 = 21,
    GPIO14 = 22,
    GPIO15 = 23,
    GPIO16 = 24,
    GPIO17 = 25,
    GPIO18 = 26,
    GPIO19 = 27,
    GPIO20 = 28,
    GPIO21 = 29,
    GPIO22 = 30,
    GPIO23 = 31,
    GPIO24 = 32,
    GPIO25 = 33,
    GPIO26 = 34,
    GPIO27 = 35,
    GPIO28 = 36,
    GPIO29 = 37,
    GPIO30 = 38,
    GPIO31 = 39,
    PWM0CMP0 = 40,
    PWM0CMP1 = 41,
    PWM0CMP2 = 42,
    PWM0CMP3 = 43,
    PWM1CMP0 = 44,
    PWM1CMP1 = 45,
    PWM1CMP2 = 46,
    PWM1CMP3 = 47,
    PWM2CMP0 = 48,
    PWM2CMP1 = 49,
    PWM2CMP2 = 50,
    PWM2CMP3 = 51,
    I2C0 = 52,
}

unsafe impl InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: u16 = Self::I2C0 as u16;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self> {
        if number == 0 || number > Self::MAX_INTERRUPT_NUMBER {
            Err(Error::InvalidVariant(number as usize))
        } else {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

unsafe impl ExternalInterruptNumber for Interrupt {}

#[repr(u8)]
#[pac_enum(unsafe PriorityNumber)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

unsafe impl PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = Self::P7 as u8;

    #[inline]
    fn number(self) -> u8 {
        self as _
    }

    #[inline]
    fn from_number(number: u8) -> Result<Self> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(Error::InvalidVariant(number as usize))
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

#[cfg(feature = "aclint-hal-async")]
riscv_peripheral::clint_codegen!(
    base 0x0200_0000,
    freq 32_768,
    async_delay,
    mtimecmps [mtimecmp0=(HartId::H0,"`H0`")],
    msips [msip0=(HartId::H0,"`H0`")],
);

#[cfg(not(feature = "aclint-hal-async"))]
riscv_peripheral::clint_codegen!(
    base 0x0200_0000,
    freq 32_768,
    mtimecmps [mtimecmp0=(HartId::H0,"`H0`")],
    msips [msip0=(HartId::H0,"`H0`")],
);

riscv_peripheral::plic_codegen!(
    base 0x0C00_0000,
    ctxs [ctx0=(HartId::H0,"`H0`")],
);

#[cfg(feature = "aclint-hal-async")]
/// extern functions needed by the `riscv-peripheral` crate for the `async` feature.
///
/// # Note
///
/// The functionality in this module is just to illustrate how to enable the `async` feature
/// The timer queue used here, while functional, is unsound and should not be used in production.
/// In this case, you should protect the timer queue with a mutex or critical section.
/// For a more robust implementation, use proper timer queues such as the ones provided by `embassy-time`
mod async_no_mangle {
    use super::CLINT;
    use heapless::binary_heap::{BinaryHeap, Min};
    use riscv_peripheral::{aclint::mtimer::MTIMER, hal_async::aclint::Timer};

    const N_TIMERS: usize = 16;
    static mut TIMER_QUEUE: BinaryHeap<Timer, Min, N_TIMERS> = BinaryHeap::new();

    #[no_mangle]
    fn _riscv_peripheral_aclint_mtimer() -> MTIMER {
        CLINT::mtimer()
    }

    #[no_mangle]
    fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer> {
        unsafe { TIMER_QUEUE.push(t) }
    }

    #[no_mangle]
    fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64> {
        let mut next_expires = None;
        while let Some(t) = unsafe { TIMER_QUEUE.peek() } {
            if t.expires() > current_tick {
                next_expires = Some(t.expires());
                break;
            }
            let t = unsafe { TIMER_QUEUE.pop() }.unwrap();
            t.waker().wake_by_ref();
        }
        next_expires
    }
}

fn main() {}
