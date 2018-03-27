//! mcycle register

/// Reads the CSR
#[inline]
pub fn read() -> usize {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0xB00, x0" : "=r"(r) ::: "volatile");
            }
            r
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}
