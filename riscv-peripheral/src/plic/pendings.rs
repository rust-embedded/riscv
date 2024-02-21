//! Interrupt pending bits register.

use crate::common::{Reg, RO};
use riscv_pac::ExternalInterruptNumber;

/// Interrupts pending bits register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PENDINGS {
    ptr: *mut u32,
}

impl PENDINGS {
    /// Creates a new Interrupts pending bits register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts pending bits register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn address(self) -> usize {
        self.ptr as _
    }

    /// Checks if an interrupt triggered by a given source is pending.
    #[inline]
    pub fn is_pending<I: ExternalInterruptNumber>(self, source: I) -> bool {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RO> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.read_bit(source % u32::BITS as usize)
    }
}

#[cfg(test)]
mod test {
    use super::super::test::Interrupt;
    use super::*;

    #[test]
    fn test_pendings() {
        // slice to emulate the interrupt pendings register
        let mut raw_reg = [0u32; 32];
        // SAFETY: valid memory address
        let pendings = unsafe { PENDINGS::new(raw_reg.as_mut_ptr() as _) };

        for i in 0..255 {
            // SAFETY: valid memory address
            unsafe { raw_reg.as_mut_ptr().write_volatile(i) };
            assert_eq!(pendings.is_pending(Interrupt::I1), i & 0x2 != 0);
            assert_eq!(pendings.is_pending(Interrupt::I2), i & 0x4 != 0);
            assert_eq!(pendings.is_pending(Interrupt::I3), i & 0x8 != 0);
            assert_eq!(pendings.is_pending(Interrupt::I4), i & 0x10 != 0);
        }
    }
}
