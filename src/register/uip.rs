/*!
    # `uip` register

    `uip` is a read/write register containing information on pending interrupts.

    Please note that `uip` is a subset of `mip` register. Reading any field, or writing any writable field, of `uip` effects a read or write of the homonymous field of `mip`. If S-mode is implemented, the `uip` register is also a subset of the `sip` register.
*/

use bit_field::BitField;

/// uip register
#[derive(Clone, Copy, Debug)]
pub struct Uip {
    bits: usize,
}

impl Uip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Pending
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Timer Interrupt Pending
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// User External Interrupt Pending
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Uip, 0x044, __read_uip);
set!(0x044, __set_mip);
clear!(0x044, __clear_mip);

set_clear_csr!(
    /// User Software Interrupt Pending
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// User Timer Interrupt Pending
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// User External Interrupt Pending
    , set_uext, clear_uext, 1 << 8);
