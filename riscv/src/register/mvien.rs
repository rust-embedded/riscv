//! mvien register

#[cfg(target_arch = "riscv32")]
const MASK: usize = 0xffff_e222;
#[cfg(not(target_arch = "riscv32"))]
const MASK: usize = 0xffff_ffff_ffff_e222;

read_write_csr! {
    /// `mvien` register
    Mvien: 0x308,
    mask: MASK,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mip.SSIP`
    ssip: 1,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mip.STIP`
    stip: 5,
}

read_write_csr_field! {
    Mvien,
    /// Alias of `mip.SEIP`
    seip: 9,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Mvien,
    /// Represents the enable status of a virtual major interrupt.
    interrupt: 13..=31,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Mvien,
    /// Represents the enable status of a virtual major interrupt.
    interrupt: 13..=63,
}

set!(0x308);
clear!(0x308);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvien() {
        let mut m = Mvien::from_bits(0);

        test_csr_field!(m, ssip);
        test_csr_field!(m, stip);
        test_csr_field!(m, seip);

        (13..64).for_each(|idx| {
            test_csr_field!(m, interrupt, idx);
        });
    }
}
