//! vstopi register — Virtual Supervisor Top Priority Interrupt (0xEB0)

read_only_csr! {
    /// Virtual Supervisor Top Priority Interrupt Register
    Vstopi: 0xEB0,
    mask: 0x0FFF_00FF,
}

read_only_csr_field! {
    Vstopi,
    /// Interrupt ID (bits 16..27)
    ///
    /// Identifies the specific interrupt source. A value of 0 indicates no interrupt is pending.
    /// Non-zero values correspond to specific interrupt sources as defined by the interrupt controller.
    iid: [16:27],
}

read_only_csr_field! {
    Vstopi,
    /// Interrupt Priority ID (bits 0..7)
    ///
    /// Represents the priority level of the pending interrupt.
    /// Lower numerical values indicate higher priority interrupts.
    iprio: [0:7],
}

impl Vstopi {
    /// Returns true if there is a valid interrupt pending
    ///
    /// When this returns true, both `interrupt_id()` and `priority()` will return meaningful values.
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        self.iid() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vstopi_fields() {
        let vstopi = Vstopi::from_bits(0);
        test_ro_csr_field!(vstopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(vstopi, iprio: [0, 7], 0x0);

        let vstopi = Vstopi::from_bits((0xB << 16) | 5);
        test_ro_csr_field!(vstopi, iid: [16, 27], 0xB);
        test_ro_csr_field!(vstopi, iprio: [0, 7], 0x5);

        let vstopi = Vstopi::from_bits((0xFFF << 16) | 0xFF);
        test_ro_csr_field!(vstopi, iid: [16, 27], 0xFFF);
        test_ro_csr_field!(vstopi, iprio: [0, 7], 0xFF);

        let vstopi = Vstopi::from_bits(1 << 16);
        test_ro_csr_field!(vstopi, iid: [16, 27], 0x1);
        test_ro_csr_field!(vstopi, iprio: [0, 7], 0x0);

        let vstopi = Vstopi::from_bits(1);
        test_ro_csr_field!(vstopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(vstopi, iprio: [0, 7], 0x1);
    }

    #[test]
    fn test_vstopi_bitmask() {
        let vstopi = Vstopi::from_bits(usize::MAX);
        assert_eq!(vstopi.bits(), 0x0FFF_00FFusize);
    }
}
