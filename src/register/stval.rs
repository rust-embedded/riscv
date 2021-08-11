//! stval register

read_csr_as_usize!(0x143, __read_stval);
write_csr!(0x143, __write_stval);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}
