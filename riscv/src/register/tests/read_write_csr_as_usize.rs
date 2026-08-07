use crate::result::{Error, Result};

read_write_csr_as_usize!(0x000);

// we don't test the `read` and `write` functions, we are only testing in-memory functions.
#[allow(unused)]
pub fn _read_csr() -> usize {
    read()
}

#[allow(unused)]
pub fn _try_read_csr() -> Result<usize> {
    try_read()
}

#[allow(unused)]
pub fn _write_csr(bits: usize) {
    unsafe { write(bits) };
}

#[allow(unused)]
pub fn _try_write_csr(bits: usize) -> Result<()> {
    unsafe { try_write(bits) }
}

#[allow(unused)]
pub fn _read_csr_bits() -> usize {
    read_bits()
}

#[allow(unused)]
pub fn _try_read_csr_bits() -> Result<usize> {
    try_read_bits()
}

#[allow(unused)]
pub fn _write_csr_bits(bits: usize) {
    unsafe { write_bits(bits) };
}

#[allow(unused)]
pub fn _try_write_csr_bits(bits: usize) -> Result<()> {
    unsafe { try_write_bits(bits) }
}

#[test]
fn test_usize_raw_bits() {
    assert_eq!(try_read_bits(), Err(Error::Unimplemented));
    assert_eq!(
        unsafe { try_write_bits(usize::MAX) },
        Err(Error::Unimplemented)
    );
}
