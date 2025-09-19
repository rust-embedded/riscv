//! dcsr register — Debug Control and Status Register (0x7b0)
//!
//! Provides control and status for debug mode, including cause of entry, step control, and privilege level.

read_write_csr! {
    /// Debug Control and Status Register
    Dcsr: 0x7b0,
    mask: 0xffff_ffff,
}

csr_field_enum! {
    /// Operating privilege level.
    Prv {
        default: Machine,
        /// User/Application.
        User = 0b00,
        /// Supervisor.
        Supervisor = 0b01,
        /// Machine.
        Machine = 0b11,
    }
}

csr_field_enum! {
    /// Cause for entering debug mode.
    Cause {
        default: None,
        /// No cause.
        None = 0,
        /// EBREAK instruction.
        Ebreak = 1,
        /// Trigger module.
        Trigger = 2,
        /// External halt request.
        HaltRequest = 3,
        /// Single-step completed.
        Step = 4,
        /// Reset-halt request.
        ResetHaltRequest = 5,
    }
}

read_write_csr_field! {
    Dcsr,
    /// Previous privilege level when entering debug mode (bits 0..1).
    prv,
    Prv: [0:1],
}

read_write_csr_field! {
    Dcsr,
    /// Single step mode (bit 2)
    step: 2,
}

read_only_csr_field! {
    Dcsr,
    /// Non-maskable interrupt pending (bit 3)
    nmip: 3,
}

read_write_csr_field! {
    Dcsr,
    /// Use mstatus.mprv in debug mode (bit 4)
    mprven: 4,
}

read_only_csr_field! {
    Dcsr,
    /// Cause for entering debug mode (bits 6..8)
    cause,
    Cause: [6:8],
}

read_write_csr_field! {
    Dcsr,
    /// Stop timer increment in debug mode (bit 9)
    stoptime: 9,
}

read_write_csr_field! {
    Dcsr,
    /// Stop counter increment in debug mode (bit 10)
    stopcount: 10,
}

read_write_csr_field! {
    Dcsr,
    /// Interrupt enable during single-step (bit 11)
    stepie: 11,
}

read_write_csr_field! {
    Dcsr,
    /// EBREAK behavior in User mode (bit 12)
    ebreaku: 12,
}

read_write_csr_field! {
    Dcsr,
    /// EBREAK behavior in Supervisor mode (bit 13)
    ebreaks: 13,
}

read_write_csr_field! {
    Dcsr,
    /// EBREAK behavior in Machine mode (bit 15)
    ebreakm: 15,
}

read_only_csr_field! {
    Dcsr,
    /// Debug version (bits 28..31)
    xdebugver: [28:31],
}

impl Dcsr {
    /// Returns the debug cause as an enum
    pub fn debug_cause(&self) -> crate::result::Result<Cause> {
        self.try_cause()
    }

    /// Returns the previous privilege level as an enum
    pub fn privilege_level(&self) -> crate::result::Result<Prv> {
        self.try_prv()
    }

    /// Sets the previous privilege level
    pub fn set_privilege_level(&mut self, level: Prv) {
        self.set_prv(level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_dcsr_bitfields() {
        let mut dcsr = Dcsr::from_bits(0);

        dcsr.set_step(true);
        assert!(dcsr.step());
        dcsr.set_mprven(true);
        assert!(dcsr.mprven());
        dcsr.set_stoptime(true);
        assert!(dcsr.stoptime());
        dcsr.set_stopcount(true);
        assert!(dcsr.stopcount());
        dcsr.set_stepie(true);
        assert!(dcsr.stepie());
        dcsr.set_ebreaku(true);
        assert!(dcsr.ebreaku());
        dcsr.set_ebreaks(true);
        assert!(dcsr.ebreaks());
        dcsr.set_ebreakm(true);
        assert!(dcsr.ebreakm());

        dcsr.set_step(false);
        assert!(!dcsr.step());
        dcsr.set_mprven(false);
        assert!(!dcsr.mprven());
        dcsr.set_stoptime(false);
        assert!(!dcsr.stoptime());
        dcsr.set_stopcount(false);
        assert!(!dcsr.stopcount());
        dcsr.set_stepie(false);
        assert!(!dcsr.stepie());
        dcsr.set_ebreaku(false);
        assert!(!dcsr.ebreaku());
        dcsr.set_ebreaks(false);
        assert!(!dcsr.ebreaks());
        dcsr.set_ebreakm(false);
        assert!(!dcsr.ebreakm());
    }

    #[test]
    fn test_dcsr_enums() {
        let mut dcsr = Dcsr::from_bits(0);

        [
            Cause::None,
            Cause::Ebreak,
            Cause::Trigger,
            Cause::HaltRequest,
            Cause::Step,
            Cause::ResetHaltRequest,
        ]
        .into_iter()
        .enumerate()
        .for_each(|(val, variant)| {
            dcsr = Dcsr::from_bits((val as usize) << 6);
            assert_eq!(dcsr.cause(), variant);
            assert_eq!(dcsr.debug_cause(), Ok(variant));
        });

        // invalid variant value 6
        dcsr = Dcsr::from_bits(6 << 6);
        assert_eq!(dcsr.try_cause(), Err(Error::InvalidVariant(6)));
    }

    #[test]
    fn test_dcsr_convenience_methods() {
        let mut dcsr = Dcsr::from_bits(0);

        dcsr.set_privilege_level(Prv::Machine);
        assert_eq!(dcsr.privilege_level().unwrap(), Prv::Machine);
        assert_eq!(dcsr.prv(), Prv::Machine);
    }
}
