#![no_std]
#![no_main]

extern crate panic_halt;
extern crate riscv_rt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // do something here
    loop {}
}
