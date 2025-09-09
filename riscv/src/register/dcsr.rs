//! dcsr register — Debug Control and Status Register (0x7b0)
//!
//! Provides control and status for debug mode, including cause of entry, step control, and privilege level.

read_write_csr! {
    /// Debug Control and Status Register
    Dcsr: 0x7b0,
    mask: 0xffff_ffff,
}

read_write_csr_field! {
    Dcsr,
    /// Previous privilege level when entering debug mode (bits 0..2)
    prv: [0:1],
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
    cause: [6:8],
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

/// Cause for entering debug mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcsrCause {
    None = 0,
    Ebreak = 1,
    Trigger = 2,
    HaltRequest = 3,
    Step = 4,
    ResetHaltRequest = 5,
}

impl DcsrCause {
    pub fn from_usize(val: usize) -> Result<Self, usize> {
        match val {
            0 => Ok(Self::None),
            1 => Ok(Self::Ebreak),
            2 => Ok(Self::Trigger),
            3 => Ok(Self::HaltRequest),
            4 => Ok(Self::Step),
            5 => Ok(Self::ResetHaltRequest),
            other => Err(other),
        }
    }
}

/// Previous privilege level when entering debug mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcsrPrv {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

impl DcsrPrv {
    pub fn from_usize(val: usize) -> Result<Self, usize> {
        match val {
            0 => Ok(Self::User),
            1 => Ok(Self::Supervisor),
            3 => Ok(Self::Machine),
            other => Err(other),
        }
    }
}

impl Dcsr {
    /// Returns the debug cause as an enum
    pub fn debug_cause(&self) -> Result<DcsrCause, usize> {
        DcsrCause::from_usize(self.cause())
    }

    /// Returns the previous privilege level as an enum
    pub fn privilege_level(&self) -> Result<DcsrPrv, usize> {
        DcsrPrv::from_usize(self.prv())
    }

    /// Sets the previous privilege level
    pub fn set_privilege_level(&mut self, level: DcsrPrv) {
        self.set_prv(level as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(DcsrCause::from_usize(0).unwrap(), DcsrCause::None);
        assert_eq!(DcsrCause::from_usize(1).unwrap(), DcsrCause::Ebreak);
        assert_eq!(DcsrCause::from_usize(2).unwrap(), DcsrCause::Trigger);
        assert_eq!(DcsrCause::from_usize(3).unwrap(), DcsrCause::HaltRequest);
        assert_eq!(DcsrCause::from_usize(4).unwrap(), DcsrCause::Step);
        assert_eq!(
            DcsrCause::from_usize(5).unwrap(),
            DcsrCause::ResetHaltRequest
        );
        assert!(DcsrCause::from_usize(6).is_err());

        assert_eq!(DcsrPrv::from_usize(0).unwrap(), DcsrPrv::User);
        assert_eq!(DcsrPrv::from_usize(1).unwrap(), DcsrPrv::Supervisor);
        assert_eq!(DcsrPrv::from_usize(3).unwrap(), DcsrPrv::Machine);
        assert!(DcsrPrv::from_usize(2).is_err());
    }

    #[test]
    fn test_dcsr_convenience_methods() {
        let mut dcsr = Dcsr::from_bits(0);

        dcsr.set_privilege_level(DcsrPrv::Machine);
        assert_eq!(dcsr.privilege_level().unwrap(), DcsrPrv::Machine);
        assert_eq!(dcsr.prv(), 3);
    }
}
