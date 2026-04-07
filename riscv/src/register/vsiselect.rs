//! `vsiselect` register.

const MASK: usize = usize::MAX;

read_write_csr! {
    /// `vsiselect` register.
    Vsiselect: 0x250,
    mask: MASK,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Vsiselect,
    /// Returns whether `vsiselect` is for custom use of indirect CSRs.
    is_custom: 31,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Vsiselect,
    /// Returns whether `vsiselect` is for custom use of indirect CSRs.
    is_custom: 63,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Vsiselect,
    /// Gets the value stored in the `vsiselect` CSR.
    ///
    /// # Note
    ///
    /// The semantics of the value depend on the extension for the referenced CSR,
    /// and the relevant `vsireg*` value.
    value: [0:30],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Vsiselect,
    /// Gets the value stored in the `vsiselect` CSR.
    ///
    /// # Note
    ///
    /// The semantics of the value depend on the extension for the referenced CSR,
    /// and the relevant `vsireg*` value.
    value: [0:62],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        (0..=usize::BITS)
            .map(|r| ((1u128 << r) - 1) as usize)
            .for_each(|bits| {
                let mut vsiselect = Vsiselect::from_bits(bits);

                test_csr_field!(vsiselect, is_custom);
                test_csr_field!(vsiselect, value: [0, usize::BITS - 2], 0);
            });
    }
}
