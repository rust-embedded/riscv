//! Minimal startup / runtime for RISC-V CPU's
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.61 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - `#[entry]` to declare the entry point of the program
//! - `#[pre_init]` to run code *before* `static` variables are initialized
//!
//! - A linker script that encodes the memory layout of a generic RISC-V
//!   microcontroller. This linker script is missing some information that must
//!   be supplied through a `memory.x` file (see example below). This file
//!   must be supplied using rustflags and listed *before* `link.x`. Arbitrary
//!   filename can be use instead of `memory.x`.
//!
//! - A `_sheap` symbol at whose address you can locate a heap.
//!
//! - Support for a runtime in supervisor mode, that can be bootstrapped via
//!   [Supervisor Binary Interface (SBI)](https://github.com/riscv-non-isa/riscv-sbi-doc).
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ # add this crate as a dependency
//! $ edit Cargo.toml && cat $_
//! [dependencies]
//! riscv-rt = "0.13.0"
//! panic-halt = "0.2.0"
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//!
//! REGION_ALIAS("REGION_TEXT", FLASH);
//! REGION_ALIAS("REGION_RODATA", FLASH);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", RAM);
//!
//! $ edit src/main.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! #![no_std]
//! #![no_main]
//!
//! extern crate panic_halt;
//!
//! use riscv_rt::entry;
//!
//! // use `main` as the entry point of this application
//! // `main` is not allowed to return
//! #[entry]
//! fn main() -> ! {
//!     // do something here
//!     loop { }
//! }
//! ```
//!
//! ``` text
//! $ mkdir .cargo && edit .cargo/config && cat $_
//! [target.riscv32imac-unknown-none-elf]
//! rustflags = [
//!   "-C", "link-arg=-Tmemory.x",
//!   "-C", "link-arg=-Tlink.x",
//! ]
//!
//! [build]
//! target = "riscv32imac-unknown-none-elf"
//! $ edit build.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! use std::env;
//! use std::fs;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//!
//!     // Put the linker script somewhere the linker can find it.
//!     fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
//!     println!("cargo:rustc-link-search={}", out_dir.display());
//!     println!("cargo:rerun-if-changed=memory.x");
//!
//!     println!("cargo:rerun-if-changed=build.rs");
//! }
//! ```
//!
//! ``` text
//! $ cargo build
//!
//! $ riscv32-unknown-elf-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 20000000 <_start>:
//! 20000000:    800011b7        lui     gp,0x80001
//! 20000004:    80018193        addi    gp,gp,-2048 # 80000800 <_stack_start+0xffffc800>
//! 20000008:    80004137        lui     sp,0x80004
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
//! [here][2], but at a minimum you'll want to create at least one memory region.
//!
//! [2]: https://sourceware.org/binutils/docs/ld/MEMORY.html
//!
//! To support different relocation models (RAM-only, FLASH+RAM) multiple regions are used:
//!
//! - `REGION_TEXT` - for `.init`, `.trap` and `.text` sections
//! - `REGION_RODATA` - for `.rodata` section and storing initial values for `.data` section
//! - `REGION_DATA` - for `.data` section
//! - `REGION_BSS` - for `.bss` section
//! - `REGION_HEAP` - for the heap area
//! - `REGION_STACK` - for hart stacks
//!
//! Specific aliases for these regions must be defined in `memory.x` file (see example below).
//!
//! ### `_stext`
//!
//! This symbol provides the loading address of `.text` section. This value can be changed
//! to override the loading address of the firmware (for example, in case of bootloader present).
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_TEXT)`.
//!
//! ### `_stack_start`
//!
//! This symbol provides the address at which the call stack will be allocated.
//! The call stack grows downwards so this address is usually set to the highest
//! valid RAM address plus one (this *is* an invalid address but the processor
//! will decrement the stack pointer *before* using its value as an address).
//!
//! In case of multiple harts present, this address defines the initial stack pointer for hart 0.
//! Stack pointer for hart `N` is calculated as  `_stack_start - N * _hart_stack_size`.
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_STACK) + LENGTH(REGION_STACK)`.
//!
//! #### Example
//!
//! Allocating the call stack on a different RAM region.
//!
//! ``` text
//! MEMORY
//! {
//!   L2_LIM : ORIGIN = 0x08000000, LENGTH = 1M
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//!
//! REGION_ALIAS("REGION_TEXT", FLASH);
//! REGION_ALIAS("REGION_RODATA", FLASH);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", L2_LIM);
//!
//! _stack_start = ORIGIN(L2_LIM) + LENGTH(L2_LIM);
//! ```
//!
//! ### `_max_hart_id`
//!
//! This symbol defines the maximum hart id supported. All harts with id
//! greater than `_max_hart_id` will be redirected to `abort()`.
//!
//! This symbol is supposed to be redefined in platform support crates for
//! multi-core targets.
//!
//! If omitted this symbol value will default to 0 (single core).
//!
//! ### `_hart_stack_size`
//!
//! This symbol defines stack area size for *one* hart.
//!
//! If omitted this symbol value will default to 2K.
//!
//! ### `_heap_size`
//!
//! This symbol provides the size of a heap region. The default value is 0. You can set `_heap_size`
//! to a non-zero value if you are planning to use heap allocations.
//!
//! ### `_sheap`
//!
//! This symbol is located in RAM right after the `.bss` and `.data` sections.
//! You can use the address of this symbol as the start address of a heap
//! region. This symbol is 4 byte aligned so that address will be a multiple of 4.
//!
//! #### Example
//!
//! ``` no_run
//! extern crate some_allocator;
//!
//! extern "C" {
//!     static _sheap: u8;
//!     static _heap_size: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let heap_bottom = &_sheap as *const u8 as usize;
//!         let heap_size = &_heap_size as *const u8 as usize;
//!         some_allocator::initialize(heap_bottom, heap_size);
//!     }
//! }
//! ```
//!
//! ## `_pre_init_trap`
//!
//! This function is set as a provisional trap handler for the early trap handling.
//! If either an exception or an interrupt occurs during the boot process, this
//! function is triggered. The default implementation of this function is a busy-loop.
//! While this function can be redefined, it is not recommended to do so, as it is
//! intended to be a temporary trap handler to detect bugs in the early boot process.
//! Recall that this trap is triggered before the `.bss` and `.data` sections are
//! initialized, so it is not safe to use any global variables in this function.
//!
//! ### `_mp_hook`
//!
//! This function is called from all the harts and must return true only for one hart,
//! which will perform memory initialization. For other harts it must return false
//! and implement wake-up in platform-dependent way (e.g. after waiting for a user interrupt).
//! The parameter `hartid` specifies the hartid of the caller.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "_mp_hook"]
//! pub extern "Rust" fn mp_hook(hartid: usize) -> bool {
//!    // ...
//! }
//! ```
//!
//! Default implementation of this function wakes hart 0 and busy-loops all the other harts.
//!
//! `_mp_hook` is only necessary in multi-core targets. If the `single-hart` feature is enabled,
//! `_mp_hook` is not included in the binary.
//!
//! ### `_setup_interrupts`
//!
//! This function is called right before the main function and is responsible for setting up
//! the interrupt controller.
//!
//! Default implementation sets the trap vector to `_start_trap` in direct mode.
//! Users can override this function by defining their own `_setup_interrupts`
//!
//! ### Core exception handlers
//!
//! This functions are called when corresponding exception occurs.
//! You can define an exception handler with the [`exception`] attribute.
//! The attribute expects the path to the exception source as an argument.
//!
//! The [`exception`] attribute ensures at compile time that there is a valid
//! exception source for the given handler.
//!
//! For example:
//! ``` no_run
//! use riscv::interrupt::Exception; // or a target-specific exception enum
//!
//! #[riscv_rt::exception(Exception::MachineEnvCall)]
//! fn custom_menv_call_handler(trap_frame: &mut riscv_rt::TrapFrame) {
//!     todo!()
//! }
//!
//! #[riscv_rt::exception(Exception::LoadFault)]
//! fn custom_load_fault_handler() -> ! {
//!     loop {}
//! }
//! ```
//!
//! If exception handler is not explicitly defined, `ExceptionHandler` is called.
//!
//! ### `ExceptionHandler`
//!
//! This function is called when exception without defined exception handler is occured.
//! The exception reason can be decoded from the
//! `mcause`/`scause` register.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "ExceptionHandler"]
//! fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn ExceptionHandler(trap_frame: &mut riscv_rt::TrapFrame) {
//!     // ...
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.
//!
//! ### Core interrupt handlers
//!
//! This functions are called when corresponding interrupt is occured.
//! You can define a core interrupt handler with the [`core_interrupt`] attribute.
//! The attribute expects the path to the interrupt source as an argument.
//!
//! The [`core_interrupt`] attribute ensures at compile time that there is a valid
//! core interrupt source for the given handler.
//!
//! For example:
//! ``` no_run
//! use riscv::interrupt::Interrupt; // or a target-specific core interrupt enum
//!
//! #[riscv_rt::core_interrupt(Interrupt::MachineSoft)]
//! unsafe fn custom_machine_soft_handler() {
//!     todo!()
//! }
//!
//! #[riscv_rt::core_interrupt(Interrupt::MachineTimer)]
//! fn custom_machine_timer_handler() -> ! {
//!     loop {}
//! }
//! ```
//!
//! In vectored mode, this macro will also generate a proper trap handler for the interrupt.
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ### External interrupt handlers
//!
//! This functions are called when corresponding interrupt is occured.
//! You can define an external interrupt handler with the [`external_interrupt`] attribute.
//! The attribute expects the path to the interrupt source as an argument.
//!
//! The [`external_interrupt`] attribute ensures at compile time that there is a valid
//! external interrupt source for the given handler.
//! Note that external interrupts are target-specific and may not be available on all platforms.
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ### `DefaultHandler`
//!
//! This function is called when interrupt without defined interrupt handler is occured.
//! The interrupt reason can be decoded from the `mcause`/`scause` register.
//! If it is an external interrupt, the interrupt reason can be decoded from a
//! target-specific peripheral interrupt controller.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "DefaultHandler"]
//! unsafe fn custom_interrupt_handler() {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn DefaultHandler() -> ! {
//!     loop {}
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.
//!
//! # Cargo Features
//!
//! ## `single-hart`
//!
//! This feature saves a little code size if there is only one hart on the target.
//! If the `single-hart` feature is enabled, `_mp_hook` is not called.
//!
//! ## `s-mode`
//!
//! The supervisor mode feature (`s-mode`) can be activated via [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! For example:
//! ``` text
//! [dependencies]
//! riscv-rt = {features=["s-mode"]}
//! ```
//! While most registers/instructions have variants for
//! both `mcause` and `scause`, the `mhartid` hardware thread register is not available in supervisor
//! mode. Instead, the hartid is passed as parameter by a bootstrapping firmware (i.e., SBI).
//!
//! Use case: QEMU supports [OpenSBI](https://github.com/riscv-software-src/opensbi) as default firmware.
//! Using the SBI requires riscv-rt to be run in supervisor mode instead of machine mode.
//! ``` text
//! APP_BINARY=$(find target -name app)
//! sudo qemu-system-riscv64 -m 2G -nographic -machine virt -kernel $APP_BINARY
//! ```
//! It requires the memory layout to be non-overlapping, like
//! ``` text
//! MEMORY
//! {
//!   RAM : ORIGIN = 0x80200000, LENGTH = 0x8000000
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//! ```
//!
//! ## `v-trap`
//!
//! The vectored trap feature (`v-trap`) can be activated via [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! For example:
//! ``` text
//! [dependencies]
//! riscv-rt = {features=["v-trap"]}
//! ```
//! When the vectored trap feature is enabled, the trap vector is set to `_vector_table` in vectored mode.
//! This table is a list of `j _start_INTERRUPT_trap` instructions, where `INTERRUPT` is the name of the core interrupt.
//!
//! ## `u-boot`
//!
//! The u-boot support feature (`u-boot`) can be activated via [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! For example:
//! ``` text
//! [dependencies]
//! riscv-rt = { features = ["u-boot"] }
//! ```
//! When the u-boot feature is enabled, acceptable signature for `#[entry]` macros is changed. This is required
//! because when booting from elf, u-boot passes `argc` and `argv`. This feature also implies `single-hart`.
//! The only way to get boot-hart is through fdt, so other harts initialization is up to you.

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]

#[cfg(riscv)]
mod asm;

#[cfg(not(feature = "no-exceptions"))]
pub mod exceptions;

#[cfg(not(feature = "no-interrupts"))]
pub mod interrupts;

#[cfg(feature = "s-mode")]
use riscv::register::scause as xcause;

#[cfg(not(feature = "s-mode"))]
use riscv::register::mcause as xcause;

pub use riscv_rt_macros::{entry, exception, external_interrupt, pre_init};

pub use riscv_pac::*;

#[cfg(riscv32)]
pub use riscv_rt_macros::core_interrupt_riscv32 as core_interrupt;
#[cfg(riscv64)]
pub use riscv_rt_macros::core_interrupt_riscv64 as core_interrupt;
#[cfg(not(riscv))]
pub use riscv_rt_macros::core_interrupt_riscv64 as core_interrupt; // just for docs, tests, etc.

/// We export this static with an informative name so that if an application attempts to link
/// two copies of riscv-rt together, linking will fail. We also declare a links key in
/// Cargo.toml which is the more modern way to solve the same problem, but we have to keep
/// __ONCE__ around to prevent linking with versions before the links key was added.
#[export_name = "error: riscv-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

/// Registers saved in trap handler
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

/// Trap entry point rust (_start_trap_rust)
///
/// `scause`/`mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and one of the
/// exception handlers or one of the core interrupt handlers is called.
///
/// # Note
///
/// Exception dispatching is performed by an extern `_dispatch_exception` function.
/// Targets that comply with the RISC-V standard can use the implementation provided
/// by this crate in the [`exceptions`] module. Targets with special exception sources
/// may provide their custom implementation of the `_dispatch_exception` function. You may
/// also need to enable the `no-exceptions` feature to op-out the default implementation.
///
/// In direct mode (i.e., `v-trap` feature disabled), interrupt dispatching is performed
/// by an extern `_dispatch_core_interrupt` function. Targets that comply with the RISC-V
/// standard can use the implementation provided by this crate in the [`interrupts`] module.
/// Targets with special interrupt sources may provide their custom implementation of the
/// `_dispatch_core_interrupt` function. You may also need to enable the `no-interrupts`
/// feature to op-out the default implementation.
///
/// In vectored mode (i.e., `v-trap` feature enabled), interrupt dispatching is performed
/// directly by hardware, and thus this function should **not** be triggered due to an
/// interrupt. If this abnormal situation happens, this function will directly call the
/// `DefaultHandler` function.
///
/// # Safety
///
/// This function must be called only from assembly `_start_trap` function.
/// Do **NOT** call this function directly.
#[cfg_attr(riscv, link_section = ".trap.rust")]
#[export_name = "_start_trap_rust"]
pub unsafe extern "C" fn start_trap_rust(trap_frame: *const TrapFrame) {
    extern "C" {
        #[cfg(not(feature = "v-trap"))]
        fn _dispatch_core_interrupt(code: usize);
        #[cfg(feature = "v-trap")]
        fn DefaultHandler();
        fn _dispatch_exception(trap_frame: &TrapFrame, code: usize);
    }

    match xcause::read().cause() {
        #[cfg(not(feature = "v-trap"))]
        xcause::Trap::Interrupt(code) => _dispatch_core_interrupt(code),
        #[cfg(feature = "v-trap")]
        xcause::Trap::Interrupt(_) => DefaultHandler(),
        xcause::Trap::Exception(code) => _dispatch_exception(&*trap_frame, code),
    }
}
