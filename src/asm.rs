//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr, $asm_fn:ident) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(all(riscv, feature = "inline-asm"))]
                () => asm!($asm :::: "volatile"),

                #[cfg(all(riscv, not(feature = "inline-asm")))]
                () => {
                    extern "C" {
                        fn $asm_fn();
                    }

                    $asm_fn();
                }

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    )
}


/// Priviledged ISA Instructions
instruction!(ebreak, "ebreak", __ebreak);
instruction!(wfi, "wfi", __wfi);
instruction!(sfence_vma_all, "sfence.vma", __sfence_vma_all);


#[inline]
#[allow(unused_variables)]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr) :: "volatile"),

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __sfence_vma(asid: usize, addr: usize);
            }

            __sfence_vma(asid, addr);
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}
