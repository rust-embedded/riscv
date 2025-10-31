//! mvienh register

read_write_csr! {
    /// `mvienh` register
    Mvienh: 0x318,
    mask: 0xffff_ffff,
}

read_write_csr_field! {
    Mvienh,
    /// Represents the enable status of a virtual major interrupt.
    interrupt: 0..=31,
}

set!(0x318);
clear!(0x318);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvienh() {
        let mut m = Mvienh::from_bits(0);

        (0..32).for_each(|idx| {
            test_csr_field!(m, interrupt, idx);
        });
    }
}
