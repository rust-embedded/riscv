//! ucause register

/// ucause register
#[derive(Clone, Copy, Debug)]
pub struct Ucause {
    bits: usize,
}

impl Ucause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Ucause, 0x042);
write_csr!(0x042);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}
