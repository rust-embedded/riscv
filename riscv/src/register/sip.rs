//! sip register

read_write_csr! {
    /// sip register
    Sip: 0x144,
    mask: 0x222,
}

read_write_csr_field! {
    Sip,
    /// Supervisor Software Interrupt Pending
    ssoft: 1,
}

read_only_csr_field! {
    Sip,
    /// Supervisor Timer Interrupt Pending
    stimer: 5,
}

read_only_csr_field! {
    Sip,
    /// Supervisor External Interrupt Pending
    sext: 9,
}

set!(0x144);
clear!(0x144);

set_clear_csr!(
    /// Supervisor Software Interrupt Pending
    , set_ssoft, clear_ssoft, 1 << 1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sip() {
        let mut sip = Sip::from_bits(0);

        test_csr_field!(sip, ssoft);
        assert!(!sip.stimer());
        assert!(!sip.sext());

        assert!(Sip::from_bits(1 << 5).stimer());
        assert!(Sip::from_bits(1 << 9).sext());
    }
}
