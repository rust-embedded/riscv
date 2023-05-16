use crate::peripheral::common::{peripheral_reg, Reg, WARL};

/// SSWI register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSWI {
    pub setssip0: SETSSIP,
}

impl SSWI {
    pub const fn new(address: usize) -> Self {
        Self {
            setssip0: SETSSIP::new(address),
        }
    }

    pub unsafe fn setssip(&self, hart_id: u16) -> SETSSIP {
        assert!(hart_id < 4095); // maximum number of HARTs allowed
        SETSSIP::new(self.setssip0.ptr.offset(hart_id as _) as _)
    }
}

peripheral_reg!(SETSSIP, u32, WARL);

impl SETSSIP {
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
