//! mcounteren register

use crate::bits::{bf_extract, bf_insert};

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

    /// Sets whether to enable the "cycle\[h\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    #[inline]
    pub fn set_cy(&mut self, cy: bool) {
        self.bits = bf_insert(self.bits, 0, 1, cy as usize);
    }

    /// Supervisor "time\[h\]" Enable
    #[inline]
    pub fn tm(&self) -> bool {
        bf_extract(self.bits, 1, 1) != 0
    }

    /// Sets whether to enable "time\[h\]".
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    #[inline]
    pub fn set_tm(&mut self, tm: bool) {
        self.bits = bf_insert(self.bits, 1, 1, tm as usize);
    }

    /// Supervisor "instret\[h\]" Enable
    #[inline]
    pub fn ir(&self) -> bool {
        bf_extract(self.bits, 2, 1) != 0
    }

    /// Sets whether to enable the "instret\[h\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    #[inline]
    pub fn set_ir(&mut self, ir: bool) {
        self.bits = bf_insert(self.bits, 2, 1, ir as usize);
    }

    /// Supervisor "hpm\[x\]" Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!((3..32).contains(&index));
        bf_extract(self.bits, index, 1) != 0
    }

    /// Sets whether to enable the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    #[inline]
    pub fn set_hpm(&mut self, index: usize, hpm: bool) {
        assert!((3..32).contains(&index));
        self.bits = bf_insert(self.bits, index, 1, hpm as usize);
    }
}

read_csr_as!(Mcounteren, 0x306);
write_csr_as!(Mcounteren, 0x306);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcounteren() {
        let mut m = Mcounteren { bits: 0 };

        assert!(!m.cy());

        m.set_cy(true);
        assert!(m.cy());

        m.set_cy(false);
        assert!(!m.cy());

        assert!(!m.tm());

        m.set_tm(true);
        assert!(m.tm());

        m.set_tm(false);
        assert!(!m.tm());

        assert!(!m.ir());

        m.set_ir(true);
        assert!(m.ir());

        m.set_ir(false);
        assert!(!m.ir());

        (3..32).for_each(|i| {
            assert!(!m.hpm(i));

            m.set_hpm(i, true);
            assert!(m.hpm(i));

            m.set_hpm(i, false);
            assert!(!m.hpm(i));
        });
    }
}
