//! stvec register

pub use crate::register::mtvec::TrapMode;

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Stvec {
    bits: usize,
}

impl Stvec {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    #[inline]
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    #[inline]
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            _ => None,
        }
    }
}

read_csr_as!(Stvec, 0x105);
write_csr!(0x105);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    _write(addr + mode as usize);
}
