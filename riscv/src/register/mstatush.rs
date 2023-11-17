//! mstatush register (RISCV-32 only)

pub use super::mstatus::Endianness;

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatush {
    bits: usize,
}

impl Mstatush {
    /// S-mode non-instruction-fetch memory endianness
    #[inline]
    pub fn sbe(&self) -> Endianness {
        Endianness::from(self.bits & (1 << 4) != 0)
    }

    /// M-mode non-instruction-fetch memory endianness
    #[inline]
    pub fn mbe(&self) -> Endianness {
        Endianness::from(self.bits & (1 << 5) != 0)
    }
}

read_csr_as_rv32!(Mstatush, 0x310);
write_csr_rv32!(0x310);
set_rv32!(0x310);
clear_rv32!(0x310);

/// Set S-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_sbe(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 4),
        Endianness::LittleEndian => _clear(1 << 4),
    }
}

/// Set M-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_mbe(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 5),
        Endianness::LittleEndian => _clear(1 << 5),
    }
}
