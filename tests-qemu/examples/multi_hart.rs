//! Multi-hart example demonstrating IPI-based hart synchronization.
//!
//! Hart 0 initializes UART and wakes Hart 1 via software interrupt (CLINT).
//! Both harts print messages and synchronize before exit.

#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use riscv_rt::entry;
use riscv_semihosting::debug::{self, EXIT_SUCCESS};

const UART_BASE: usize = 0x1000_0000;
const UART_THR: usize = UART_BASE;
const UART_LCR: usize = UART_BASE + 3;
const UART_LSR: usize = UART_BASE + 5;
const LCR_DLAB: u8 = 1 << 7;
const LCR_8N1: u8 = 0x03;
const LSR_THRE: u8 = 1 << 5;

static UART_LOCK: AtomicBool = AtomicBool::new(false);
static HART1_DONE: AtomicBool = AtomicBool::new(false);

fn uart_lock() {
    while UART_LOCK
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        core::hint::spin_loop();
    }
}

fn uart_unlock() {
    UART_LOCK.store(false, Ordering::Release);
}

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
        uart_write_reg(UART_BASE + 1, 0x00);
        uart_write_reg(UART_LCR, LCR_8N1);
        uart_write_reg(UART_BASE + 2, 0x07);
    }
}

fn uart_write_byte(b: u8) {
    unsafe {
        while (uart_read_reg(UART_LSR) & LSR_THRE) == 0 {}
        uart_write_reg(UART_THR, b);
    }
}

fn uart_print(s: &str) {
    uart_lock();
    for &b in s.as_bytes() {
        uart_write_byte(b);
    }
    uart_unlock();
}

// Custom _mp_hook implementation in assembly
// Hart 0 returns 1 (true) to initialize RAM
// Hart 1 polls for IPI via CLINT, then returns 0 (false) to skip RAM init
global_asm!(
    r#"
.section .init.mp_hook, "ax"
.global _mp_hook
_mp_hook:
    beqz a0, 2f         // if hart 0, return true

    // Hart 1: Poll for IPI (no interrupts, just polling)
    // Clear any pending software interrupt first
    li t0, 0x02000004   // CLINT msip address for hart 1
    sw zero, 0(t0)

1:  // Poll mip register for software interrupt pending
    csrr t0, mip
    andi t0, t0, 8      // Check MSIP bit
    beqz t0, 1b         // If not set, keep polling

    // Clear the software interrupt
    li t0, 0x02000004
    sw zero, 0(t0)

    // Return false (0) - don't initialize RAM again
    li a0, 0
    ret

2:  // Hart 0: return true to initialize RAM
    li a0, 1
    ret
"#
);

#[entry]
fn main(hartid: usize) -> ! {
    if hartid == 0 {
        uart_init();
        uart_print("Hart 0: Initializing\n");

        // Send IPI to Hart 1 (write to CLINT msip register for hart 1)
        unsafe {
            (0x02000004usize as *mut u32).write_volatile(1);
        }

        while !HART1_DONE.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }

        uart_print("Hart 0: Both harts done\n");
        debug::exit(EXIT_SUCCESS);
    } else {
        // Hart 1 reaches here after _mp_hook detects IPI
        uart_print("Hart 1: Running\n");
        HART1_DONE.store(true, Ordering::Release);
    }

    loop {
        core::hint::spin_loop();
    }
}
