//! medeleg register builder

use crate::register::medeleg::*;
use bit_field::BitField;
use core::mem::size_of;

/// medeleg register builder
pub struct MedelegBuilder {
    bits: usize,
}

/// medeleg register value
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct MedelegValue {
    bits: usize,
}

macro_rules! impl_medeleg_writable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_set_bit!(set_instruction_misaligned, $inner, set_bit, 0);

            impl_set_bit!(set_instruction_fault, $inner, set_bit, 1);

            impl_set_bit!(set_illegal_instruction, $inner, set_bit, 2);

            impl_set_bit!(set_breakpoint, $inner, set_bit, 3);

            impl_set_bit!(set_load_misaligned, $inner, set_bit, 4);

            impl_set_bit!(set_load_fault, $inner, set_bit, 5);

            impl_set_bit!(set_store_misaligned, $inner, set_bit, 6);

            impl_set_bit!(set_store_fault, $inner, set_bit, 7);

            impl_set_bit!(set_user_env_call, $inner, set_bit, 8);

            impl_set_bit!(set_supervisor_env_call, $inner, set_bit, 9);

            impl_set_bit!(set_machine_env_call, $inner, set_bit, 11);

            impl_set_bit!(set_instruction_page_fault, $inner, set_bit, 12);

            impl_set_bit!(set_load_page_fault, $inner, set_bit, 13);

            impl_set_bit!(set_store_page_fault, $inner, set_bit, 15);
        }
    };
}

macro_rules! impl_medeleg_readable {
    ($type_name: ident, $inner: ident) => {
        impl $type_name {
            impl_get_bit!(instruction_misaligned, $inner, get_bit, 0);

            impl_get_bit!(instruction_fault, $inner, get_bit, 1);

            impl_get_bit!(illegal_instruction, $inner, get_bit, 2);

            impl_get_bit!(breakpoint, $inner, get_bit, 3);

            impl_get_bit!(load_misaligned, $inner, get_bit, 4);

            impl_get_bit!(load_fault, $inner, get_bit, 5);

            impl_get_bit!(store_misaligned, $inner, get_bit, 6);

            impl_get_bit!(store_fault, $inner, get_bit, 7);

            impl_get_bit!(user_env_call, $inner, get_bit, 8);

            impl_get_bit!(supervisor_env_call, $inner, get_bit, 9);

            impl_get_bit!(machine_env_call, $inner, get_bit, 11);

            impl_get_bit!(instruction_page_fault, $inner, get_bit, 12);

            impl_get_bit!(load_page_fault, $inner, get_bit, 13);

            impl_get_bit!(store_page_fault, $inner, get_bit, 15);
        }
    };
}

impl_medeleg_writable!(MedelegBuilder, bits);
impl_medeleg_readable!(MedelegBuilder, bits);

impl MedelegBuilder {
    #[inline]
    pub fn new() -> Self {
        Self { bits: 0usize }
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn build(&self) -> MedelegValue {
        MedelegValue { bits: self.bits }
    }
}

impl_medeleg_readable!(MedelegValue, bits);

impl MedelegValue {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

impl From<MedelegValue> for MedelegBuilder {
    #[inline]
    fn from(value: MedelegValue) -> Self {
        MedelegBuilder { bits: value.bits() }
    }
}

impl From<MedelegBuilder> for MedelegValue {
    #[inline]
    fn from(value: MedelegBuilder) -> Self {
        MedelegValue { bits: value.bits() }
    }
}
