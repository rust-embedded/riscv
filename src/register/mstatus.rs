//! mstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

/// Machine Previous Privilege Mode
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Mstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits & (1 << 3) == 1 << 3
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) == (1 << 8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        match (self.bits & (0b11 << 11)) >> 11 {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }
}


read_csr_as!(Mstatus, 0x300, __read_mstatus);
set!(0x300, __set_mstatus);
clear!(0x300, __clear_mstatus);

/// User Interrupt Enable
set_clear_csr!(set_uie, clear_uie, 1 << 0);
/// Supervisor Interrupt Enable
set_clear_csr!(set_sie, clear_sie, 1 << 1);
/// Machine Interrupt Enable
set_clear_csr!(set_mie, clear_mie, 1 << 3);
/// User Previous Interrupt Enable
set_csr!(set_upie, 1 << 4);
/// Supervisor Previous Interrupt Enable
set_csr!(set_spie, 1 << 5);
/// Machine Previous Interrupt Enable
set_csr!(set_mpie, 1 << 7);
/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    _set((spp as usize) << 8);
}
/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    _set((mpp as usize) << 11);
}
