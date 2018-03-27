//! mvendorid register

/// mvendorid register
#[derive(Clone, Copy, Debug)]
pub struct Mvendorid {
    bits: usize,
}

impl Mvendorid {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the JEDEC manufacturer ID
    pub fn jedec_manufacturer(&self) -> usize {
        self.bits >> 7
    }
}

/// Reads the CSR
#[inline]
pub fn read() -> Option<Mvendorid> {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0xF11, x0" : "=r"(r) ::: "volatile");
            }
            // When mvendorid is hardwired to zero it means that the mvendorid
            // csr isn't implemented.
            if r == 0 {
                None
            } else {
                Some(Mvendorid { bits: r })
            }
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}
