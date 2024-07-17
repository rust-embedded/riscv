//! mcounteren register

use crate::bits::{bf_extract, bf_insert};
use crate::result::{Error, Result};

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
    ///
    /// **WARNING**: panics on `index` out-of-bounds
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        self.try_hpm(index).unwrap()
    }

    /// Fallible Supervisor "hpm\[x\]" Enable (bits 3-31).
    ///
    /// Attempts to read the "hpm\[x\]" value, and returns an error if the `index` is invalid.
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

    /// Sets whether to enable the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    ///
    /// **WARNING**: panics on `index` out-of-bounds
    #[inline]
    pub fn set_hpm(&mut self, index: usize, hpm: bool) {
        self.try_set_hpm(index, hpm).unwrap()
    }

    /// Sets whether to enable the "hpm\[X\]" counter.
    ///
    /// Only updates the in-memory value, does not modify the `mcounteren` register.
    ///
    /// Attempts to update the "hpm\[x\]" value, and returns an error if the `index` is invalid.
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

/// Enables the "hpm\[X\]" counter.
///
/// Updates the `mcounteren` register.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - `index` out-of-bounds
#[inline]
pub unsafe fn set_hpm(index: usize) {
    try_set_hpm(index).unwrap();
}

/// Attempts to enable the "hpm\[X\]" counter.
///
/// Updates the `mcounteren` register.
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

/// Disables the "hpm\[X\]" counter.
///
/// Updates the `mcounteren` register.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - `index` out-of-bounds
#[inline]
pub unsafe fn clear_hpm(index: usize) {
    try_clear_hpm(index).unwrap();
}

/// Attempts to disable the "hpm\[X\]" counter.
///
/// Updates the `mcounteren` register.
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
            assert_eq!(m.try_hpm(i), Ok(false));

            m.set_hpm(i, true);
            assert!(m.hpm(i));

            assert_eq!(m.try_set_hpm(i, false), Ok(()));
            assert_eq!(m.try_hpm(i), Ok(false));

            assert!(!m.hpm(i));
        });

        (0..3).chain(32..64).for_each(|index| {
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
        })
    }
}
