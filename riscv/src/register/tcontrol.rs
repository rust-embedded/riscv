//! tcontrol register — Trigger Control (0x7a5)
//!
//! Global control for machine-mode triggers (Sdtrig).

read_write_csr! {
    /// Trigger Control Register
    Tcontrol: 0x7a5,
    mask: 0x88,
}

read_write_csr_field! {
    Tcontrol,
    /// M-mode trigger enable (bit 3).
    ///
    /// When a trap into M-mode is taken, `mte` is set to 0. `mret` restores it
    /// from `mpte`.
    mte: 3,
}

read_write_csr_field! {
    Tcontrol,
    /// M-mode previous trigger enable (bit 7).
    mpte: 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcontrol() {
        let mut tcontrol = Tcontrol::from_bits(0);

        test_csr_field!(tcontrol, mte);
        test_csr_field!(tcontrol, mpte);
    }
}
