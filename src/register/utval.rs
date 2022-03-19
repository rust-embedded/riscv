//! utval register

read_csr_as_usize!(0x043);
write_csr!(0x043);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}
