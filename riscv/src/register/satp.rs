//! satp register

use crate::result::{Error, Result};

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
    ///
    /// **WARNING**: panics if the field has an invalid variant.
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn mode(&self) -> Mode {
        self.try_mode().unwrap()
    }

    /// Attempts to get the current address-translation scheme.
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn try_mode(&self) -> Result<Mode> {
        ((self.bits >> 31) as u8).try_into()
    }

    /// Current address-translation scheme
    ///
    /// **WARNING**: panics if the field has an invalid variant.
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn mode(&self) -> Mode {
        self.try_mode().unwrap()
    }

    /// Attempts to get the current address-translation scheme.
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn try_mode(&self) -> Result<Mode> {
        ((self.bits >> 60) as u8).try_into()
    }

    /// Address space identifier
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn asid(&self) -> usize {
        (self.bits >> 22) & 0x1FF // bits 22-30
    }

    /// Address space identifier
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn asid(&self) -> usize {
        self.bits >> 44 & 0xFFFF // bits 44-59
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn ppn(&self) -> usize {
        self.bits & 0x3F_FFFF // bits 0-21
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn ppn(&self) -> usize {
        self.bits & 0xFFF_FFFF_FFFF // bits 0-43
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

#[cfg(target_pointer_width = "32")]
impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match val {
            0 => Ok(Mode::Bare),
            1 => Ok(Mode::Sv32),
            _ => Err(Error::InvalidVariant {
                field: "mode",
                value: val as usize,
            }),
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match val {
            0 => Ok(Mode::Bare),
            8 => Ok(Mode::Sv39),
            9 => Ok(Mode::Sv48),
            10 => Ok(Mode::Sv57),
            11 => Ok(Mode::Sv64),
            _ => Err(Error::InvalidVariant {
                field: "mode",
                value: val as usize,
            }),
        }
    }
}

read_csr_as!(Satp, 0x180);
write_csr_as_usize!(0x180);

/// Sets the register to corresponding page table mode, physical page number and address space id.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - invalid field values
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn set(mode: Mode, asid: usize, ppn: usize) {
    try_set(mode, asid, ppn).unwrap();
}

/// Attempts to set the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn try_set(mode: Mode, asid: usize, ppn: usize) -> Result<()> {
    if asid != asid & 0x1FF {
        Err(Error::InvalidValue {
            field: "asid",
            value: asid,
            bitmask: 0x1FF,
        })
    } else if ppn != ppn & 0x3F_FFFF {
        Err(Error::InvalidValue {
            field: "ppn",
            value: ppn,
            bitmask: 0x3F_FFFF,
        })
    } else {
        let bits = (mode as usize) << 31 | (asid << 22) | ppn;
        _try_write(bits)
    }
}

/// Sets the register to corresponding page table mode, physical page number and address space id.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - invalid field values
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn set(mode: Mode, asid: usize, ppn: usize) {
    try_set(mode, asid, ppn).unwrap()
}

/// Attempts to set the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn try_set(mode: Mode, asid: usize, ppn: usize) -> Result<()> {
    if asid != asid & 0xFFFF {
        Err(Error::InvalidValue {
            field: "asid",
            value: asid,
            bitmask: 0xFFFF,
        })
    } else if ppn != ppn & 0xFFF_FFFF_FFFF {
        Err(Error::InvalidValue {
            field: "ppn",
            value: ppn,
            bitmask: 0xFFF_FFFF_FFFF,
        })
    } else {
        let bits = (mode as usize) << 60 | (asid << 44) | ppn;
        _try_write(bits)
    }
}
