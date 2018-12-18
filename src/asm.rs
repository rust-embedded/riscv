//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => asm!($asm :::: "volatile"),
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => unimplemented!(),
            }
        }
    )
}


/// Priviledged ISA Instructions
instruction!(ecall, "ecall");
instruction!(ebreak, "ebreak");
instruction!(uret, "uret");
instruction!(sret, "sret");
instruction!(mret, "mret");
instruction!(wfi, "wfi");
