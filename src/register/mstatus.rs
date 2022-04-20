//! mstatus register

// FIXME: in 1.12 spec there will be `SBE` and `MBE` bits.
// They allows to execute supervisor in given big endian,
// they would be in a new register `mstatush` in RV32; we should implement `mstatush`
// at that time.
// FIXME: `SXL` and `UXL` bits require a structure interpreting XLEN,
// which would be the best way we implement this using Rust?

use bit_field::BitField;
use core::mem::size_of;

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

impl Mstatus {
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

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits.get_bit(3)
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

    /// Machine Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits.get_bit(8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        match self.bits.get_bits(11..13) {
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
        match self.bits.get_bits(13..15) {
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
        match self.bits.get_bits(15..17) {
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
        self.bits.get_bit(17)
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

    /// Trap Virtual Memory
    ///
    /// If this bit is set, reads or writes to `satp` CSR or execute `sfence.vma`
    /// instruction when in S-mode will raise an illegal instruction exception.
    ///
    /// TVM is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tvm(&self) -> bool {
        self.bits.get_bit(20)
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
        self.bits.get_bit(21)
    }

    /// Trap SRET
    ///
    /// Indicates that if SRET instruction should be trapped to raise illegal
    /// instruction exception.
    ///
    /// If S-mode is not supported, TSR bit is hard-wired to 0.
    #[inline]
    pub fn tsr(&self) -> bool {
        self.bits.get_bit(22)
    }

    /*
        FIXME: There are MBE and SBE bits in 1.12; once Privileged Specification version 1.12
        is ratified, there should be read functions of these bits as well.
    */

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
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
    value.set_bits(11..13, mpp as usize);
    _write(value);
}

/// Floating-point extension state
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value.set_bits(13..15, fs as usize);
    _write(value);
}
