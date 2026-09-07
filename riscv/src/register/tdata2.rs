//! tdata2 register — Trigger Data 2 (0x7a2)
//!
//! Second data register of the trigger selected by [`tselect`](crate::register::tselect). For address/data
//! match triggers this typically holds the compare value.

read_write_csr_as_usize!(0x7a2);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tdata2_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
