//! misa register

#[cfg(target_arch = "riscv32")]
read_only_csr! {
    /// `misa` register
    Misa: 0x301,
    mask: 0xc3ff_ffff,
    sentinel: 0,
}

#[cfg(not(target_arch = "riscv32"))]
read_only_csr! {
    /// `misa` register
    Misa: 0x301,
    mask: 0xc000_0000_03ff_ffff,
    sentinel: 0,
}

csr_field_enum! {
    /// Base integer ISA width
    XLEN {
        default: XLEN32,
        XLEN32 = 1,
        XLEN64 = 2,
        XLEN128 = 3,
    }
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Misa,
    /// Effective xlen in M-mode (i.e., `MXLEN`).
    mxl,
    XLEN: [30:31],
}

#[cfg(not(target_arch = "riscv32"))]
read_only_csr_field! {
    Misa,
    /// Effective xlen in M-mode (i.e., `MXLEN`).
    mxl,
    XLEN: [62:63],
}

impl Misa {
    /// Returns true when a given extension is implemented.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let misa = unsafe { riscv::register::misa::try_read() }.unwrap();
    /// assert!(misa.has_extension('A')); // panics if atomic extension is not implemented
    /// ```
    #[inline]
    pub fn has_extension(&self, extension: char) -> bool {
        let bit = ext_char_to_bit(extension);
        if bit > 25 {
            return false;
        }
        self.bits() & (1 << bit) == (1 << bit)
    }
}

#[inline]
const fn ext_char_to_bit(extension: char) -> u8 {
    (extension as u8).saturating_sub(b'A')
}
