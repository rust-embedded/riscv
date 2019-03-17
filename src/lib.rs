//! Low level access to RISC-V processors
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.30 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//! Note that `riscv64imac-unknown-none-elf` and `riscv64gc-unknown-none-elf` targets
//! are not supported on stable yet.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Access to core registers like `mstatus` or `mcause`.
//! - Interrupt manipulation mechanisms.
//! - Wrappers around assembly instructions like `WFI`.

#![no_std]
#![deny(warnings)]
#![cfg_attr(feature = "inline-asm", feature(asm))]

extern crate bare_metal;
extern crate bit_field;

pub mod asm;
pub mod interrupt;
pub mod register;
