//! dscratch1

read_write_csr_as_usize!(Dscratch1, 0x7b3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dscratch1_mask() {
        let reg = Dscratch1::from_bits(usize::MAX);
        assert_eq!(reg.bits(), usize::MAX);
        assert_eq!(Dscratch1::BITMASK, usize::MAX);
    }

    #[test]
    fn test_dscratch1_roundtrip() {
        let reg = Dscratch1::from_bits(0xDEAD_BEEFusize);
        assert_eq!(reg.bits(), 0xDEAD_BEEFusize);
    }
}
