//! utval register

read_csr_as_usize!(0x043, __read_utval);
write_csr!(0x043, __write_utval);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}
