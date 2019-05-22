//! Minimal startup / runtime for RISC-V CPU's
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.31 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//! Note that `riscv64imac-unknown-none-elf` and `riscv64gc-unknown-none-elf` targets
//! are not supported on stable yet.
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - Before main initialization of the FPU (for targets that have a FPU).
//!
//! - `#[entry]` to declare the entry point of the program
//! - `#[pre_init]` to run code *before* `static` variables are initialized
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
//! [dependencies]
//! riscv-rt = "0.4.0"
//! panic-halt = "0.2.0"
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//! }
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
//!   "-C", "link-arg=-Tlink.x"
//! ]
//!
//! [build]
//! target = "riscv32imac-unknown-none-elf"
//! $ edit build.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! use std::env;
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::Path;
//!
//! /// Put the linker script somewhere the linker can find it.
//! fn main() {
//!     let out_dir = env::var("OUT_DIR").expect("No out dir");
//!     let dest_path = Path::new(&out_dir);
//!     let mut f = File::create(&dest_path.join("memory.x"))
//!         .expect("Could not create file");
//!
//!     f.write_all(include_bytes!("memory.x"))
//!         .expect("Could not write file");
//!
//!     println!("cargo:rustc-link-search={}", dest_path.display());
//!
//!     println!("cargo:rerun-if-changed=memory.x");
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
//! 20000000:	800011b7          	lui	gp,0x80001
//! 20000004:	80018193          	addi	gp,gp,-2048 # 80000800 <_stack_start+0xffffc800>
//! 20000008:	80004137          	lui	sp,0x80004
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
//! ```
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
//! ## `pre_init!`
//!
//! A user-defined function can be run at the start of the reset handler, before RAM is
//! initialized. The macro `pre_init!` can be called to set the function to be run. The function is
//! intended to perform actions that cannot wait the time it takes for RAM to be initialized, such
//! as disabling a watchdog. As the function is called before RAM is initialized, any access of
//! static variables will result in undefined behavior.

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate riscv;
extern crate riscv_rt_macros as macros;
extern crate r0;

pub use macros::{entry, pre_init};

use riscv::register::mstatus;

#[export_name = "error: riscv-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}


/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust() -> ! {
    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();

        fn _mp_hook() -> bool;
    }

    if _mp_hook() {
        __pre_init();

        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    // TODO: Enable FPU when available

    main();
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

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "Rust" fn default_pre_init() {}

#[doc(hidden)]
#[no_mangle]
pub extern "Rust" fn default_mp_hook() -> bool {
    use riscv::register::mhartid;
    match mhartid::read() {
        0 => true,
        _ => loop {
            unsafe { riscv::asm::wfi() }
        },
    }
}
