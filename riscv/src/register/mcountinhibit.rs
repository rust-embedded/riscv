//! `mcountinhibit` register

use crate::read_write_csr;
use crate::result::{Error, Result};

read_write_csr! {
    /// `mcountinhibit` register
    Mcountinhibit: 0x320,
    mask: 0xffff_fffd,
    /// Gets the `cycle[h]` inhibit field value.
    cy,
    /// Attempts to get the `cycle[h]` inhibit field value.
    try_cy,
    /// Sets the `cycle[h]` inhibit field value.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    set_cy,
    /// Attempts to set the `cycle[h]` inhibit field value.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    try_set_cy,
    bit: 0,
    /// Gets the `instret[h]` inhibit field value.
    ir,
    /// Attempts to get the `instret[h]` inhibit field value.
    try_ir,
    /// Sets the `instret[h]` inhibit field value.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    set_ir,
    /// Attempts to set the `instret[h]` inhibit field value.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    try_set_ir,
    bit: 2,
}

read_write_csr_field! {
    Mcountinhibit,
    /// Gets the `mhpmcounterX[h]` inhibit field value.
    ///
    /// **WARN**: `index` must be in the range `[31:3]`.
    hpm,
    /// Attempts to get the `mhpmcounterX[h]` inhibit field value.
    ///
    /// **WARN**: `index` must be in the range `[31:3]`.
    try_hpm,
    /// Sets the `mhpmcounterX[h]` inhibit field value.
    ///
    /// **WARN**: `index` must be in the range `[31:3]`.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    set_hpm,
    /// Sets the `mhpmcounterX[h]` inhibit field value.
    ///
    /// **WARN**: `index` must be in the range `[31:3]`.
    ///
    /// **NOTE**: only updates the in-memory value without touching the CSR.
    try_set_hpm,
    range: 3..=31,
}

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
