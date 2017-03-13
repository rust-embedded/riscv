//! Semihosting operations

pub const CLOCK: usize = 0x10;
pub const CLOSE: usize = 0x05;
pub const ELAPSED: usize = 0x30;
pub const ERRNO: usize = 0x13;
pub const FLEN: usize = 0x0c;
pub const GET_CMDLINE: usize = 0x15;
pub const HEAPINFO: usize = 0x16;
pub const ISERROR: usize = 0x08;
pub const ISTTY: usize = 0x09;
pub const OPEN: usize = 0x01;
pub const READ: usize = 0x06;
pub const READC: usize = 0x07;
pub const REMOVE: usize = 0x0e;
pub const RENAME: usize = 0x0f;
pub const SEEK: usize = 0x0a;
pub const SYSTEM: usize = 0x12;
pub const TICKFREQ: usize = 0x31;
pub const TIME: usize = 0x11;
pub const TMPNAM: usize = 0x0d;
pub const WRITE0: usize = 0x04;
pub const WRITE: usize = 0x05;
pub const WRITEC: usize = 0x03;
pub const ENTER_SVC: usize = 0x17;
pub const REPORT_EXCEPTION: usize = 0x18;
