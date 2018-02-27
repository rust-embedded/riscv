//! Semihosting for RISCV processors
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on a RISCV target to
//!  communicate and use the Input/Output facilities on a host computer that is
//!  running a debugger." - RISCV
//!
//! # Interface
//!
//! Since semihosting operations are modeled as [system calls][sc], this crate
//! exposes an untyped `syscall!` interface just like the [`sc`] crate does.
//!
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//!
//! # Forewarning
//!
//! Semihosting operations are *very* slow. Like, each WRITE operation can take
//! hundreds of milliseconds.
//!
//! # Example
//!
//! This example will show how to print "Hello, world!" on the host.
//!
//! Target program:
//!
//! ```
//! #[macro_use]
//! extern crate riscv_semihosting;
//!
//! fn main() {
//!     // File descriptor (on the host)
//!     const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
//!     static MSG: &'static [u8] = b"Hello, world!\n";
//!
//!     // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
//!     let r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
//! }
//! ```
//!
//! On the host side:
//!
//! ``` text
//! $ openocd -f $INTERFACE -f $TARGET -l /tmp/openocd.log
//! Open On-Chip Debugger 0.9.0 (2016-04-27-23:18)
//! Licensed under GNU GPL v2
//! For bug reports, read
//!         http://openocd.org/doc/doxygen/bugs.html
//! # the command will block at this point
//! ```
//!
//! The OpenOCD logs will be redirected to `/tmp/openocd.log`. You can view
//! those logs in "real time" using `tail`
//!
//! ``` text
//! $ tail -f /tmp/openocd.log
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : clock speed 950 kHz
//! Info : STLINK v1 JTAG v11 API v2 SWIM v0 VID 0x0483 PID 0x3744
//! Info : using stlink api v2
//! Info : nrf51.cpu: hardware has 4 breakpoints, 2 watchpoints
//! ```
//!
//! Alternatively you could omit the `-l` flag from the `openocd` call, and the
//! `tail -f` command but the OpenOCD output will have intermingled in it logs
//! from its normal operation.
//!
//! Then, we run the program:
//!
//! ``` text
//! $ arm-none-eabi-gdb hello-world
//! # Connect to OpenOCD
//! (gdb) target remote :3333
//!
//! # Enable OpenOCD's semihosting support
//! (gdb) monitor arm semihosting enable
//!
//! # Flash the program
//! (gdb) load
//!
//! # Run the program
//! (gdb) continue
//! ```
//!
//! And you'll see the output under OpenOCD's terminal
//!
//! ``` text
//! # openocd -f $INTERFACE -f $TARGET -l /tmp/openocd.log
//! (..)
//! Hello, world!
//! ```
//!
//! # Reference
//!
//! For documentation about the semihosting operations, check:
//!
//! 'Chapter 8 - Semihosting' of the ['ARM Compiler toolchain Version 5.0'][pdf]
//! manual.
//!
//! [pdf]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0471e/DUI0471E_developing_for_arm_processors.pdf

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![no_std]

#[macro_use]
mod macros;

pub mod debug;
pub mod hio;
pub mod nr;

/// The hint that differentiates the semihosting call.
// WARNING: This variable is hardcoded in the asm! Don't forget to update
// if it changes.
pub const RISCV_SEMIHOSTING_CALL_NUMBER: usize = 7;

/// Performs a semihosting operation, takes a pointer to an argument block
#[inline]
#[cfg(target_arch = "riscv")]
pub unsafe fn syscall<T>(mut nr: usize, arg: &T) -> usize {
    // .option push
    // .option norvc
    asm!(r"
      slli x0, x0, 0x1f
      ebreak
      srai x0, x0, 0x7
    "
         : "+{x10}"(nr)
         : "{x11}"(arg)
         : "memory"
         : "volatile");
    // .option pop
    nr
}

/// Performs a semihosting operation, takes a pointer to an argument block
#[cfg(not(target_arch = "riscv"))]
pub unsafe fn syscall<T>(_nr: usize, _arg: &T) -> usize {
    0
}

/// Performs a semihosting operation, takes one integer as an argument
#[inline]
#[cfg(target_arch = "riscv")]
pub unsafe fn syscall1(mut nr: usize, arg: usize) -> usize {
    // .option push
    // .option norvc
    asm!(r"
      slli x0, x0, 0x1f
      ebreak
      srai x0, x0, 0x7
    "
         : "+{x10}"(nr)
         : "{x11}"(arg)
         : "memory"
         : "volatile");
    // .option pop
    nr
}

/// Performs a semihosting operation, takes one integer as an argument
#[cfg(not(target_arch = "riscv"))]
pub unsafe fn syscall1(_nr: usize, _arg: usize) -> usize {
    0
}
