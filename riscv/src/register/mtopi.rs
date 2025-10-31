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
//! if mtopi_val.is_interrupt_pending() {
//!     let interrupt_id = mtopi_val.iid();
//!     let priority = mtopi_val.iprio();
//!     println!("Highest priority interrupt: ID={}, Priority={}", interrupt_id, priority);
//! } else {
//!     println!("No interrupts pending");
//! }
//! ```

read_only_csr! {
    /// Machine Top Priority Interrupt Register
    Mtopi: 0x7C0,
    mask: 0x0FFF_00FF,
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
    /// Lower numerical values indicate higher priority interrupts.
    iprio: [0:7],
}

impl Mtopi {
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
    fn test_mtopi_fields() {
        let mtopi = Mtopi::from_bits(0);

        test_ro_csr_field!(mtopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(mtopi, iprio: [0, 7], 0x0);

        let mtopi = Mtopi::from_bits((11 << 16) | 5);
        test_ro_csr_field!(mtopi, iid: [16, 27], 0xB);
        test_ro_csr_field!(mtopi, iprio: [0, 7], 0x5);

        let mtopi = Mtopi::from_bits((0xFFF << 16) | 0xFF);
        test_ro_csr_field!(mtopi, iid: [16, 27], 0xFFF);
        test_ro_csr_field!(mtopi, iprio: [0, 7], 0xFF);

        let mtopi = Mtopi::from_bits(1 << 16);
        test_ro_csr_field!(mtopi, iid: [16, 27], 0x1);
        test_ro_csr_field!(mtopi, iprio: [0, 7], 0x0);

        let mtopi = Mtopi::from_bits(1);
        test_ro_csr_field!(mtopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(mtopi, iprio: [0, 7], 0x1);
    }

    #[test]
    fn test_mtopi_bitmask() {
        let mtopi = Mtopi::from_bits(usize::MAX);
        assert_eq!(mtopi.bits(), 0x0FFF_00FFusize);
    }
}
