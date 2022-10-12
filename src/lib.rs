//! Low level access to RISC-V processors
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.59 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Access to core registers like `mstatus` or `mcause`.
//! - Interrupt manipulation mechanisms.
//! - Wrappers around assembly instructions like `WFI`.
//!
//! # Optional features
//!
//! ## `critical-section-single-hart`
//!
//! This feature enables a [`critical-section`](https://github.com/rust-embedded/critical-section)
//! implementation suitable for single-hart targets, based on disabling interrupts globally.
//!
//! It is **unsound** to enable it on multi-hart targets,
//! and may cause functional problems in systems where some interrupts must be not be disabled
//! or critical sections are managed as part of an RTOS. In these cases, you should use
//! a target-specific implementation instead, typically provided by a HAL or RTOS crate.

#![no_std]

pub mod asm;
pub mod delay;
pub mod interrupt;
pub mod register;

#[macro_use]
mod macros;

#[cfg(all(riscv, feature = "critical-section-single-hart"))]
mod critical_section;

/// Used to reexport items for use in macros. Do not use directly.
/// Not covered by semver guarantees.
#[doc(hidden)]
pub mod _export {
    pub use critical_section;
}
