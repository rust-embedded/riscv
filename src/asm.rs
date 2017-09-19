//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline(always)]
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


/// User Level ISA instructions
instruction!(nop, "addi zero, zero, 0");
instruction!(ecall, "ecall");
instruction!(ebreak, "ebreak");
instruction!(fence, "fence iorw, iorw");
instruction!(fencei, "fence.i");

/// Priviledged ISA Instructions
instruction!(wfi, "wfi");
instruction!(uret, "uret");
instruction!(sret, "sret");
instruction!(mret, "mret");
instruction!(sfencevma, "sfence.vma");
