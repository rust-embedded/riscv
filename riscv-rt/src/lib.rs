//! Minimal startup / runtime for RISC-V CPU's
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - Before main initialization of the FPU (for targets that have a FPU).
//!
//! - A `panic_fmt` implementation that just calls abort that you can opt into
//!   through the "abort-on-panic" Cargo feature. If you don't use this feature
//!   you'll have to provide the `panic_fmt` lang item yourself. Documentation
//!   [here][1]
//!
//! [1]: https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
//!
//! - A minimal `start` lang item to support the standard `fn main()`
//!   interface. (The processor goes to sleep after returning from `main`)
//!
//! - A linker script that encodes the memory layout of a generic RISC-V
//!   microcontroller. This linker script is missing some information that must
//!   be supplied through a `memory.x` file (see example below).
//!
//! - A `_sheap` symbol at whose address you can locate a heap.
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ # add this crate as a dependency
//! $ edit Cargo.toml && cat $_
//! [dependencies.riscv-rt]
//! version = "0.1.0"
//!
//! $ # tell Xargo which standard crates to build
//! $ edit Xargo.toml && cat $_
//! [dependencies.core]
//! stage = 0
//!
//! [dependencies.compiler_builtins]
//! features = ["mem"]
//! stage = 1
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 128K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 8K
//! }
//!
//! $ edit src/main.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! #![no_std]
//! #![no_main]
//!
//! #[macro_use(entry)]
//! extern crate riscv_rt;
//!
//! // use `main` as the entry point of this application
//! entry!(main);
//!
//! fn main() -> ! {
//!     // do something here
//! }
//! ```
//!
//! ``` text
//! $ cargo install xargo
//!
//! $ xargo rustc --target riscv32-unknown-none -- \
//!    -C link-arg=-Tlink.x -C linker=riscv32-unknown-elf-ld -Z linker-flavor=ld
//!
//! $ riscv32-unknown-elf-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 20400000 <_start>:
//! 20400000:	800011b7    lui	gp,0x80001
//! 20400004:	80018193    addi	gp,gp,-2048 # 80000800 <_stack_start+0xffffc800>
//! 20400008:	80004137    lui	sp,0x80004
//! ```
//!
//! # Symbol interfaces
//!
//! This crate makes heavy use of symbols, linker sections and linker scripts to
//! provide most of its functionality. Below are described the main symbol
//! interfaces.
//!
//! ## `memory.x`
//!
//! This file supplies the information about the device to the linker.
//!
//! ### `MEMORY`
//!
//! The main information that this file must provide is the memory layout of
//! the device in the form of the `MEMORY` command. The command is documented
//! [here][2], but at a minimum you'll want to create two memory regions: one
//! for Flash memory and another for RAM.
//!
//! [2]: https://sourceware.org/binutils/docs/ld/MEMORY.html
//!
//! The program instructions (the `.text` section) will be stored in the memory
//! region named FLASH, and the program `static` variables (the sections `.bss`
//! and `.data`) will be allocated in the memory region named RAM.
//!
//! ### `_stack_start`
//!
//! This symbol provides the address at which the call stack will be allocated.
//! The call stack grows downwards so this address is usually set to the highest
//! valid RAM address plus one (this *is* an invalid address but the processor
//! will decrement the stack pointer *before* using its value as an address).
//!
//! If omitted this symbol value will default to `ORIGIN(RAM) + LENGTH(RAM)`.
//!
//! #### Example
//!
//! Allocating the call stack on a different RAM region.
//!
//! ```
//! MEMORY
//! {
//!   /* call stack will go here */
//!   CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!   /* static variables will go here */
//!   RAM : ORIGIN = 0x20000000, LENGTH = 40K
//! }
//!
//! _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);
//! ```
//!
//! ### `_sheap`
//!
//! This symbol is located in RAM right after the `.bss` and `.data` sections.
//! You can use the address of this symbol as the start address of a heap
//! region. This symbol is 4 byte aligned so that address will be a multiple of
//! 4.
//!
//! #### Example
//!
//! ```
//! extern crate some_allocator;
//!
//! // Size of the heap in bytes
//! const SIZE: usize = 1024;
//!
//! extern "C" {
//!     static mut _sheap: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let start_address = &mut _sheap as *mut u8;
//!         some_allocator::initialize(start_address, SIZE);
//!     }
//! }
//! ```

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate riscv;
extern crate r0;

use riscv::register::{mstatus, mtvec};

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;

    // Address of _start_trap
    static _start_trap: u32;
}


/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub extern "C" fn start_rust() -> ! {
    extern "C" {
        // This symbol will be provided by the user via the `entry!` macro
        fn main() -> !;
    }

    unsafe {
        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    // TODO: Enable FPU when available

    unsafe {
        // Set mtvec to _start_trap
        mtvec::write(_start_trap as usize, mtvec::TrapMode::Direct);

        main();
    }
}


/// Macro to define the entry point of the program
///
/// **NOTE** This macro must be invoked once and must be invoked from an accessible module, ideally
/// from the root of the crate.
///
/// Usage: `entry!(path::to::entry::point)`
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
///
/// The signature of the specified function must be `fn() -> !` (never ending function).
#[macro_export]
macro_rules! entry {
    ($path:expr) => {
        #[inline(never)]
        #[export_name = "main"]
        pub extern "C" fn __impl_main() -> ! {
            // validate the signature of the program entry point
            let f: fn() -> ! = $path;

            f()
        }
    };
}


/// Trap entry point rust (_start_trap_rust)
///
/// mcause is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is converted to an element
/// of the Interrupt or Exception enum and passed to handle_interrupt or
/// handle_exception.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust() {
    extern "C" {
        fn trap_handler();
    }

    unsafe {
        // dispatch trap to handler
        trap_handler();

        // mstatus, remain in M-mode after mret
        mstatus::set_mpp(mstatus::MPP::Machine);
    }
}


/// Default Trap Handler
#[no_mangle]
pub fn default_trap_handler() {}
