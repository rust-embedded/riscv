//! Peripheral definitions for the E310x chip.
//! This is a simple example of how to use the `riscv-peripheral` crate to generate
//! peripheral definitions for a target.

use riscv_pac::{HartIdNumber, InterruptNumber, PriorityNumber};

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HartId {
    H0 = 0,
}

unsafe impl HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: u16 = 0;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number > Self::MAX_HART_ID_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid context number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
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
    const MAX_INTERRUPT_NUMBER: u16 = 52;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number == 0 || number > Self::MAX_INTERRUPT_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
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
    const MAX_PRIORITY_NUMBER: u8 = 7;

    #[inline]
    fn number(self) -> u8 {
        self as _
    }

    #[inline]
    fn from_number(number: u8) -> Result<Self, u8> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

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

fn main() {}
