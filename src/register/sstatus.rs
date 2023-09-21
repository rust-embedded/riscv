//! sstatus register

pub use super::misa::XLEN;
pub use super::mstatus::FS;

/// Supervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: usize,
}

/// Supervisor Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Sstatus {
    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) != 0 {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// The status of the floating-point unit
    #[inline]
    pub fn fs(&self) -> FS {
        let fs = (self.bits >> 13) & 0x3; // bits 13-14
        match fs {
            0 => FS::Off,
            1 => FS::Initial,
            2 => FS::Clean,
            3 => FS::Dirty,
            _ => unreachable!(),
        }
    }

    /// The status of additional user-mode extensions
    /// and associated state
    #[inline]
    pub fn xs(&self) -> FS {
        let xs = (self.bits >> 15) & 0x3; // bits 15-16
        match xs {
            0 => FS::Off,
            1 => FS::Initial,
            2 => FS::Clean,
            3 => FS::Dirty,
            _ => unreachable!(),
        }
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

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits & (1 << (usize::BITS as usize - 1)) != 0
    }
}

read_csr_as!(Sstatus, 0x100);
write_csr!(0x100);
set!(0x100);
clear!(0x100);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);
set_clear_csr!(
    /// Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);
set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
set_csr!(
    /// Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);
set_clear_csr!(
    /// Permit Supervisor User Memory access
    , set_sum, clear_sum, 1 << 18);
set_clear_csr!(
    /// Make eXecutable Readable
    , set_mxr, clear_mxr, 1 << 19);

/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    match spp {
        SPP::Supervisor => _set(1 << 8),
        SPP::User => _clear(1 << 8),
    }
}

/// The status of the floating-point unit
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value &= !(0x3 << 13); // clear previous value
    value |= (fs as usize) << 13;
    _write(value);
}
