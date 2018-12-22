//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(all(riscv, feature = "inline-asm"))]
                () => asm!($asm :::: "volatile"),

                #[cfg(all(riscv, not(feature = "inline-asm")))]
                () => unimplemented!(),

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    )
}


/// Priviledged ISA Instructions
instruction!(ebreak, "ebreak");
instruction!(wfi, "wfi");
instruction!(sfence_vma_all, "sfence.vma");


#[inline]
#[allow(unused_variables)]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr) :: "volatile"),

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => unimplemented!(),

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}
