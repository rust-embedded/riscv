//! Peripheral definitions for the E310x chip.
//! This is a simple example of how to use the `riscv-peripheral` crate to generate
//! peripheral definitions for a target.

use riscv_pac::{
    result::{Error, Result},
    ExternalInterruptNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HartId {
    H0 = 0,
}

unsafe impl HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: usize = Self::H0 as usize;

    #[inline]
    fn number(self) -> usize {
        self as _
    }

    #[inline]
    fn from_number(number: usize) -> Result<Self> {
        match number {
            0 => Ok(Self::H0),
            _ => Err(Error::InvalidVariant(number)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
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
    const MAX_INTERRUPT_NUMBER: usize = Self::I2C0 as usize;

    #[inline]
    fn number(self) -> usize {
        self as _
    }

    #[inline]
    fn from_number(number: usize) -> Result<Self> {
        if number == 0 || number > Self::MAX_INTERRUPT_NUMBER {
            Err(Error::InvalidVariant(number))
        } else {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute::<usize, Interrupt>(number) })
        }
    }
}

unsafe impl ExternalInterruptNumber for Interrupt {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
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
    const MAX_PRIORITY_NUMBER: usize = Self::P7 as usize;

    #[inline]
    fn number(self) -> usize {
        self as _
    }

    #[inline]
    fn from_number(number: usize) -> Result<Self> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(Error::InvalidVariant(number))
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute::<usize, Priority>(number) })
        }
    }
}

riscv_peripheral::clint_codegen!(
    base 0x0200_0000,
    mtime_freq 32_768,
    harts [HartId::H0 => 0],
);

riscv_peripheral::plic_codegen!(
    base 0x0C00_0000,
    harts [HartId::H0 => 0],
);

fn main() {}
