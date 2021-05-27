/*!
    # `ustatus` register

    `ustatus` is a read/write register, keeps track of and controls the hart’s current operating state.

    Please note that the UIE and UPIE bits are mirrored in the `mstatus` and `sstatus` registers in the same bit positions. In some implementation, `ustatus` is a subset of `mstatus` like `sstatus`.

    There are some methods like `set_uie` are implemented to form the value that will be written to the register.
*/

// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;

/// ustatus register
#[derive(Clone, Copy, Debug)]
pub struct Ustatus {
    bits: usize,
}

impl Ustatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
    }

    #[inline]
    pub fn set_upie(&mut self, val: bool) {
        self.bits.set_bit(4, val);
    }

    #[inline]
    pub fn set_uie(&mut self, val: bool) {
        self.bits.set_bit(0, val);
    }
}

read_csr_as!(Ustatus, 0x000, __read_ustatus);
write_csr!(0x000, __write_ustatus);
set!(0x000, __set_ustatus);
clear!(0x000, __clear_ustatus);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);

set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
