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
instruction!(ebreak, "ebreak");
instruction!(wfi, "wfi");
instruction!(sfence_vma_all, "sfence.vma");


#[inline]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr) :: "volatile");
}

#[inline]
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub fn sfence_vma(_asid: usize, _addr: usize) {}
