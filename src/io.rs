//! Host I/O

use core::{fmt, slice};
use core::fmt::Write;

/// File descriptors
const STDOUT: usize = 1;
const STDERR: usize = 2;

/// Host's standard error
struct Stderr;

/// Host's standard output
struct Stdout;

fn write_all(fd: usize, mut buffer: &[u8]) {
    while !buffer.is_empty() {
        match unsafe { syscall!(WRITE, fd, buffer.as_ptr(), buffer.len()) } {
            // Done
            0 => return,
            // `n` bytes were not written
            n => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe {
                    slice::from_raw_parts(buffer.as_ptr().offset(offset as isize), n)
                }
            }
        }
    }
}

impl Stderr {
    fn write_all(&mut self, buffer: &[u8]) {
        write_all(STDERR, buffer);
    }
}

impl Stdout {
    fn write_all(&mut self, buffer: &[u8]) {
        write_all(STDOUT, buffer);
    }
}

impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}

/// Write a `buffer` to the host's stderr
pub fn ewrite(buffer: &[u8]) {
    Stderr.write_all(buffer)
}

/// Write `fmt::Arguments` to the host's stderr
pub fn ewrite_fmt(args: fmt::Arguments) {
    Stderr.write_fmt(args).ok();
}

/// Write a `string` to the host's stderr
pub fn ewrite_str(string: &str) {
    Stderr.write_all(string.as_bytes())
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
