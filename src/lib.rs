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
//! and may cause functional problems in systems where some interrupts must NOT be disabled
//! or critical sections are managed as part of an RTOS. In these cases, you should use
//! a target-specific implementation instead, typically provided by a HAL or RTOS crate.
//!
//! ## `aclint`
//!
//! This feature enables the `riscv::peripheral::aclint` module, which provides access to the
//! [Advanced Core Local Interruptor (ACLINT) devices](https://github.com/riscv/riscv-aclint/blob/main/riscv-aclint.adoc).
//! PACs may use this feature to provide a homogenous interface to ACLINT peripherals among targets.
//!
//! ## `clint`
//!
//! This feature enables the `riscv::peripheral::CLINT` struct, which provides access to the
//! [Core Local Interruptor (CLINT) peripheral](https://chromitem-soc.readthedocs.io/en/latest/clint.html).
//! PACs may use this feature to provide a homogenous interface to CLINT peripherals among targets.
//!
//! ## `plic`
//!
//! This feature enables the `riscv::peripheral::PLIC` struct, which provides access to the
//! [Platform-Level Interrupt Controller (PLIC) peripheral](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc).
//! PACs may use this feature to provide a homogenous interface to PLIC peripherals among targets.

#![no_std]
#![allow(clippy::missing_safety_doc)]

pub mod asm;
pub mod delay;
pub mod interrupt;
#[cfg(feature = "peripheral")]
pub mod peripheral;
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
