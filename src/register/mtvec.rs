//! mtvec register

/// mtvec register
#[derive(Clone, Copy, Debug)]
pub struct Mtvec {
    bits: usize,
}

/// Trap mode
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Mtvec {
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

/// Reads the CSR
#[inline]
pub fn read() -> Mtvec {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x305, x0" : "=r"(r) ::: "volatile");
            }
            Mtvec { bits: r }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}

/// Writes the CSR
#[cfg_attr(not(any(target_arch = "riscv32", target_arch = "riscv64")), allow(unused_variables))]
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits = addr + mode as usize;
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => asm!("csrrw x0, 0x305, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}
