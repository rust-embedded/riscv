//! smclicconfig extension register.

use crate::common::{Reg, RW};

/// smclicconfig register.
///
/// Hardware implementations may wish to have a single implementation support different
/// parameterizations of clic extensions. This extension defines that programmability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SMCLICCONFIG {
    ptr: *mut u32,
}

impl SMCLICCONFIG {
    /// Creates a smclicconfig register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid smclicconfig register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    /// Check how many upper bits in `clicintctl[i]` are assigned to encode the interrupt level at
    /// that privilege level.
    #[inline]
    pub fn mnlbits(self) -> u32 {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.read_bits(0, 3)
    }

    /// Set how many upper bits in `clicintctl[i]` are assigned to encode the interrupt level at
    /// that privilege level.
    #[inline]
    pub fn set_mnlbits(self, mnlbits: u32) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(0, 3, mnlbits)
    }

    /// Check how many bits are physically implemented in `clicintattr[i]`.mode to represent an input
    /// i's privilege mode.
    #[inline]
    pub fn nmbits(self) -> u32 {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.read_bits(4, 5)
    }

    /// Check how many bits are physically implemented in `clicintattr[i]`.mode to represent an input
    /// i's privilege mode.
    #[inline]
    pub fn set_nmbits(self, nmbits: u32) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr) };

        reg.write_bits(4, 5, nmbits)
    }
}
