//! IMPLEMENTATION DETAILS USED BY MACROS

use crate::hio::{self, HostStream};
use core::fmt::{self, Write};

static mut HSTDOUT: Option<HostStream> = None;

static mut HSTDERR: Option<HostStream> = None;

#[cfg(not(feature = "u-mode"))]
mod machine {
    use super::*;

    pub fn hstdout_str(s: &str) {
        let _result = critical_section::with(|_| unsafe {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }

            HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
        });
    }

    pub fn hstdout_fmt(args: fmt::Arguments) {
        let _result = critical_section::with(|_| unsafe {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }

            HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
        });
    }

    pub fn hstderr_str(s: &str) {
        let _result = critical_section::with(|_| unsafe {
            if HSTDERR.is_none() {
                HSTDERR = Some(hio::hstderr()?);
            }

            HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
        });
    }

    pub fn hstderr_fmt(args: fmt::Arguments) {
        let _result = critical_section::with(|_| unsafe {
            if HSTDERR.is_none() {
                HSTDERR = Some(hio::hstderr()?);
            }

            HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
        });
    }
}
#[cfg(not(feature = "u-mode"))]
pub use machine::*;

#[cfg(feature = "u-mode")]
mod user {
    use super::*;
    pub fn hstdout_str(s: &str) {
        let _result = unsafe {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout().unwrap());
            }

            HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
        };
    }

    pub fn hstdout_fmt(args: fmt::Arguments) {
        let _result = unsafe {
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout().unwrap());
            }

            HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
        };
    }

    static mut HSTDERR: Option<HostStream> = None;

    pub fn hstderr_str(s: &str) {
        let _result = unsafe {
            if HSTDERR.is_none() {
                HSTDERR = Some(hio::hstderr().unwrap());
            }

            HSTDERR.as_mut().unwrap().write_str(s).map_err(drop)
        };
    }

    pub fn hstderr_fmt(args: fmt::Arguments) {
        let _result = unsafe {
            if HSTDERR.is_none() {
                HSTDERR = Some(hio::hstderr().unwrap());
            }

            HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop)
        };
    }
}
#[cfg(feature = "u-mode")]
pub use user::*;
