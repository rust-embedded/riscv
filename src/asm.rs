//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(riscv)]
                () => asm!($asm :::: "volatile"),
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
#[cfg(riscv)]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr) :: "volatile");
}

#[inline]
#[cfg(not(riscv))]
pub fn sfence_vma(_asid: usize, _addr: usize) {}
