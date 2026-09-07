//! tselect register — Trigger Select (0x7a0)
//!
//! Selects which trigger the `tdata1`, `tdata2` and `tdata3` registers access.
//! The register is WARL, so writing an unsupported index and reading the value
//! back is the way to discover how many triggers the hart implements.

read_write_csr_as_usize!(0x7a0);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tselect_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
