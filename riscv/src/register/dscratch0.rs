//! dscratch0

read_write_csr_as_usize!(0x7b2);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_dscratch0_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
