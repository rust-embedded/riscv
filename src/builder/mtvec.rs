//! mtvec builder

use crate::register::mtvec::*;
use bit_field::BitField;

/// mtvec register builder
#[derive(Clone, Copy, Debug)]
pub struct MtvecBuilder {
    bits: usize,
}

/// mtvec register value
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct MtvecValue {
    bits: usize,
}

impl MtvecBuilder {
    #[inline]
    pub fn new() -> Self {
        Self { bits: 0usize }
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn build(&self) -> MtvecValue {
        MtvecValue { bits: self.bits() }
    }

    #[inline]
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    #[inline]
    pub fn set_address(&mut self, value: usize) {
        self.bits = value - (value & 0b11);
    }

    #[inline]
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            _ => None,
        }
    }

    #[inline]
    pub fn set_trap_mode(&mut self, mode: TrapMode) {
        let mode_bits = match mode {
            TrapMode::Direct => 0,
            TrapMode::Vectored => 1,
        };
        self.bits.set_bits(0..2, mode_bits);
    }
}

impl MtvecValue {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
}

impl From<MtvecValue> for MtvecBuilder {
    #[inline]
    fn from(value: MtvecValue) -> Self {
        MtvecBuilder { bits: value.bits() }
    }
}

impl From<MtvecBuilder> for MtvecValue {
    #[inline]
    fn from(value: MtvecBuilder) -> Self {
        MtvecValue { bits: value.bits() }
    }
}
