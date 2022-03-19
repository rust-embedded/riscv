//! mcounteren register

use bit_field::BitField;

/// mcounteren register
#[derive(Clone, Copy, Debug)]
pub struct Mcounteren {
    bits: usize,
}

impl Mcounteren {
    /// Supervisor "cycle\[h\]" Enable
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor "time\[h\]" Enable
    #[inline]
    pub fn tm(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Supervisor "instret\[h\]" Enable
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Supervisor "hpm\[x\]" Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!(3 <= index && index < 32);
        self.bits.get_bit(index)
    }
}

read_csr_as!(Mcounteren, 0x306);
write_csr!(0x306);
set!(0x306);
clear!(0x306);

set_clear_csr!(
/// Supervisor cycle Enable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// Supervisor time Enable
    , set_tm, clear_tm, 1 << 1);

set_clear_csr!(
/// Supervisor instret Enable
    , set_ir, clear_ir, 1 << 2);

#[inline]
pub unsafe fn set_hpm(index: usize) {
    assert!(3 <= index && index < 32);
    _set(1 << index);
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!(3 <= index && index < 32);
    _clear(1 << index);
}
