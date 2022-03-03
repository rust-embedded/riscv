//! mstatus register builder

use bit_field::BitField;
use core::mem::size_of;
use core::arch::asm;
use crate::register::mstatus::*;

/// mstatus register builder
pub struct MstatusBuilder {
    bits: usize
}

/// mstatus register value
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct MstatusValue {
    bits: usize
}

macro_rules! impl_mstatus_writable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_set_bit!(set_uie, $inner, set_bit, 0);

            impl_set_bit!(set_sie, $inner, set_bit, 1);

            impl_set_bit!(set_mie, $inner, set_bit, 3);

            impl_set_bit!(set_upie, $inner, set_bit, 4);

            impl_set_bit!(set_spie, $inner, set_bit, 5);

            impl_set_bit!(set_mpie, $inner, set_bit, 7);

            #[inline]
            pub fn set_spp(&mut self, spp: SPP) {
                match spp {
                    SPP::Supervisor => self.bits.set_bit(8, true),
                    SPP::User => self.bits.set_bit(8, false),
                };
            }
            #[inline]
            pub fn set_mpp(&mut self, mpp: MPP) {
                match mpp {
                    MPP::User => self.bits.set_bits(11..13, 0b00),
                    MPP::Supervisor => self.bits.set_bits(11..13, 0b01),
                    MPP::Machine => self.bits.set_bits(11..13, 0b11),
                };
            }

            #[inline]
            pub fn set_fs(&mut self, fs: FS) {
                match fs {
                    FS::Off => self.bits.set_bits(13..15, 0b00),
                    FS::Initial => self.bits.set_bits(13..15, 0b01),
                    FS::Clean => self.bits.set_bits(13..15, 0b10),
                    FS::Dirty => self.bits.set_bits(13..15, 0b11),
                };
            }
        
            #[inline]
            pub fn set_xs(&mut self, xs: XS) {
                match xs {
                    XS::AllOff => self.bits.set_bits(15..17, 0b00),
                    XS::NoneDirtyOrClean => self.bits.set_bits(15..17, 0b01),
                    XS::NoneDirtySomeClean => self.bits.set_bits(15..17, 0b10),
                    XS::SomeDirty => self.bits.set_bits(15..17, 0b11),
                };
            }

            impl_set_bit!(set_mprv, $inner, set_bit, 17);

            impl_set_bit!(set_sum, $inner, set_bit, 18);

            impl_set_bit!(set_mxr, $inner, set_bit, 19);

            impl_set_bit!(set_tvm, $inner, set_bit, 20);

            impl_set_bit!(set_tw, $inner, set_bit, 21);

            impl_set_bit!(set_tsr, $inner, set_bit, 22);

        }
    }
}

macro_rules! impl_mstatus_readable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_get_bit!(uie, $inner, get_bit, 0);

            impl_get_bit!(sie, $inner, get_bit, 1);

            impl_get_bit!(mie, $inner, get_bit, 3);

            impl_get_bit!(upie, $inner, get_bit, 4);

            impl_get_bit!(spie, $inner, get_bit, 5);

            impl_get_bit!(mpie, $inner, get_bit, 7);

            #[inline]
            pub fn spp(&self) -> SPP {
                match self.$inner.get_bit(8) {
                    true => SPP::Supervisor,
                    false => SPP::User,
                }
            }

            #[inline]
            pub fn mpp(&self) -> MPP {
                match self.$inner.get_bits(11..13) {
                    0b00 => MPP::User,
                    0b01 => MPP::Supervisor,
                    0b11 => MPP::Machine,
                    _ => unreachable!(),
                }
            }

            #[inline]
            pub fn fs(&self) -> FS {
                match self.$inner.get_bits(13..15) {
                    0b00 => FS::Off,
                    0b01 => FS::Initial,
                    0b10 => FS::Clean,
                    0b11 => FS::Dirty,
                    _ => unreachable!(),
                }
            }

            #[inline]
            pub fn xs(&self) -> XS {
                match self.$inner.get_bits(15..17) {
                    0b00 => XS::AllOff,
                    0b01 => XS::NoneDirtyOrClean,
                    0b10 => XS::NoneDirtySomeClean,
                    0b11 => XS::SomeDirty,
                    _ => unreachable!(),
                }
            }

            impl_get_bit!(mprv, $inner, get_bit, 17);

            impl_get_bit!(sum, $inner, get_bit, 18);

            impl_get_bit!(mxr, $inner, get_bit, 19);

            impl_get_bit!(tvm, $inner, get_bit, 20);

            impl_get_bit!(tw, $inner, get_bit, 21);

            impl_get_bit!(tsr, $inner, get_bit, 22);

            #[inline]
            pub fn sd(&self) -> bool {
                self.$inner.get_bit(size_of::<usize>() * 8 - 1)
            }
        }
    }
}

impl_mstatus_readable!(MstatusBuilder, bits);
impl_mstatus_writable!(MstatusBuilder, bits);

impl MstatusBuilder {
    pub fn build(&self) -> MstatusValue {
        return MstatusValue { bits: self.bits }
    }
}

impl MstatusValue {
    pub unsafe fn write_mstatus(&self) {
        asm!("csrw mstatus, {0}", in(reg) self.bits)
    }
}
