//! Minimal startup / runtime for RISCV CPU's
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
//!
//! extern crate riscv_rt;
//!
//! fn main() {
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
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(extern_prelude)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(panic_implementation)]
#![feature(used)]

extern crate riscv;
extern crate r0;

mod lang_items;

use riscv::asm;
use riscv::register::{mcause, mstatus};

extern "C" {
    // NOTE `rustc` forces this signature on us. See `src/lang_items.rs`
    fn main(argc: isize, argv: *const *const u8) -> isize;

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


/// Entry point of all programs (_start).
///
/// It initializes DWARF call frame information, the stack pointer, the
/// frame pointer (needed for closures to work in start_rust) and the global
/// pointer. Then it calls _start_rust.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
global_asm!(r#"
.section .init
.globl _start
_start:
  .cfi_startproc
  .cfi_undefined ra

  // .option push
  // .option norelax
  lui gp, %hi(__global_pointer$)
  addi gp, gp, %lo(__global_pointer$)
  // .option pop

  lui sp, %hi(_stack_start)
  addi sp, sp, %lo(_stack_start)

  add s0, sp, zero

  jal zero, _start_rust

  .cfi_endproc
"#);


/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[naked]
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub extern "C" fn start_rust() -> ! {
    unsafe {
        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    // TODO: Enable FPU when available

    // Set mtvec to _start_trap
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        //mtvec::write(_start_trap as usize, mtvec::TrapMode::Direct);
        asm!("csrrw zero, 0x305, $0"
             :
             : "r"(&_start_trap)
             :
             : "volatile");
    }

    // Neither `argc` or `argv` make sense in bare metal context so we
    // just stub them
    unsafe {
        main(0, ::core::ptr::null());
    }

    // If `main` returns, then we go into "reactive" mode and simply attend
    // interrupts as they occur.
    loop {
        asm::wfi();
    }
}


/// Trap entry point (_start_trap)
///
/// Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
/// restores caller saved registers and then returns.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
global_asm!(r#"
  .section .trap
  .align 4
  .global _start_trap

_start_trap:
  addi sp, sp, -16*4

  sw ra, 0*4(sp)
  sw t0, 1*4(sp)
  sw t1, 2*4(sp)
  sw t2, 3*4(sp)
  sw t3, 4*4(sp)
  sw t4, 5*4(sp)
  sw t5, 6*4(sp)
  sw t6, 7*4(sp)
  sw a0, 8*4(sp)
  sw a1, 9*4(sp)
  sw a2, 10*4(sp)
  sw a3, 11*4(sp)
  sw a4, 12*4(sp)
  sw a5, 13*4(sp)
  sw a6, 14*4(sp)
  sw a7, 15*4(sp)

  jal ra, _start_trap_rust

  lw ra, 0*4(sp)
  lw t0, 1*4(sp)
  lw t1, 2*4(sp)
  lw t2, 3*4(sp)
  lw t3, 4*4(sp)
  lw t4, 5*4(sp)
  lw t5, 6*4(sp)
  lw t6, 7*4(sp)
  lw a0, 8*4(sp)
  lw a1, 9*4(sp)
  lw a2, 10*4(sp)
  lw a3, 11*4(sp)
  lw a4, 12*4(sp)
  lw a5, 13*4(sp)
  lw a6, 14*4(sp)
  lw a7, 15*4(sp)

  addi sp, sp, 16*4
  mret
"#);


/// Trap entry point rust (_start_trap_rust)
///
/// mcause is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is converted to an element
/// of the Interrupt or Exception enum and passed to handle_interrupt or
/// handle_exception.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust() {
    // dispatch trap to handler
    trap_handler(mcause::read().cause());
    // mstatus, remain in M-mode after mret
    unsafe {
        mstatus::set_mpp(mstatus::MPP::Machine);
    }
}


/// Default Trap Handler
#[no_mangle]
#[linkage = "weak"]
pub fn trap_handler(_: mcause::Trap) {}

// Make sure there is an abort when linking
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
global_asm!(r#"
.section .init
.globl abort
abort:
  jal zero, _start
"#);
