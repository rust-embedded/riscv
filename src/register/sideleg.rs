/*!
    # `sideleg` register

    For systems with both S-mode and the N extension, new CSR `sideleg` is added. It has the same layout as the machine trap delegation register `mideleg`.

    `sideleg` allow S-mode to delegate traps to U-mode. Only bits corresponding to traps that have been delegated to S-mode are writable; the others are hardwired to zero. Setting a bit in `sideleg` delegates the corresponding trap in U-mode to the U-mode trap handler.

    Since normally interrupts related to S-mode and higher privilege are not delegated to U-mode, currently only bits related to U-mode interrupts are supported.

    Please note that specific hardware implementations may not fully support `sideleg`. Trying to read or write may cause **Illegal Instruction Exception**.
*/

use bit_field::BitField;

/// sideleg register
#[derive(Clone, Copy, Debug)]
pub struct Sideleg {
    bits: usize,
}

impl Sideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Delegate
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Timer Interrupt Delegate
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// User External Interrupt Delegate
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Sideleg, 0x103, __read_sideleg);
set!(0x103, __set_sideleg);
clear!(0x103, __clear_sideleg);

set_clear_csr!(
    /// User Software Interrupt Delegate
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// User Timer Interrupt Delegate
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// User External Interrupt Delegate
    , set_uext, clear_uext, 1 << 8);
