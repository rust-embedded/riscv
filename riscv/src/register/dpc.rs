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
        (0..=usize::BITS).map(|r| ((1u128 << r) - 1) as usize).for_each(|pc| {
            // ensure lowest bit is cleared
            let exp_pc = pc & !1usize;
            let dpc = Dpc::from_bits(pc);
            assert_eq!(dpc.bits(), exp_pc);
            assert_eq!(Dpc::from_bits(dpc.bits()).bits(), dpc.bits());
        });
    }
}
