//! Semihosting for ARM Cortex-M processors
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on an ARM target to
//!  communicate and use the Input/Output facilities on a host computer that is
//!  running a debugger." - ARM
//!
//! # Interface
//!
//! Since semihosting operations are modeled as [system calls][sc], this crate
//! exposes an untyped `syscall!` interface just like the [`sc`] crate does.
//!
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//!
//! And since the most used semihosting operation is writing to the host's
//! stdout, convenience `hprint` and `hprintln` macros are provided.
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
//! fn main() {
//!     // File descriptor (on the host)
//!     const STDOUT: usize = 1;
//!     static MSG: &'static [u8] = b"Hello, world!\n";
//!
//!     // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
//!     let r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
//!
//!     // (Or you could have just written `hprintln!("Hello, world!")`)
//!     // ..
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
//! those logs in "real time" using `watch` + `tail`
//!
//! ``` text
//! $ watch 'tail /tmp/openocd.log'
//! Every 2.0s: tail /tmp/openocd.log
//!
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : clock speed 950 kHz
//! Info : STLINK v1 JTAG v11 API v2 SWIM v0 VID 0x0483 PID 0x3744
//! Info : using stlink api v2
//! Info : nrf51.cpu: hardware has 4 breakpoints, 2 watchpoints
//! ```
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

#![feature(asm)]
#![no_std]

#[macro_use]
mod macros;

pub mod io;
pub mod nr;

/// Performs a semihosting operation
#[inline(always)]
#[cfg(target_arch = "arm")]
pub unsafe fn syscall<T>(mut nr: usize, arg: &T) -> usize {
    asm!("bkpt 0xAB"
         : "+{r0}"(nr)
         : "{r1}"(arg)
         : "memory"
         : "volatile");
    nr
}

#[cfg(not(target_arch = "arm"))]
pub unsafe fn syscall<T>(_nr: usize, _arg: &T) -> usize {
    0
}
