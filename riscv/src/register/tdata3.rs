//! tdata3 register — Trigger Data 3 (0x7a3)
//!
//! Third data register of the trigger selected by [`tselect`](crate::register::tselect).
//! Holds trigger-specific data. When [`tdata1`](crate::register::tdata1) `type` is 2, 3, 4, 5, or 6,
//! this register is interpreted as `textra32` (RV32) or `textra64` (RV64).

read_write_csr_as_usize!(0x7a3);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tdata3_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
