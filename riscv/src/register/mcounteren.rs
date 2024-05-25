//! mcounteren register

use crate::bits::bf_extract;

/// mcounteren register
#[derive(Clone, Copy, Debug)]
pub struct Mcounteren {
    bits: usize,
}

impl Mcounteren {
    /// Supervisor "cycle\[h\]" Enable
    #[inline]
    pub fn cy(&self) -> bool {
        bf_extract(self.bits, 0, 1) != 0
    }

    /// Supervisor "time\[h\]" Enable
    #[inline]
    pub fn tm(&self) -> bool {
        bf_extract(self.bits, 1, 1) != 0
    }

    /// Supervisor "instret\[h\]" Enable
    #[inline]
    pub fn ir(&self) -> bool {
        bf_extract(self.bits, 2, 1) != 0
    }

    /// Supervisor "hpm\[x\]" Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!((3..32).contains(&index));
        bf_extract(self.bits, index, 1) != 0
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
    assert!((3..32).contains(&index));
    _set(1 << index);
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _clear(1 << index);
}
