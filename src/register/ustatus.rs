//! mstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;
use core::mem::size_of;
pub use super::mstatus::{XS, FS};

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Ustatus {
    bits: usize,
}

impl Ustatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
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

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }
}


read_csr_as!(Ustatus, 0x000, __read_ustatus);
write_csr!(0x000, __write_ustatus);
set!(0x000, __set_ustatus);
clear!(0x000, __clear_ustatus);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);

set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
