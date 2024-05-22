//! `mcountinhibit` register

use crate::bits::{bf_extract, bf_insert};

/// `mcountinhibit` register
#[derive(Clone, Copy, Debug)]
pub struct Mcountinhibit {
    bits: usize,
}

impl Mcountinhibit {
    /// Machine "cycle\[h\]" Disable
    #[inline]
    pub fn cy(&self) -> bool {
        bf_extract(self.bits, 0, 1) != 0
    }

    /// Sets whether to inhibit the "cycle\[h\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcountinhibit` register.
    #[inline]
    pub fn set_cy(&mut self, cy: bool) {
        self.bits = bf_insert(self.bits, 0, 1, cy as usize);
    }

    /// Machine "instret\[h\]" Disable
    #[inline]
    pub fn ir(&self) -> bool {
        bf_extract(self.bits, 2, 1) != 0
    }

    /// Sets whether to inhibit the "instret\[h\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcountinhibit` register.
    #[inline]
    pub fn set_ir(&mut self, ir: bool) {
        self.bits = bf_insert(self.bits, 2, 1, ir as usize);
    }

    /// Machine "hpm\[x\]" Disable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!((3..32).contains(&index));
        bf_extract(self.bits, index, 1) != 0
    }

    /// Sets whether to inhibit the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcountinhibit` register.
    #[inline]
    pub fn set_hpm(&mut self, index: usize, hpm: bool) {
        assert!((3..32).contains(&index));
        self.bits = bf_insert(self.bits, index, 1, hpm as usize);
    }
}

read_csr_as!(Mcountinhibit, 0x320);
write_csr_as!(Mcountinhibit, 0x320);
set!(0x320);
clear!(0x320);

set_clear_csr!(
/// Machine cycle Disable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// Machine instret Disable
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
    fn test_mcountinhibit() {
        let mut m = Mcountinhibit { bits: 0 };

        assert!(!m.cy());

        m.set_cy(true);
        assert!(m.cy());

        m.set_cy(false);
        assert!(!m.cy());

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
