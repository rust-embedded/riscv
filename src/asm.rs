//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub fn $fnname() {
            match () {
                #[cfg(target_arch = "riscv")]
                () => unsafe {
                    asm!($asm :::: "volatile");
                },
                #[cfg(not(target_arch = "riscv"))]
                () => {}
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
