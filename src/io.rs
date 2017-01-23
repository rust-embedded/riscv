//! Host I/O

use core::{fmt, slice};
use core::fmt::Write;

/// File descriptor
const STDOUT: usize = 1;

/// Host's standard output
struct Stdout;

impl Stdout {
    fn write_all(&mut self, mut buffer: &[u8]) {
        while !buffer.is_empty() {
            match unsafe {
                syscall!(WRITE, STDOUT, buffer.as_ptr(), buffer.len())
            } {
                // Done
                0 => return,
                // `n` bytes were not written
                n => {
                    buffer = unsafe {
                        slice::from_raw_parts(buffer.as_ptr(), buffer.len() - n)
                    }
                }
            }
        }
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}

/// Write a `buffer` to the host's stdout
pub fn write(buffer: &[u8]) {
    Stdout.write_all(buffer)
}

/// Write `fmt::Arguments` to the host's stdout
pub fn write_fmt(args: fmt::Arguments) {
    Stdout.write_fmt(args).ok();
}

/// Write a `string` to the host's stdout
pub fn write_str(string: &str) {
    Stdout.write_all(string.as_bytes())
}
