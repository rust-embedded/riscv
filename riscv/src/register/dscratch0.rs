//! dscratch0

read_write_csr! {
    /// Debug scratch register 0
    Dscratch0: 0x7b2,
    mask: usize::MAX,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dscratch0_mask() {
        let reg = Dscratch0::from_bits(usize::MAX);
        assert_eq!(reg.bits(), usize::MAX);
        assert_eq!(Dscratch0::BITMASK, usize::MAX);
    }

    #[test]
    fn test_dscratch0_roundtrip() {
        let reg = Dscratch0::from_bits(0xDEAD_BEEFusize);
        assert_eq!(reg.bits(), 0xDEAD_BEEFusize);
    }
}
