//! misa register

use core::num::NonZeroUsize;

/// misa register
#[derive(Clone, Copy, Debug)]
pub struct Misa {
    bits: NonZeroUsize,
}

/// Base integer ISA width
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XLEN {
    XLEN32,
    XLEN64,
    XLEN128,
}

impl XLEN {
    /// Converts a number into an ISA width
    pub(crate) fn from(value: u8) -> Self {
        match value {
            1 => XLEN::XLEN32,
            2 => XLEN::XLEN64,
            3 => XLEN::XLEN128,
            _ => unreachable!(),
        }
    }
}

impl Misa {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits.get()
    }

    /// Effective xlen in M-mode (i.e., `MXLEN`).
    #[inline]
    pub fn mxl(&self) -> XLEN {
        let value = (self.bits() >> (usize::BITS - 2)) as u8;
        XLEN::from(value)
    }

    /// Returns true when a given extension is implemented.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let misa = unsafe { riscv::register::misa::read() }.unwrap();
    /// assert!(misa.has_extension('A')); // panics if atomic extension is not implemented
    /// ```
    #[inline]
    pub fn has_extension(&self, extension: char) -> bool {
        let bit = extension as u8 - 65;
        if bit > 25 {
            return false;
        }
        self.bits() & (1 << bit) == (1 << bit)
    }
}

read_csr!(0x301);

/// Reads the CSR
#[inline]
pub fn read() -> Option<Misa> {
    let r = unsafe { _read() };
    // When misa is hardwired to zero it means that the misa csr
    // isn't implemented.
    NonZeroUsize::new(r).map(|bits| Misa { bits })
}
