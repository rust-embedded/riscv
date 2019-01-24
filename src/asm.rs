//! Assembly instructions

macro_rules! instruction {
    ($(#[$attr:meta])*, $fnname:ident, $asm:expr, $asm_fn:ident) => (
        $(#[$attr])*
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


instruction!(
    /// `EBREAK` instruction wrapper
    ///
    /// Generates a breakpoint exception.
    , ebreak, "ebreak", __ebreak);
instruction!(
    /// `WFI` instruction wrapper
    ///
    /// Provides a hint to the implementation that the current hart can be stalled until an interrupt might need servicing.
    /// The WFI instruction is just a hint, and a legal implementation is to implement WFI as a NOP.
    , wfi, "wfi", __wfi);
instruction!(
    /// `SFENCE.VMA` instruction wrapper (all address spaces and page table levels)
    ///
    /// Synchronizes updates to in-memory memory-management data structures with current execution.
    /// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
    /// are ordinarily not ordered with respect to loads and stores in the instruction stream.
    /// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
    /// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
    , sfence_vma_all, "sfence.vma", __sfence_vma_all);


/// `SFENCE.VMA` instruction wrapper
///
/// Synchronizes updates to in-memory memory-management data structures with current execution.
/// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
/// are ordinarily not ordered with respect to loads and stores in the instruction stream.
/// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
/// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
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
