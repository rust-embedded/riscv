//! mtopi register — Machine Top Priority Interrupt (0x7C0)
//!
//! Provides information about the highest-priority pending interrupt when AIA (Advanced Interrupt Architecture) is supported.
//! This CSR is part of the RISC-V Advanced Interrupt Architecture extension and allows software to quickly
//! identify the most important pending interrupt without scanning through multiple interrupt pending registers.
//!
//! # Usage
//!
//! ```no_run
//! use riscv::register::mtopi;
//!
//! // Read the machine top priority interrupt register
//! let mtopi_val = mtopi::read();
//!
//! if mtopi_val.has_interrupt() {
//!     let interrupt_id = mtopi_val.interrupt_id();
//!     let priority = mtopi_val.priority();
//!     println!("Highest priority interrupt: ID={}, Priority={}", interrupt_id, priority);
//! } else {
//!     println!("No interrupts pending");
//! }
//! ```

read_only_csr! {
    /// Machine Top Priority Interrupt Register
    Mtopi: 0x7C0,
    mask: usize::MAX,
}

read_only_csr_field! {
    Mtopi,
    /// Interrupt ID (bits 16..27)
    ///
    /// Identifies the specific interrupt source. A value of 0 indicates no interrupt is pending.
    /// Non-zero values correspond to specific interrupt sources as defined by the interrupt controller.
    iid: [16:27],
}

read_only_csr_field! {
    Mtopi,
    /// Interrupt Priority ID (bits 0..7)
    ///
    /// Represents the priority level of the pending interrupt.
    /// Higher numerical values indicate higher priority interrupts.
    ipid: [0:7],
}

impl Mtopi {
    /// Returns true if there is a valid interrupt pending
    ///
    /// When this returns true, both `interrupt_id()` and `priority()` will return meaningful values.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        self.iid() != 0
    }

    /// Returns the interrupt priority, with higher values indicating higher priority
    ///
    /// This value is only meaningful when `has_interrupt()` returns true.
    #[inline]
    pub fn priority(&self) -> usize {
        self.ipid()
    }

    /// Returns the interrupt identifier
    ///
    /// A value of 0 indicates no interrupt is pending. Non-zero values identify
    /// specific interrupt sources as defined by the interrupt controller configuration.
    #[inline]
    pub fn interrupt_id(&self) -> usize {
        self.iid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ro_csr_field {
        // test a multi-bit bitfield for read-only CSR
        ($reg:ident, $field:ident: [$start:expr, $end:expr]) => {{
            let bits = $reg.bits();
            let shift = $end - $start + 1;
            let mask = (1usize << shift) - 1;
            let exp_val = (bits >> $start) & mask;

            // Test that field extraction matches expected value
            assert_eq!($reg.$field(), exp_val);
        }};
    }

    #[test]
    fn test_mtopi_fields() {
        let mut mtopi = Mtopi::from_bits(0);

        // Test iid field [16:27] with zero bits
        test_ro_csr_field!(mtopi, iid: [16, 27]);
        // Test ipid field [0:7] with zero bits
        test_ro_csr_field!(mtopi, ipid: [0, 7]);

        assert!(!mtopi.has_interrupt());
        assert_eq!(mtopi.priority(), 0);
        assert_eq!(mtopi.interrupt_id(), 0);

        // Test with some interrupt pending (IID = 11, IPID = 5)
        mtopi = Mtopi::from_bits((11 << 16) | 5);
        test_ro_csr_field!(mtopi, iid: [16, 27]);
        test_ro_csr_field!(mtopi, ipid: [0, 7]);
        assert!(mtopi.has_interrupt());
        assert_eq!(mtopi.priority(), 5);
        assert_eq!(mtopi.interrupt_id(), 11);

        // Test maximum values for each field
        mtopi = Mtopi::from_bits((0xFFF << 16) | 0xFF);
        test_ro_csr_field!(mtopi, iid: [16, 27]);
        test_ro_csr_field!(mtopi, ipid: [0, 7]);
        assert!(mtopi.has_interrupt());
    }

    #[test]
    fn test_mtopi_bitmask() {
        let mtopi = Mtopi::from_bits(usize::MAX);
        assert_eq!(mtopi.bits(), usize::MAX);
    }
}
