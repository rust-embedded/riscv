//! dpc register — Debug PC (0x7b1)

read_write_csr! {
    /// Debug PC Register
    Dpc: 0x7b1,
    mask: !1usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpc_alignment_mask() {
        let dpc = Dpc::from_bits(0x1);
        assert_eq!(dpc.bits() & 1, 0);
    }

    #[test]
    fn test_dpc_bits_roundtrip() {
        let dpc = Dpc::from_bits(0x12345);
        assert_eq!(dpc.bits(), 0x12344);
        assert_eq!(Dpc::from_bits(dpc.bits()).bits(), dpc.bits());
    }
}
