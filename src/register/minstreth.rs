//! minstreth register

/// Reads the CSR
#[inline]
pub fn read() -> usize {
    match () {
        #[cfg(all(target_arch = "riscv", target_pointer_width = "32"))]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0xB82, x0" : "=r"(r) ::: "volatile");
            }
            r
        },
        #[cfg(any(not(target_arch = "riscv"), not(target_pointer_width = "32")))]
        () => unimplemented!(),
    }
}
