//! satp register

use bit_field::BitField;

/// satp register
#[derive(Clone, Copy, Debug)]
pub struct Satp {
    bits: usize,
}

impl Satp {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Current address-translation scheme
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn mode(&self) -> Mode {
        match self.bits.get_bit(31) {
            false => Mode::Bare,
            true => Mode::Sv32,
        }
    }

    /// Current address-translation scheme
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn mode(&self) -> Mode {
        match self.bits.get_bits(60..64) {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            10 => Mode::Sv57,
            11 => Mode::Sv64,
            _ => unreachable!(),
        }
    }

    /// Address space identifier
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn asid(&self) -> usize {
        self.bits.get_bits(22..31)
    }

    /// Address space identifier
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn asid(&self) -> usize {
        self.bits.get_bits(44..60)
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..22)
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..44)
    }
}

/// 32-bit satp mode
#[cfg(target_pointer_width = "32")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// No translation or protection
    Bare = 0,
    /// Page-based 32-bit virtual addressing
    Sv32 = 1,
}

/// 64-bit satp mode
#[cfg(target_pointer_width = "64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// No translation or protection
    Bare = 0,
    /// Page-based 39-bit virtual addressing
    Sv39 = 8,
    /// Page-based 48-bit virtual addressing
    Sv48 = 9,
    /// Page-based 57-bit virtual addressing
    Sv57 = 10,
    /// Page-based 64-bit virtual addressing
    Sv64 = 11,
}

read_csr_as!(Satp, 0x180);
write_csr_as_usize!(0x180);

/// Sets the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn set(mode: Mode, asid: usize, ppn: usize) {
    let mut bits = 0usize;
    bits.set_bits(31..32, mode as usize);
    bits.set_bits(22..31, asid);
    bits.set_bits(0..22, ppn);
    _write(bits);
}

/// Sets the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn set(mode: Mode, asid: usize, ppn: usize) {
    let mut bits = 0usize;
    bits.set_bits(60..64, mode as usize);
    bits.set_bits(44..60, asid);
    bits.set_bits(0..44, ppn);
    _write(bits);
}
