//! async trait implementations for embedded-hal

pub use embedded_hal_async::*; // re-export embedded-hal-async to allow macros to use it

#[cfg(feature = "aclint-hal-async")]
pub mod aclint; // ACLINT and CLINT peripherals
