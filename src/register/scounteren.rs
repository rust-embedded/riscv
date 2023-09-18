//! scounteren register

/// scounteren register
#[derive(Clone, Copy, Debug)]
pub struct Scounteren {
    bits: usize,
}

impl Scounteren {
    /// User "cycle\[h\]" Enable
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits & (1 << 0) != 0
    }

    /// User "time\[h\]" Enable
    #[inline]
    pub fn tm(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// User "instret\[h]\" Enable
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits & (1 << 2) != 0
    }

    /// User "hpm\[x\]" Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!((3..32).contains(&index));
        self.bits & (1 << index) != 0
    }
}

read_csr_as!(Scounteren, 0x106);
write_csr!(0x106);
set!(0x106);
clear!(0x106);

set_clear_csr!(
/// User cycle Enable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// User time Enable
    , set_tm, clear_tm, 1 << 1);

set_clear_csr!(
/// User instret Enable
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
