//! sstatus register

use bit_field::BitField;
use core::mem::size_of;

/// Supervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: usize,
}

/// Supervisor Previous Privilege Mode
#[derive(Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

/// Floating-point unit Status
#[derive(Eq, PartialEq)]
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

impl Sstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits.get_bit(8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// The status of the floating-point unit
    #[inline]
    pub fn fs(&self) -> FS {
        match self.bits.get_bits(13..15) {
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
        match self.bits.get_bits(15..17) {
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
        self.bits.get_bit(18)
    }

    /// Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits.get_bit(19)
    }

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }
}

read_csr_as!(Sstatus, 0x100);
set!(0x100);
clear!(0x100);

/// User Interrupt Enable
set_clear_csr!(set_uie, clear_uie, 1 << 0);
/// Supervisor Interrupt Enable
set_clear_csr!(set_sie, clear_sie, 1 << 1);
/// User Previous Interrupt Enable
set_csr!(set_upie, 1 << 4);
/// Supervisor Previous Interrupt Enable
set_csr!(set_spie, 1 << 5);
/// Make eXecutable Readable
set_clear_csr!(set_mxr, clear_mxr, 1 << 19);
/// Permit Supervisor User Memory access
set_clear_csr!(set_sum, clear_sum, 1 << 18);

/// Supervisor Previous Privilege Mode
#[inline]
#[cfg(riscv)]
pub unsafe fn set_spp(spp: SPP) {
    _set((spp as usize) << 8);
}

/// The status of the floating-point unit
#[inline]
#[cfg(riscv)]
pub unsafe fn set_fs(fs: FS) {
    _set((fs as usize) << 13);
}
