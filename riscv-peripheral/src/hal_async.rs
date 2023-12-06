//! async trait implementations for embedded-hal

pub use embedded_hal_async::*; // re-export embedded-hal-async to allow macros to use it

pub mod aclint; // ACLINT and CLINT peripherals
