//! `mcountinhibit` register

use crate::bits::{bf_extract, bf_insert};
use crate::result::{Error, Result};

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

    /// Machine "hpm\[x\]" Disable (bits 3-31)
    ///
    /// Attempts to read the "hpm\[x\]" value, and returns an error if the index is invalid.
    #[inline]
    pub fn try_hpm(&self, index: usize) -> Result<bool> {
        if (3..32).contains(&index) {
            Ok(bf_extract(self.bits, index, 1) != 0)
        } else {
            Err(Error::IndexOutOfBounds {
                index,
                min: 3,
                max: 31,
            })
        }
    }

    /// Sets whether to inhibit the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcountinhibit` register.
    #[inline]
    pub fn set_hpm(&mut self, index: usize, hpm: bool) {
        assert!((3..32).contains(&index));
        self.bits = bf_insert(self.bits, index, 1, hpm as usize);
    }

    /// Sets whether to inhibit the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcountinhibit` register.
    ///
    /// Attempts to update the "hpm\[x\]" value, and returns an error if the index is invalid.
    #[inline]
    pub fn try_set_hpm(&mut self, index: usize, hpm: bool) -> Result<()> {
        if (3..32).contains(&index) {
            self.bits = bf_insert(self.bits, index, 1, hpm as usize);
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds {
                index,
                min: 3,
                max: 31,
            })
        }
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
pub unsafe fn try_set_hpm(index: usize) -> Result<()> {
    if (3..32).contains(&index) {
        _try_set(1 << index)
    } else {
        Err(Error::IndexOutOfBounds {
            index,
            min: 3,
            max: 31,
        })
    }
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _clear(1 << index);
}

#[inline]
pub unsafe fn try_clear_hpm(index: usize) -> Result<()> {
    if (3..32).contains(&index) {
        _try_clear(1 << index)
    } else {
        Err(Error::IndexOutOfBounds {
            index,
            min: 3,
            max: 31,
        })
    }
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
            assert_eq!(m.try_hpm(i), Ok(false));

            m.set_hpm(i, true);
            assert!(m.hpm(i));
            assert_eq!(m.try_hpm(i), Ok(true));

            assert_eq!(m.try_set_hpm(i, false), Ok(()));
            assert_eq!(m.try_hpm(i), Ok(false));
        });

        (0..2).chain(32..64).for_each(|index| {
            assert_eq!(
                m.try_hpm(index),
                Err(Error::IndexOutOfBounds {
                    index,
                    min: 3,
                    max: 31
                })
            );
            assert_eq!(
                m.try_set_hpm(index, false),
                Err(Error::IndexOutOfBounds {
                    index,
                    min: 3,
                    max: 31
                })
            );
        });
    }
}
