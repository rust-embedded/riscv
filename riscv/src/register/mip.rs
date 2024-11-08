//! mip register

read_write_csr! {
    /// `mip` register
    Mip: 0x344,
    mask: 0xaaa,
}

read_write_csr_field! {
    Mip,
    /// Supervisor Software Interrupt Pending
    ssoft: 1,
}

read_only_csr_field! {
    Mip,
    /// Machine Software Interrupt Pending
    msoft: 3,
}

read_write_csr_field! {
    Mip,
    /// Supervisor Timer Interrupt Pending
    stimer: 5,
}

read_only_csr_field! {
    Mip,
    /// Machine Timer Interrupt Pending
    mtimer: 7,
}

read_write_csr_field! {
    Mip,
    /// Supervisor External Interrupt Pending
    sext: 9,
}

read_only_csr_field! {
    Mip,
    /// Machine External Interrupt Pending
    mext: 11,
}

set!(0x344);
clear!(0x344);

set_clear_csr!(
    /// Supervisor Software Interrupt Pending
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Supervisor Timer Interrupt Pending
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Supervisor External Interrupt Pending
    , set_sext, clear_sext, 1 << 9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mip() {
        let mut m = Mip::from_bits(0);

        test_csr_field!(m, ssoft);
        test_csr_field!(m, stimer);
        test_csr_field!(m, sext);

        assert!(!m.msoft());
        assert!(!m.mtimer());
        assert!(!m.mext());

        assert!(Mip::from_bits(1 << 3).msoft());
        assert!(Mip::from_bits(1 << 7).mtimer());
        assert!(Mip::from_bits(1 << 11).mext());
    }
}
