//! mie register

read_write_csr! {
    /// `mie` register
    Mie: 0x304,
    mask: 0xaaa,
}

read_write_csr_field! {
    Mie,
    /// Supervisor Software Interrupt Enable
    ssoft: 1,
}

read_write_csr_field! {
    Mie,
    /// Machine Software Interrupt Enable
    msoft: 3,
}

read_write_csr_field! {
    Mie,
    /// Supervisor Timer Interrupt Enable
    stimer: 5,
}

read_write_csr_field! {
    Mie,
    /// Machine Timer Interrupt Enable
    mtimer: 7,
}

read_write_csr_field! {
    Mie,
    /// Supervisor External Interrupt Enable
    sext: 9,
}

read_write_csr_field! {
    Mie,
    /// Machine External Interrupt Enable
    mext: 11,
}

set!(0x304);
clear!(0x304);

set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Machine Software Interrupt Enable
    , set_msoft, clear_msoft, 1 << 3);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Machine Timer Interrupt Enable
    , set_mtimer, clear_mtimer, 1 << 7);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);
set_clear_csr!(
    /// Machine External Interrupt Enable
    , set_mext, clear_mext, 1 << 11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mie() {
        let mut m = Mie::from_bits(0);

        test_csr_field!(m, ssoft);
        test_csr_field!(m, msoft);
        test_csr_field!(m, stimer);
        test_csr_field!(m, mtimer);
        test_csr_field!(m, sext);
        test_csr_field!(m, mext);
    }
}
