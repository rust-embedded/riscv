//! Standard RISC-V peripherals for embedded systems written in Rust

#![deny(missing_docs)]
#![no_std]

pub use riscv; // re-export riscv crate to allow macros to use it

pub mod common; // common definitions for all peripherals
pub mod hal; // trait implementations for embedded-hal
#[cfg(feature = "embedded-hal-async")]
pub mod hal_async; // async trait implementations for embedded-hal
pub mod macros; // macros for easing the definition of peripherals in PACs

pub mod aclint; // ACLINT and CLINT peripherals
pub mod plic; // PLIC peripheral
