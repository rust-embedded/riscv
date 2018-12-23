//! stvec register

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Stvec {
    bits: usize,
}

/// Trap mode
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Stvec {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    pub fn trap_mode(&self) -> TrapMode {
        let mode = self.bits & 0b11;
        match mode {
            0 => TrapMode::Direct,
            1 => TrapMode::Vectored,
            _ => unimplemented!()
        }
    }
}

read_csr_as!(Stvec, 0x105, __read_stvec);
write_csr!(0x105, __write_stvec);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    _write(addr + mode as usize);
}
