use crate::peripheral::common::{peripheral_reg, Reg, WARL};

/// MSWI register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSWI {
    pub msip0: MSIP,
}

impl MSWI {
    pub const fn new(address: usize) -> Self {
        Self {
            msip0: MSIP::new(address),
        }
    }

    pub unsafe fn msip(&self, hart_id: u16) -> MSIP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        MSIP::new(self.msip0.ptr.offset(hart_id as _) as _)
    }
}

peripheral_reg!(MSIP, u32, WARL);

impl MSIP {
    pub unsafe fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    pub unsafe fn pend(self) {
        self.register.write(1);
    }

    pub unsafe fn unpend(self) {
        self.register.write(0);
    }
}
