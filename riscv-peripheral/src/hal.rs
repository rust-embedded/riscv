//! trait implementations for embedded-hal

pub use embedded_hal::*; // re-export embedded-hal to allow macros to use it

pub mod aclint; // ACLINT and CLINT peripherals
