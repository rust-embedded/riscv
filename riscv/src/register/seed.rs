//! `seed` register — Entropy Source (0x015)
//!
//! This CSR is part of the RISC-V  Scalar Cryptography & Entropy Source extension.
//! It provides an interface to an NIST SP 800-90B or BSI AIS-31 compliant physical entropy source,
//! offering 16 bits of entropy at a time.
//!
//! Access from outside of M-mode is controlled by the `mseccfg` CSR.

csr! {
    ///  Entropy Source Register
    Seed,
    0xC000_FFFF
}

/// Reads the CSR.
///
/// **WARNING**: panics on non-`riscv` targets.
#[inline]
pub fn read() -> Seed {
    try_read().unwrap()
}

/// Attempts to reads the CSR.
#[inline]
pub fn try_read() -> crate::result::Result<Seed> {
    // `seed` can't actually be written to, but read-only CSR instructions will fault
    // because the write is required to signal polling and flushing of the entropy source.
    // For this reason, we need to manually implement reads rather than using the read_only_csr! macro.
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let r: usize;
            unsafe {
                core::arch::asm!(concat!("csrrw {0}, 0x015, x0"), out(reg) r);
            }
            Ok(Seed::from_bits(r))
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => Err(crate::result::Error::Unimplemented),
    }
}

/// Entropy source operational state.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opst {
    /// Built-in self-test in progress; no entropy available yet.
    Bist = 0b00,
    /// Waiting for entropy
    Wait = 0b01,
    /// 16 bits of entropy are available from an NIST SP 800-90B or BSI AIS-31 compliant entropy source (bits 0..15).
    Es16(u16) = 0b10,
    /// Unrecoverable self-test error. The entropy source is dead.
    Dead = 0b11,
}

// Because the lower 16 bits are only meaningful if opst is Es16, we manually implement the enum instead of
// using csr_field_enum! to prevent people from accidentally using insecure entropy bits.
impl Opst {
    /// Attempts to convert a [`usize`] into a valid variant.
    pub const fn from_usize(val: usize) -> crate::result::Result<Self> {
        match (val & Seed::OPST_MASK) >> Seed::OPST_SHIFT {
            0b00 => Ok(Self::Bist),
            0b01 => Ok(Self::Wait),
            0b10 => Ok(Self::Es16(val as u16)),
            0b11 => Ok(Self::Dead),
            _ => Err(crate::result::Error::InvalidVariant(val)),
        }
    }
    /// Converts the variant into a [`usize`].
    pub const fn into_usize(self) -> usize {
        match self {
            Opst::Bist => 0b00,
            Opst::Wait => 0b01,
            Opst::Es16(_) => 0b10,
            Opst::Dead => 0b11,
        }
    }
}

impl From<Opst> for usize {
    fn from(val: Opst) -> Self {
        val.into_usize()
    }
}
impl TryFrom<usize> for Opst {
    type Error = crate::result::Error;
    fn try_from(val: usize) -> crate::result::Result<Self> {
        Self::from_usize(val)
    }
}

impl Seed {
    pub const OPST_SHIFT: usize = 30;
    pub const OPST_WIDTH: usize = 2;
    pub const OPST_MASK: usize = 0xC000_0000;

    pub const ENTROPY_SHIFT: usize = 0;
    pub const ENTROPY_WIDTH: usize = 0x10;
    pub const ENTROPY_MASK: usize = 0xFFFF;

    /// Operational state of the entropy source (bits 30..31).
    #[inline]
    pub fn opst(&self) -> Opst {
        self.try_opst().unwrap()
    }
    /// Operational state of the entropy source (bits 30..31).
    #[inline]
    pub const fn try_opst(&self) -> crate::result::Result<Opst> {
        Opst::from_usize(self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed() {
        let seed = Seed::from_bits((0b00 << 30) | 0x1234);
        assert_eq!(seed.opst(), Opst::Bist);

        let seed = Seed::from_bits((0b01 << 30) | 0x1234);
        assert_eq!(seed.opst(), Opst::Wait);
        let seed = Seed::from_bits((0b10 << 30) | 0x1234);
        assert_eq!(seed.opst(), Opst::Es16(0x1234));

        let seed = Seed::from_bits((0b11 << 30) | 0x1234);
        assert_eq!(seed.opst(), Opst::Dead);
    }

    #[test]
    fn test_seed_bitmask() {
        let seed = Seed::from_bits(usize::MAX);
        assert_eq!(seed.bits(), 0xC000_FFFFusize);
    }
}
