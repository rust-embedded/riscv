//! Startup code and minimal runtime for RISC-V CPUs
//!
//! This crate contains all the required parts to build a `no_std` application
//! (binary crate) that targets a RISC-V microcontroller.
//!
//! # Features
//!
//! This crate takes care of:
//!
//! - The memory layout of the program.
//!
//! - Initializing `static` variables before the program entry point.
//!
//! - Enabling the FPU before the program entry point if the target has the `f` or `d` extension.
//!
//! - Support for a runtime in supervisor mode, that can be bootstrapped via
//!   [Supervisor Binary Interface (SBI)](https://github.com/riscv-non-isa/riscv-sbi-doc).
//!
//! - Support for bootstrapping a runtime with [U-Boot](https://github.com/u-boot/u-boot).
//!
//! This crate also provides the following attributes:
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - [`#[entry]`][attr-entry] to declare the entry point of the program
//! - [`#[exception]`][attr-exception] to override an exception handler.
//! - [`#[core_interrupt]`][attr-core-interrupt] to override a core interrupt handler.
//! - [`#[external_interrupt]`][attr-external-interrupt] to override an external interrupt handler.
//!
//! If not overridden, all exception and interrupt handlers default to an infinite loop.
//!
//! The documentation for these attributes can be found in the [Attribute Macros](#attributes)
//! section.
//!
//! # Requirements
//!
//! ## `memory.x`
//!
//! This crate expects the user, or some other crate, to provide the memory layout of the target
//! device via a linker script, described in this section. We refer to this file as `memory.x`.
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
//! These aliases must be mapped to a valid `MEMORY` region. Usually, `REGION_TEXT` and
//! `REGION_RODATA` are mapped to the flash memory, while `REGION_DATA`, `REGION_BSS`,
//! `REGION_HEAP`, and `REGION_STACK` are mapped to the RAM.
//!
//! ### `_stext`
//!
//! This symbol provides the loading address of `.text` section. This value can be changed
//! to override the loading address of the firmware (for example, in case of bootloader present).
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_TEXT)`.
//!
//! ### `_heap_size`
//!
//! This symbol provides the size of a heap region. The default value is 0. You can set
//! `_heap_size` to a non-zero value if you are planning to use heap allocations.
//!
//!
//! More information about using the heap can be found in the
//! [Using the heap](#using-the-heap) section.
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
//! If omitted this symbol value will default to `SIZEOF(.stack) / (_max_hart_id + 1)`.
//!
//! Note that due to alignment, each individual stack may differ slightly in
//! size.
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
//! ### Example of a fully featured `memory.x` file
//!
//! Next, we present a `memory.x` file that includes all the symbols
//! that can be defined in the file. It also allocates the stack on a different RAM region:
//!
//! ```text
//! /* Fully featured memory.x file */
//! MEMORY
//! {
//!   L2_LIM : ORIGIN = 0x08000000, LENGTH = 1M /* different RAM region for stack */
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
//! _stext = ORIGIN(REGION_TEXT) + 0x400000;        /* Skip first 4M of text region */
//! _heap_size = 1K;                                /* Set heap size to 1KB */
//! _max_hart_id = 1;                               /* Two harts present */
//! _hart_stack_size = 1K;                          /* Set stack size per hart to 1KB */
//! _stack_start = ORIGIN(L2_LIM) + LENGTH(L2_LIM);
//! ```
//!
//! # Starting a minimal application
//!
//! This section presents a minimal application built on top of `riscv-rt`.
//! Let's create a new binary crate:
//!
//! ```text
//! $ cargo new --bin app && cd $_
//! ```
//!
//! Next, we will add a few dependencies to the `Cargo.toml` file:
//!
//! ```toml
//! # in Cargo.toml
//!
//! [dependencies]
//! riscv-rt = "0.13.0"  # <- this crate
//! panic-halt = "1.0.0" # <- a simple panic handler
//! ```
//!
//! Our application would look like this:
//!
//! ```no_run
//! // src/main.rs
//! #![no_main]
//! #![no_std]
//!
//! // make sure the panic handler is linked in
//! extern crate panic_halt;
//!
//! // Use `main` as the entry point of this application, which may not return.
//! #[riscv_rt::entry]
//! fn main() -> ! {
//!     // initialization
//!     loop {
//!         // application logic
//!     }
//! }
//! ```
//!
//! To actually build this program you need to place a `memory.x` linker script
//! somewhere the linker can find it, e.g., in the current directory:
//!
//! ```text
//! /* memory.x */
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
//! ```
//!
//! Feel free to adjust the memory layout to your needs.
//!
//! Next, let's make sure that Cargo uses this linker script by adding a build script:
//!
//! ``` ignore,no_run
//! // build.rs
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
//! In this way, the `memory.x` file will be copied to the build directory so the linker can
//! find it. Also, we tell Cargo to re-run the build script if the `memory.x` file changes.
//!
//! Finally, we can add a `.cargo/config.toml` file to specify the linker script to use, as well
//! as the target to build for when using `cargo build`. In this case, we will build for the
//! `riscv32imac-unknown-none-elf` target:
//!
//! ```toml
//! # .cargo/config.toml
//! [target.riscv32imac-unknown-none-elf]
//! rustflags = [
//!   "-C", "link-arg=-Tmemory.x", # memory.x must appear BEFORE link.x
//!   "-C", "link-arg=-Tlink.x",
//! ]
//!
//! [build]
//! target = "riscv32imac-unknown-none-elf"
//! ```
//!
//! ``` text
//! $ cargo build
//!
//! $ riscv32-unknown-elf-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 20000000 <__stext>:
//! 20000000:       200000b7                lui     ra,0x20000
//! 20000004:       00808067                jr      8(ra) # 20000008 <_abs_start>
//! ```
//!
//! # Using the heap
//!
//! To use the heap, you need to define the `_heap_size` symbol in the `memory.x` file.
//! For instance, we can define a 1 K heap region like this:
//!
//! ``` text
//! /* memory.x */
//!
//! /* ... */
//!
//! _heap_size = 1K;
//! ```
//!
//! The heap region will start right after the `.bss` and `.data` sections.
//!
//! If you plan to use heap allocations, you must include a heap allocator.
//! For example, you can use [`embedded-alloc`](https://github.com/rust-embedded/embedded-alloc).
//! When initializing the heap, you must provide the start address and the size of the heap.
//! You can use the [`heap_start`] function to get the start address of the heap.
//! This symbol is 4 byte aligned so that address will be a multiple of 4.
//!
//! ## Example
//!
//! ``` no_run
//! extern crate some_allocator; // e.g., embedded_alloc::LlffHeap
//!
//! extern "C" {
//!     static _heap_size: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let heap_bottom = riscv_rt::heap_start() as usize;
//!         let heap_size = &_heap_size as *const u8 as usize;
//!         some_allocator::initialize(heap_bottom, heap_size);
//!     }
//! }
//! ```
//!
//! # Additional weak functions
//!
//! This crate uses additional functions to control the behavior of the runtime.
//! These functions are weakly defined in the `riscv-rt` crate, but they can be redefined
//! in the user code. Next, we will describe these symbols and how to redefine them.
//!
//! ## `abort`
//!
//! This function is called when an unrecoverable error occurs. For example, if the
//! current hart id is greater than `_max_hart_id`, the `abort` function is called.
//! This function is also called when an exception or an interrupt occurs and there is no
//! handler for it.
//!
//! If this function is not defined, the linker will use the `_default_abort` function
//! defined in the `riscv-rt` crate. This function is a busy-loop that will never return.
//!
//! ### Note
//!
//! Recall that the `abort` function is called when an unrecoverable error occurs.
//! This function should not be used to handle recoverable errors. Additionally, it may
//! be triggered before the `.bss` and `.data` sections are initialized, so it is not safe
//! to use any global variable in this function.
//!
//! ## `_pre_init_trap`
//!
//! This function is set as a provisional trap handler for the early trap handling.
//! If either an exception or an interrupt occurs during the boot process, this
//! function is triggered.
//!
//! If this function is not defined, the linker will use the `_default_abort` function
//! defined in the `riscv-rt` crate. This function is a busy-loop that will never return.
//!
//! ### Note
//!
//! While this function can be redefined, it is not recommended to do so, as it is
//! intended to be a temporary trap handler to detect bugs in the early boot process.
//! Recall that this trap is triggered before the `.bss` and `.data` sections are
//! initialized, so it is not safe to use any global variables in this function.
//!
//! Furthermore, as this function is expected to behave like a trap handler, it is
//! necessary to make it be 4-byte aligned.
//!
//! ## `_mp_hook` (for multi-core targets only)
//!
//! This function is called from all the harts and must return true only for one hart,
//! which will perform memory initialization. For other harts it must return false
//! and implement wake-up in platform-dependent way (e.g., after waiting for a user interrupt).
//!
//! Default implementation of this function wakes hart 0 and busy-loops all the other harts.
//!
//! ### Note
//!
//! `_mp_hook` is only necessary in multi-core targets. If the `single-hart` feature is enabled,
//! `_mp_hook` is not included in the binary.
//!
//! ### Important implementation guidelines
//!
//! This function is called during the early boot process. Thus, when implementing it, you **MUST** follow these guidelines:
//!
//! - Implement it in assembly (no Rust code is allowed at this point).
//! - Allocate this function within the `.init` section.
//! - You can get the hart id from the `a0` register.
//! - You must set the return value in the `a0` register.
//! - Do **NOT** use callee-saved registers `s0-s2`, as they are used to preserve the initial values of `a0-a2` registers.
//! - In RVE targets, do **NOT** use the `a5` register, as it is used to preserve the `a2` register.
//!
//! **Violating these constraints will result in incorrect arguments being passed to `main()`.**
//!
//! ### Implementation example
//!
//! The following example shows how to implement the `_mp_hook` function in assembly.
//!
//! ``` no_run
//! core::arch::global_asm!(
//!     r#".section .init.mp_hook, "ax"
//!     .global _mp_hook
//! _mp_hook:
//!     beqz a0, 2f // check if hartid is 0
//! 1:  wfi         // If not, wait for interrupt in a loop
//!     j 1b
//! 2:  li a0, 1    // Otherwise, return true
//!     ret
//!     "#
//! );
//! ```
//!
//! ## `_setup_interrupts`
//!
//! This function is called right before the main function and is responsible for setting up
//! the interrupt controller.
//!
//! Default implementation sets the trap vector to `_start_trap` in direct mode.
//! If the `v-trap` feature is enabled, the trap vector is set to `_vector_table`
//! in vectored mode. Users can override this function by defining their own `_setup_interrupts`.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "_setup_interrupts"]
//! pub fn setup_interrupts() {
//!    // ...
//! }
//! ```
//!
//! ## `hal_main`
//!
//! Internally, `riscv-rt` does not jump to the `main` function created by the user using the
//! [`#[entry]`][attr-entry] attribute. Instead, it jumps to the `hal_main` function.
//! The linker will map `hal_main` to `main` if the prior is not defined, which is the typical case.
//! However, the `hal_main` function allows HALs to inject additional code before jumping to the
//! user's `main` function. This might be useful for certain HALs that need to perform additional
//! configuration before the main function is executed.
//!
//! # Attributes
//!
//! ## Core exception handlers
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
//! ## `ExceptionHandler`
//!
//! This function is called when exception without defined exception handler is occurred.
//! The exception reason can be decoded from the `mcause`/`scause` register.
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
//! If `ExceptionHandler` is not defined, the linker will use the `abort` function instead.
//!
//! ## Core interrupt handlers
//!
//! This functions are called when corresponding interrupt is occurred.
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
//! For example, `MachineSoft` interrupt will generate a `_start_MachineSoft_trap` trap handler.
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ## External interrupt handlers
//!
//! This functions are called when corresponding interrupt is occurred.
//! You can define an external interrupt handler with the [`external_interrupt`] attribute.
//! The attribute expects the path to the interrupt source as an argument.
//!
//! The [`external_interrupt`] attribute ensures at compile time that there is a valid
//! external interrupt source for the given handler.
//! Note that external interrupts are target-specific and may not be available on all platforms.
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ## `DefaultHandler`
//!
//! This function is called when interrupt without defined interrupt handler is occurred.
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
//! If `DefaultHandler` is not defined, the linker will use the `abort` function instead.
//!
//! # Cargo Features
//!
//! Those unfamiliar with crate dependency features may want to first refer to
//! [The Cargo Book](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features)
//! for a quick rundown on they work. None are enabled by default.
//!
//! ## `pre-init`
//!
//! When enabled, the runtime will execute the `__pre_init` function to be run **before RAM is initialized**.
//! If the feature is enabled, the `__pre_init` function must be defined in the user code (i.e., no default implementation is
//! provided by this crate). If the feature is disabled, the `__pre_init` function is not required.
//!
//! ### Important implementation guidelines
//!
//! This function is called during the early boot process. Thus, when implementing it, you **MUST** follow these guidelines:
//!
//! - Implement it in assembly (no Rust code is allowed at this point).
//! - Allocate this function within the `.init` section.
//! - Do **NOT** use callee-saved registers `s0-s2`, as they are used to preserve the initial values of `a0-a2` registers.
//! - In RVE targets, do **NOT** use the `a5` register, as it is used to preserve the `a2` register.
//!
//! **Violating these constraints will result in incorrect arguments being passed to `main()`.**
//!
//! ### Implementation example
//!
//! The following example shows how to implement the `__pre_init` function in assembly.
//!
//! ``` no_run
//! core::arch::global_asm!(
//!     r#".section .init.pre_init, "ax"
//!     .global __pre_init
//! __pre_init:
//!     // Do some pre-initialization work here and return
//!     ret
//!     "#
//! );
//! ```
//!
//! ## `post-init`
//!
//! When enabled, the runtime will execute the `__post_init` function to be run before jumping to the main function.
//! If the feature is enabled, the `__post_init` function must be defined in the user code (i.e., no default implementation
//! is provided by this crate). If the feature is disabled, the `__post_init` function is not required.
//!
//! You can use the [`#[post_init]`][attr-post-init] attribute to define a post-init function with Rust.
//!
//! ## `single-hart`
//!
//! Saves a little code size if there is only one hart on the target.
//!
//! ## `s-mode`
//!
//! Supervisor mode. While most registers/instructions have variants for both `mcause` and
//! `scause`, the `mhartid` hardware thread register is not available in supervisor mode.
//! Instead, the hartid is passed as parameter by a bootstrapping firmware (i.e., SBI).
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
//! When vectored trap handling is enabled, the trap vector is set to `_vector_table` in vectored mode.
//! This table is a list of `j _start_INTERRUPT_trap` instructions, where `INTERRUPT` is the name of the
//! core interrupt.
//!
//! ## `u-boot`
//!
//! When the U-boot feature is enabled, acceptable signature for `#[entry]` macros is changed. This is required
//! because when booting from elf, U-boot passes `argc` and `argv`. This feature also implies `single-hart`.
//! The only way to get boot-hart is through fdt, so other harts initialization is up to you.
//!
//! ## `pre-default-start-trap`
//!
//! This provides a mechanism to execute custom code prior to `_default_start_trap`.
//!
//! To use it, the user must define a symbol named `_pre_default_start_trap`, which the system will jump to.
//! After executing the custom code, control should return by jumping to `_pre_default_start_trap_ret`.
//!
//! It's recommended to place the code in the `.trap.start` section to make sure it's reachable from `_default_start_trap`.
//!
//! It is expected that the custom code does not clobber any registers.
//!
//! Please note that your code won't be run for interrupts in vectored mode.
//!
//! ### Example
//!
//! ```rust,no_run
//! core::arch::global_asm!(
//! r#"
//!     .section .trap.start, "ax"
//!     .extern _pre_default_start_trap_ret
//!     .global _pre_default_start_trap
//!
//! _pre_default_start_trap:
//!
//!     // your code goes here remember to not clobber any registers,
//!     // use mscratch to retain a single register if needed
//!
//!     // jump back to continue with _default_start_trap
//!     j _pre_default_start_trap_ret
//! "#
//! );
//! ```
//! [attr-entry]: attr.entry.html
//! [attr-exception]: attr.exception.html
//! [attr-external-interrupt]: attr.external_interrupt.html
//! [attr-core-interrupt]: attr.core_interrupt.html
//! [attr-pre-init]: attr.pre_init.html
//! [attr-post-init]: attr.post_init.html

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod asm;

#[cfg(not(feature = "no-exceptions"))]
pub mod exceptions;

#[cfg(not(feature = "no-interrupts"))]
pub mod interrupts;

#[cfg(feature = "s-mode")]
use riscv::register::{
    scause as xcause,
    stvec::{self as xtvec, Stvec as Xtvec, TrapMode},
};

#[cfg(not(feature = "s-mode"))]
use riscv::register::{
    mcause as xcause,
    mtvec::{self as xtvec, Mtvec as Xtvec, TrapMode},
};

pub use riscv_pac::*;
pub use riscv_rt_macros::{core_interrupt, entry, exception, external_interrupt};

#[cfg(feature = "post-init")]
pub use riscv_rt_macros::post_init;
#[cfg(feature = "pre-init")]
pub use riscv_rt_macros::pre_init;

/// We export this static with an informative name so that if an application attempts to link
/// two copies of riscv-rt together, linking will fail. We also declare a links key in
/// Cargo.toml which is the more modern way to solve the same problem, but we have to keep
/// __ONCE__ around to prevent linking with versions before the links key was added.
#[export_name = "error: riscv-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

/// Rust entry point (_start_rust)
///
/// Configures interrupts and calls main. This function never returns.
///
/// # Safety
///
/// This function should not be called directly by the user, and should instead
/// be invoked by the runtime implicitly.
#[cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    link_section = ".init.rust"
)]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust(a0: usize, a1: usize, a2: usize) -> ! {
    extern "Rust" {
        #[cfg(feature = "post-init")]
        fn __post_init(a0: usize);
        fn _setup_interrupts();
        fn hal_main(a0: usize, a1: usize, a2: usize) -> !;
    }

    #[cfg(feature = "post-init")]
    __post_init(a0);
    _setup_interrupts();
    hal_main(a0, a1, a2);
}

/// Default implementation of `_setup_interrupts`.
///
/// In direct mode (i.e., `v-trap` feature disabled), it sets the trap vector to `_start_trap`.
/// In vectored mode (i.e., `v-trap` feature enabled), it sets the trap vector to `_vector_table`.
///
/// # Note
///
/// Users can override this function by defining their own `_setup_interrupts` function.
///
/// # Safety
///
/// This function should not be called directly by the user, and should instead
/// be invoked by the runtime implicitly. It is expected to be called before the main function.
#[export_name = "_default_setup_interrupts"]
pub unsafe extern "Rust" fn setup_interrupts() {
    extern "C" {
        #[cfg(not(feature = "v-trap"))]
        fn _start_trap();
        #[cfg(feature = "v-trap")]
        fn _vector_table();
    }

    let xtvec_val = match () {
        #[cfg(not(feature = "v-trap"))]
        _ => Xtvec::new(_start_trap as usize, TrapMode::Direct),
        #[cfg(feature = "v-trap")]
        _ => Xtvec::new(_vector_table as usize, TrapMode::Vectored),
    };
    xtvec::write(xtvec_val);
}

/// Registers saved in trap handler
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TrapFrame {
    /// `x1`: return address, stores the address to return to after a function call or interrupt.
    pub ra: usize,
    /// `x5`: temporary register `t0`, used for intermediate values.
    pub t0: usize,
    /// `x6`: temporary register `t1`, used for intermediate values.
    pub t1: usize,
    /// `x7`: temporary register `t2`, used for intermediate values.
    pub t2: usize,
    /// `x28`: temporary register `t3`, used for intermediate values.
    #[cfg(riscvi)]
    pub t3: usize,
    /// `x29`: temporary register `t4`, used for intermediate values.
    #[cfg(riscvi)]
    pub t4: usize,
    /// `x30`: temporary register `t5`, used for intermediate values.
    #[cfg(riscvi)]
    pub t5: usize,
    /// `x31`: temporary register `t6`, used for intermediate values.
    #[cfg(riscvi)]
    pub t6: usize,
    /// `x10`: argument register `a0`. Used to pass the first argument to a function.
    pub a0: usize,
    /// `x11`: argument register `a1`. Used to pass the second argument to a function.
    pub a1: usize,
    /// `x12`: argument register `a2`. Used to pass the third argument to a function.
    pub a2: usize,
    /// `x13`: argument register `a3`. Used to pass the fourth argument to a function.
    pub a3: usize,
    /// `x14`: argument register `a4`. Used to pass the fifth argument to a function.
    pub a4: usize,
    /// `x15`: argument register `a5`. Used to pass the sixth argument to a function.
    pub a5: usize,
    #[cfg(riscvi)]
    /// `x16`: argument register `a6`. Used to pass the seventh argument to a function.
    pub a6: usize,
    #[cfg(riscvi)]
    /// `x17`: argument register `a7`. Used to pass the eighth argument to a function.
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
#[cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    link_section = ".trap.rust"
)]
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

/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut usize {
    extern "C" {
        static mut __sheap: usize;
    }

    #[allow(unused_unsafe)] // no longer unsafe since rust 1.82.0
    unsafe {
        core::ptr::addr_of_mut!(__sheap)
    }
}
