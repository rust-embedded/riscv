//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => asm!($asm :::: "volatile"),
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => {}
            }
        }
    )
}


/// Priviledged ISA Instructions
instruction!(ebreak, "ebreak");
instruction!(wfi, "wfi");
