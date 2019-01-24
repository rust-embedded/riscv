//! Low level access to RISC-V processors
//!
//! This crate provides:
//!
//! - Access to core registers like mstatus or mcause.
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
