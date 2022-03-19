//! mtvec register

/// mtvec register
#[derive(Clone, Copy, Debug)]
pub struct Mtvec {
    bits: usize,
}

/// Trap mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Mtvec {
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

read_csr_as!(Mtvec, 0x305);

write_csr!(0x305);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits = addr + mode as usize;
    _write(bits);
}
