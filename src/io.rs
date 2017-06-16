//! Host I/O

use core::{fmt, slice};
use core::fmt::Write;
use nr;

/// Host's standard error
pub struct Stderr {
    fd: usize,
}

/// Construct a new handle to the host's standard error.
pub fn stderr() -> Stderr {
    Stderr { fd: open(":tt\0", nr::open::W_APPEND).unwrap() }
}

/// Host's standard output
pub struct Stdout {
    fd: usize,
}

/// Construct a new handle to the host's standard output.
pub fn stdout() -> Stdout {
    Stdout { fd: open(":tt\0", nr::open::W_TRUNC).unwrap() }
}

fn open(name: &str, mode: usize) -> Result<usize, ()> {
    let name = name.as_bytes();
    match unsafe { syscall!(OPEN, name.as_ptr(), mode, name.len() - 1) } as isize {
        -1 => Err(()),
        fd => Ok(fd as usize),
    }
}

fn write_all(fd: usize, mut buffer: &[u8]) -> fmt::Result {
    while !buffer.is_empty() {
        match unsafe { syscall!(WRITE, fd, buffer.as_ptr(), buffer.len()) } {
            // Done
            0 => return Ok(()),
            // `n` bytes were not written
            n if n <= buffer.len() => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe {
                    slice::from_raw_parts(buffer.as_ptr().offset(offset), n)
                }
            }
            // Error
            _ => return Err(fmt::Error::default()),
        }
    }
    Ok(())
}

impl Stderr {
    fn write_all(&mut self, buffer: &[u8]) -> fmt::Result {
        write_all(self.fd, buffer)
    }
}

impl Stdout {
    fn write_all(&mut self, buffer: &[u8]) -> fmt::Result {
        write_all(self.fd, buffer)
    }
}

impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes())
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes())
    }
}

/// Write a `buffer` to the host's stderr
pub fn ewrite(buffer: &[u8]) {
    stderr().write_all(buffer).ok();
}

/// Write `fmt::Arguments` to the host's stderr
pub fn ewrite_fmt(args: fmt::Arguments) {
    stderr().write_fmt(args).ok();
}

/// Write a `string` to the host's stderr
pub fn ewrite_str(string: &str) {
    stderr().write_all(string.as_bytes()).ok();
}

/// Write a `buffer` to the host's stdout
pub fn write(buffer: &[u8]) {
    stdout().write_all(buffer).ok();
}

/// Write `fmt::Arguments` to the host's stdout
pub fn write_fmt(args: fmt::Arguments) {
    stdout().write_fmt(args).ok();
}

/// Write a `string` to the host's stdout
pub fn write_str(string: &str) {
    stdout().write_all(string.as_bytes()).ok();
}
