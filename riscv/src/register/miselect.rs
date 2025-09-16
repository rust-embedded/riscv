//! `miselect` register.

const MASK: usize = usize::MAX;

read_write_csr! {
    /// `miselect` register.
    Miselect: 0x350,
    mask: MASK,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Miselect,
    /// Returns whether `miselect` is for custom use of indirect CSRs.
    is_custom: 31,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Miselect,
    /// Returns whether `miselect` is for custom use of indirect CSRs.
    is_custom: 63,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Miselect,
    /// Gets the value stored in the `miselect` CSR.
    ///
    /// # Note
    ///
    /// The semantics of the value depend on the extension for the referenced CSR,
    /// and the relevant `mireg*` value.
    value: [0:30],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Miselect,
    /// Gets the value stored in the `miselect` CSR.
    ///
    /// # Note
    ///
    /// The semantics of the value depend on the extension for the referenced CSR,
    /// and the relevant `mireg*` value.
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
                let mut miselect = Miselect::from_bits(bits);

                test_csr_field!(miselect, is_custom);
                test_csr_field!(miselect, value: [0, usize::BITS - 2], 0);
            });
    }
}
