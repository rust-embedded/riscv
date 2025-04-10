//! sie register

read_write_csr! {
/// sie register
    Sie: 0x104,
    mask: 0x222,
}

read_write_csr_field! {
    Sie,
    /// Supervisor Software Interrupt Enable
    ssoft: 1,
}

read_write_csr_field! {
    Sie,
    /// Supervisor Timer Interrupt Enable
    stimer: 5,
}

read_write_csr_field! {
    Sie,
    /// Supervisor Timer Interrupt Enable
    sext: 9,
}

set!(0x104);
clear!(0x104);

set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sie() {
        let mut sie = Sie::from_bits(0);

        test_csr_field!(sie, ssoft);
        test_csr_field!(sie, stimer);
        test_csr_field!(sie, sext);
    }
}
