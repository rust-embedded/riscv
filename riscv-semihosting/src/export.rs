//! IMPLEMENTATION DETAILS USED BY MACROS
use crate::hio::{self, HostStream};
use core::{
    fmt::{self, Write},
    ptr::addr_of_mut,
};

static mut HSTDOUT: Option<HostStream> = None;
static mut HSTDERR: Option<HostStream> = None;

#[cfg(not(feature = "u-mode"))]
mod machine {
    use super::*;

    pub fn hstdout_str(s: &str) {
        let _result = critical_section::with(|_| {
            let hstdout = unsafe { &mut *addr_of_mut!(HSTDOUT) };
            if hstdout.is_none() {
                *hstdout = Some(hio::hstdout()?);
            }

            hstdout.as_mut().unwrap().write_str(s).map_err(drop)
        });
    }

    pub fn hstdout_fmt(args: fmt::Arguments) {
        let _result = critical_section::with(|_| {
            let hstdout = unsafe { &mut *addr_of_mut!(HSTDOUT) };
            if hstdout.is_none() {
                *hstdout = Some(hio::hstdout()?);
            }

            hstdout.as_mut().unwrap().write_fmt(args).map_err(drop)
        });
    }

    pub fn hstderr_str(s: &str) {
        let _result = critical_section::with(|_| {
            let hstderr = unsafe { &mut *addr_of_mut!(HSTDERR) };
            if hstderr.is_none() {
                *hstderr = Some(hio::hstderr()?);
            }

            hstderr.as_mut().unwrap().write_str(s).map_err(drop)
        });
    }

    pub fn hstderr_fmt(args: fmt::Arguments) {
        let _result = critical_section::with(|_| {
            let hstderr = unsafe { &mut *addr_of_mut!(HSTDERR) };
            if hstderr.is_none() {
                *hstderr = Some(hio::hstderr()?);
            }

            hstderr.as_mut().unwrap().write_fmt(args).map_err(drop)
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
            let hstdout = &mut *addr_of_mut!(HSTDOUT);
            if hstdout.is_none() {
                *hstdout = Some(hio::hstdout().unwrap());
            }

            hstdout.as_mut().unwrap().write_str(s).map_err(drop)
        };
    }

    pub fn hstdout_fmt(args: fmt::Arguments) {
        let _result = unsafe {
            let hstdout = &mut *addr_of_mut!(HSTDOUT);
            if hstdout.is_none() {
                *hstdout = Some(hio::hstdout().unwrap());
            }

            hstdout.as_mut().unwrap().write_fmt(args).map_err(drop)
        };
    }

    pub fn hstderr_str(s: &str) {
        let _result = unsafe {
            let hstderr = &mut *addr_of_mut!(HSTDERR);
            if hstderr.is_none() {
                *hstderr = Some(hio::hstderr().unwrap());
            }

            hstderr.as_mut().unwrap().write_str(s).map_err(drop)
        };
    }

    pub fn hstderr_fmt(args: fmt::Arguments) {
        let _result = unsafe {
            let hstderr = &mut *addr_of_mut!(HSTDERR);
            if hstderr.is_none() {
                *hstderr = Some(hio::hstderr().unwrap());
            }

            hstderr.as_mut().unwrap().write_fmt(args).map_err(drop)
        };
    }
}
#[cfg(feature = "u-mode")]
pub use user::*;
