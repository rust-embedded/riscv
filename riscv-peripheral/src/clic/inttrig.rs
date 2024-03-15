//! CLIC interrupt trigger register.

/// CLIC interrupt trigger register.
///
/// Optional interrupt triggers (clicinttrig[i]) are used to generate a breakpoint exception,
/// entry into Debug Mode, or a trace action. If these registers are not implemented, they
/// appear as hard-wired zeros.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct INTTRIG {
    ptr: *mut u32,
}

impl INTTRIG {
    /// Creates a new interrupt trigger register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid interrupt trigger register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }
}
