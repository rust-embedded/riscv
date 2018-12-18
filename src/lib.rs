//! Low level access to RISCV processors
//!
//! This crate provides:
//!
//! - Access to core registers like mstatus or mcause.
//! - Interrupt manipulation mechanisms.
//! - Safe wrappers around assembly instructions like `mret`.

#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(const_fn)]

extern crate bare_metal;
extern crate bit_field;

pub mod asm;
pub mod interrupt;
pub mod register;
