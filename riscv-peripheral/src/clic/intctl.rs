//! CLIC interrupt control register.

/// CLIC interrupt control register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTCTL {
    ptr: *mut u8,
}

impl INTCTL {
    /// Creates a new interrupt control register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt control register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }
}
