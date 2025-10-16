//! Standard RISC-V peripherals for embedded systems written in Rust.

#![deny(missing_docs)]
#![no_std]

pub use riscv_types::result; // re-export the result module

pub mod common; // common definitions for all peripherals
pub mod hal; // trait implementations for embedded-hal
pub mod macros; // macros for easing the definition of peripherals in PACs

pub mod aclint; // ACLINT and CLINT peripherals
pub mod plic; // PLIC peripheral

#[cfg(test)]
mod test {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[riscv::pac_enum(unsafe ExternalInterruptNumber)]
    pub(crate) enum Interrupt {
        I1 = 1,
        I2 = 2,
        I3 = 3,
        I4 = 4,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[riscv::pac_enum(unsafe PriorityNumber)]
    pub(crate) enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[riscv::pac_enum(unsafe HartIdNumber)]
    pub(crate) enum HartId {
        H0 = 0,
        H1 = 1,
        H2 = 2,
    }
}
