//! UART example for QEMU virt machine
//!
//! This example demonstrates direct UART output on QEMU's virt machine.
//! It writes to the NS16550-compatible UART at 0x1000_0000.

#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use riscv_semihosting::debug::{self, EXIT_SUCCESS};

const UART_BASE: usize = 0x1000_0000;
const UART_THR: usize = UART_BASE;
const UART_IER: usize = UART_BASE + 1;
const UART_FCR: usize = UART_BASE + 2;
const UART_LCR: usize = UART_BASE + 3;
const UART_LSR: usize = UART_BASE + 5;
const LCR_DLAB: u8 = 1 << 7;
const LCR_8N1: u8 = 0x03;
const LSR_THRE: u8 = 1 << 5;

unsafe fn uart_write_reg(off: usize, v: u8) {
    (off as *mut u8).write_volatile(v);
}

unsafe fn uart_read_reg(off: usize) -> u8 {
    (off as *const u8).read_volatile()
}

fn uart_init() {
    unsafe {
        uart_write_reg(UART_LCR, LCR_DLAB);
        uart_write_reg(UART_THR, 0x01);
        uart_write_reg(UART_IER, 0x00);
        uart_write_reg(UART_LCR, LCR_8N1);
        uart_write_reg(UART_FCR, 0x07);
    }
}

fn uart_write_byte(b: u8) {
    unsafe {
        while (uart_read_reg(UART_LSR) & LSR_THRE) == 0 {}
        uart_write_reg(UART_THR, b);
    }
}

fn uart_write_str(s: &str) {
    for &b in s.as_bytes() {
        uart_write_byte(b);
    }
}

#[entry]
fn main() -> ! {
    uart_init();
    uart_write_str("Hello from UART!\n");
    loop {}
}
