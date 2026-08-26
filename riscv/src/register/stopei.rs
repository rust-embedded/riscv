//! `stopei` register — Supervisor Top External Interrupt (0x15C)
//!
//! This CSR is part of the RISC-V Advanced Interrupt Architecture (AIA). It reports the
//! highest-priority pending-and-enabled interrupt from the IMSIC supervisor-level interrupt file.
//! The interrupt identity is in bits 26:16 and the interrupt priority (same value) in bits 10:0.

read_write_csr! {
    /// Supervisor Top External Interrupt Register
    Stopei: 0x15C,
    mask: 0x07FF_07FF,
}

read_clear_csr_as!(Stopei, 0x15C);

read_write_csr_field! {
    Stopei,
    /// Interrupt ID (bits 16..26)
    ///
    /// Identifies the specific interrupt source. A value of 0 indicates no interrupt is pending.
    iid: [16:26],
}

read_write_csr_field! {
    Stopei,
    /// Interrupt Priority ID (bits 0..10)
    ///
    /// Represents the priority level of the pending interrupt.
    /// Lower numerical values indicate higher priority interrupts.
    iprio: [0:10],
}

impl Stopei {
    /// Returns true if there is a valid interrupt pending.
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        self.iid() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopei_fields() {
        let mut stopei = Stopei::from_bits(0);
        test_csr_field!(stopei, iid: [16, 26], 0x0);
        test_csr_field!(stopei, iprio: [0, 10], 0x0);

        let mut stopei = Stopei::from_bits((0xB << 16) | 5);
        test_csr_field!(stopei, iid: [16, 26], 0xB);
        test_csr_field!(stopei, iprio: [0, 10], 0x5);

        let mut stopei = Stopei::from_bits((0x7FF << 16) | 0x7FF);
        test_csr_field!(stopei, iid: [16, 26], 0x7FF);
        test_csr_field!(stopei, iprio: [0, 10], 0x7FF);

        let mut stopei = Stopei::from_bits(1 << 16);
        test_csr_field!(stopei, iid: [16, 26], 0x1);
        test_csr_field!(stopei, iprio: [0, 10], 0x0);

        let mut stopei = Stopei::from_bits(1);
        test_csr_field!(stopei, iid: [16, 26], 0x0);
        test_csr_field!(stopei, iprio: [0, 10], 0x1);
    }

    #[test]
    fn test_stopei_bitmask() {
        let stopei = Stopei::from_bits(usize::MAX);
        assert_eq!(stopei.bits(), 0x07FF_07FFusize);
    }
}
