//! mepc register

/// Reads the CSR
#[inline]
pub fn read() -> usize {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x341, x0" : "=r"(r) ::: "volatile");
            }
            r
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}
