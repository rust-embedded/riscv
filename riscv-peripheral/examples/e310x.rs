//! Peripheral definitions for the E310x chip.
//!
//! This is a simple example of how to use the `riscv-peripheral` crate to generate
//! peripheral definitions for a target.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[riscv::pac_enum(unsafe HartIdNumber)]
pub enum HartId {
    H0 = 0,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[riscv::pac_enum(unsafe ExternalInterruptNumber)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[riscv::pac_enum(unsafe PriorityNumber)]
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

// We can define CLINT::new() as a public, safe function
riscv_peripheral::clint_codegen!(
    pub CLINT,
    base 0x0200_0000,
    mtime_freq 32_768,
    harts [HartId::H0 => 0]
);

// We can define PLIC::new() as a private, safe function...
riscv_peripheral::plic_codegen!(
    PLIC,
    base 0x0C00_0000,
    harts [HartId::H0 => 0]
);

// ... and then implement a public, unsafe function to steal the PLIC instance
// Usually, this function is implemented by svd2rust, but we do it manually here
impl PLIC {
    pub unsafe fn steal() -> Self {
        PLIC::new()
    }
}

fn main() {
    let _clint = CLINT::new();
    let _plic = unsafe { PLIC::steal() };
}
