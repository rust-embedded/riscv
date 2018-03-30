//! mepc register

/// Reads the CSR
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x341, x0" : "=r"(r) ::: "volatile");
            }
            r as u32
        },
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}
