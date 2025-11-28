//! Semihosting example for QEMU
//!
//! This example uses RISC-V semihosting to print output and cleanly exit QEMU.
//! Run with: `qemu-system-riscv32 -machine virt -nographic -semihosting-config enable=on,target=native -bios none -kernel <binary>`

#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use riscv_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprintln,
};

#[entry]
fn main() -> ! {
    hprintln!("Hello from semihosting!");
    debug::exit(EXIT_SUCCESS);
    loop {}
}
