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
    hprintln!("HELLO_QEMU");
    debug::exit(EXIT_SUCCESS);
    loop {}
}
