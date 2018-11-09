//! timeh register

/// Reads the CSR
#[inline]
pub fn read() -> usize {
    match () {
        #[cfg(target_arch = "riscv32")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0xC81, x0" : "=r"(r) ::: "volatile");
            }
            r
        }
        #[cfg(not(target_arch = "riscv32"))]
        () => unimplemented!(),
    }
}