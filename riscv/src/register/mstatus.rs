//! mstatus register

pub use super::misa::XLEN;

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

/// Additional extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XS {
    /// All off
    AllOff = 0,

    /// None dirty or clean, some on
    NoneDirtyOrClean = 1,

    /// None dirty, some clean
    NoneDirtySomeClean = 2,

    /// Some dirty
    SomeDirty = 3,
}

/// Floating-point extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

/// Machine Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

/// Non-instruction-fetch memory endianness
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endianness {
    BigEndian = 1,
    LittleEndian = 0,
}

impl From<bool> for Endianness {
    fn from(value: bool) -> Self {
        match value {
            true => Self::BigEndian,
            false => Self::LittleEndian,
        }
    }
}

impl Mstatus {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits & (1 << 3) != 0
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// U-mode non-instruction-fetch memory endianness
    #[inline]
    pub fn ube(&self) -> Endianness {
        Endianness::from(self.bits & (1 << 6) != 0)
    }

    /// Machine Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits & (1 << 7) != 0
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) != 0 {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        let mpp = (self.bits >> 11) & 0x3; // bits 11-12
        match mpp {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    /// Floating-point extension state
    ///
    /// Encodes the status of the floating-point unit,
    /// including the CSR `fcsr` and floating-point data registers `f0–f31`.
    #[inline]
    pub fn fs(&self) -> FS {
        let fs = (self.bits >> 13) & 0x3; // bits 13-14
        match fs {
            0b00 => FS::Off,
            0b01 => FS::Initial,
            0b10 => FS::Clean,
            0b11 => FS::Dirty,
            _ => unreachable!(),
        }
    }

    /// Additional extension state
    ///
    /// Encodes the status of additional user-mode extensions and associated state.
    #[inline]
    pub fn xs(&self) -> XS {
        let xs = (self.bits >> 15) & 0x3; // bits 15-16
        match xs {
            0b00 => XS::AllOff,
            0b01 => XS::NoneDirtyOrClean,
            0b10 => XS::NoneDirtySomeClean,
            0b11 => XS::SomeDirty,
            _ => unreachable!(),
        }
    }

    /// Modify Memory PRiVilege
    #[inline]
    pub fn mprv(&self) -> bool {
        self.bits & (1 << 17) != 0
    }

    /// Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits & (1 << 18) != 0
    }

    /// Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits & (1 << 19) != 0
    }

    /// Trap Virtual Memory
    ///
    /// If this bit is set, reads or writes to `satp` CSR or execute `sfence.vma`
    /// instruction when in S-mode will raise an illegal instruction exception.
    ///
    /// TVM is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tvm(&self) -> bool {
        self.bits & (1 << 20) != 0
    }

    /// Timeout Wait
    ///
    /// Indicates that if WFI instruction should be intercepted.
    ///
    /// If this bit is set, when WFI is executed in S-mode, and it does not complete
    /// within an implementation specific, bounded time limit, the WFI instruction will cause
    /// an illegal instruction trap; or could always cause trap then the time limit is zero.
    ///
    /// TW is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tw(&self) -> bool {
        self.bits & (1 << 21) != 0
    }

    /// Trap SRET
    ///
    /// Indicates that if SRET instruction should be trapped to raise illegal
    /// instruction exception.
    ///
    /// If S-mode is not supported, TSR bit is hard-wired to 0.
    #[inline]
    pub fn tsr(&self) -> bool {
        self.bits & (1 << 22) != 0
    }

    /// Effective xlen in U-mode (i.e., `UXLEN`).
    ///
    /// In RISCV-32, UXL does not exist, and `UXLEN` is always [`XLEN::XLEN32`].
    #[inline]
    pub fn uxl(&self) -> XLEN {
        match () {
            #[cfg(riscv32)]
            () => XLEN::XLEN32,
            #[cfg(not(riscv32))]
            () => XLEN::from((self.bits >> 32) as u8 & 0x3),
        }
    }

    /// Effective xlen in S-mode (i.e., `SXLEN`).
    ///
    /// In RISCV-32, SXL does not exist, and SXLEN is always [`XLEN::XLEN32`].
    #[inline]
    pub fn sxl(&self) -> XLEN {
        match () {
            #[cfg(riscv32)]
            () => XLEN::XLEN32,
            #[cfg(not(riscv32))]
            () => XLEN::from((self.bits >> 34) as u8 & 0x3),
        }
    }

    /// S-mode non-instruction-fetch memory endianness.
    ///
    /// In RISCV-32, this field is read from the [`crate::register::mstatush`] register.
    pub fn sbe(&self) -> Endianness {
        match () {
            #[cfg(riscv32)]
            () => super::mstatush::read().sbe(),
            #[cfg(not(riscv32))]
            () => Endianness::from(self.bits & (1 << 36) != 0),
        }
    }

    /// M-mode non-instruction-fetch memory endianness
    ///
    /// In RISCV-32, this field is read from the [`crate::register::mstatush`] register
    pub fn mbe(&self) -> Endianness {
        match () {
            #[cfg(riscv32)]
            () => super::mstatush::read().mbe(),
            #[cfg(not(riscv32))]
            () => Endianness::from(self.bits & (1 << 37) != 0),
        }
    }

    /// Whether either the FS field or XS field signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits & (1 << (usize::BITS as usize - 1)) != 0
    }
}

read_csr_as!(Mstatus, 0x300);
write_csr!(0x300);
set!(0x300);
clear!(0x300);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);
set_clear_csr!(
    /// Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);
set_clear_csr!(
    /// Machine Interrupt Enable
    , set_mie, clear_mie, 1 << 3);
set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
set_csr!(
    /// Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);
set_csr!(
    /// Machine Previous Interrupt Enable
    , set_mpie, 1 << 7);
set_clear_csr!(
    /// Modify Memory PRiVilege
    , set_mprv, clear_mprv, 1 << 17);
set_clear_csr!(
    /// Permit Supervisor User Memory access
    , set_sum, clear_sum, 1 << 18);
set_clear_csr!(
    /// Make eXecutable Readable
    , set_mxr, clear_mxr, 1 << 19);
set_clear_csr!(
    /// Trap Virtual Memory
    , set_tvm, clear_tvm, 1 << 20);
set_clear_csr!(
    /// Timeout Wait
    , set_tw, clear_tw, 1 << 21);
set_clear_csr!(
    /// Trap SRET
    , set_tsr, clear_tsr, 1 << 22);

/// Set U-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_ube(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 6),
        Endianness::LittleEndian => _clear(1 << 6),
    }
}

/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    match spp {
        SPP::Supervisor => _set(1 << 8),
        SPP::User => _clear(1 << 8),
    }
}

/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    let mut value = _read();
    value &= !(0x3 << 11); // clear previous value
    value |= (mpp as usize) << 11;
    _write(value);
}

/// Floating-point extension state
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value &= !(0x3 << 13); // clear previous value
    value |= (fs as usize) << 13;
    _write(value);
}

/// Set S-mode non-instruction-fetch memory endianness
///
/// # Note
///
/// In RISCV-32, this function calls [`crate::register::mstatush::set_sbe`]
#[inline]
pub unsafe fn set_sbe(endianness: Endianness) {
    match () {
        #[cfg(riscv32)]
        () => super::mstatush::set_sbe(endianness),
        #[cfg(not(riscv32))]
        () => match endianness {
            Endianness::BigEndian => _set(1 << 36),
            Endianness::LittleEndian => _clear(1 << 36),
        },
    }
}

/// Set M-mode non-instruction-fetch memory endianness
///
/// # Note
///
/// In RISCV-32, this function calls [`crate::register::mstatush::set_mbe`]
#[inline]
pub unsafe fn set_mbe(endianness: Endianness) {
    match () {
        #[cfg(riscv32)]
        () => super::mstatush::set_mbe(endianness),
        #[cfg(not(riscv32))]
        () => match endianness {
            Endianness::BigEndian => _set(1 << 37),
            Endianness::LittleEndian => _clear(1 << 37),
        },
    }
}
