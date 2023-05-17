//! mideleg builder

use crate::register::mideleg::*;
use bit_field::BitField;

/// mideleg builder
#[derive(Clone, Copy, Debug)]
pub struct MidelegBuilder {
    bits: usize,
}

/// mideleg builder
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct MidelegValue {
    bits: usize,
}

macro_rules! impl_mideleg_writable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_set_bit!(set_usoft, $inner, set_bit, 0);

            impl_set_bit!(set_ssoft, $inner, set_bit, 1);

            impl_set_bit!(set_utimer, $inner, set_bit, 4);

            impl_set_bit!(set_stimer, $inner, set_bit, 5);

            impl_set_bit!(set_uext, $inner, set_bit, 8);

            impl_set_bit!(set_sext, $inner, set_bit, 9);
        }
    };
}

macro_rules! impl_mideleg_readable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_get_bit!(usoft, $inner, get_bit, 0);

            impl_get_bit!(ssoft, $inner, get_bit, 1);

            impl_get_bit!(utimer, $inner, get_bit, 4);

            impl_get_bit!(stimer, $inner, get_bit, 5);

            impl_get_bit!(uext, $inner, get_bit, 8);

            impl_get_bit!(sext, $inner, get_bit, 9);
        }
    };
}

impl_mideleg_readable!(MidelegBuilder, bits);
impl_mideleg_writable!(MidelegBuilder, bits);

impl MidelegBuilder {
    #[inline]
    pub fn new() -> Self {
        Self { bits: 0usize }
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn build(&self) -> MidelegValue {
        MidelegValue { bits: self.bits }
    }
}

impl_mideleg_readable!(MidelegValue, bits);

impl MidelegValue {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

impl From<MidelegValue> for MidelegBuilder {
    #[inline]
    fn from(value: MidelegValue) -> Self {
        MidelegBuilder { bits: value.bits() }
    }
}

impl From<MidelegBuilder> for MidelegValue {
    #[inline]
    fn from(value: MidelegBuilder) -> Self {
        MidelegValue { bits: value.bits() }
    }
}
