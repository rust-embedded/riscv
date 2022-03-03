//! Assembly instructions

macro_rules! instruction {
    ($(#[$attr:meta])*, $fnname:ident, $asm:expr, $asm_fn:ident) => (
        $(#[$attr])*
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(all(riscv, feature = "inline-asm"))]
                () => core::arch::asm!($asm),

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
    /// `nop` instruction wrapper
    ///
    /// Generates a no-operation.  Useful to prevent delay loops from being optimized away.
    , nop, "nop", __nop);
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
        () => core::arch::asm!("sfence.vma {0}, {1}", in(reg) addr, in(reg) asid),

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __sfence_vma(addr: usize, asid: usize);
            }

            __sfence_vma(addr, asid);
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Blocks the program for *at least* `cycles` CPU cycles.
///
/// This is implemented in assembly so its execution time is independent of the optimization
/// level, however it is dependent on the specific architecture and core configuration.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution
/// and the execution time may vary with other factors. This delay is mainly useful for simple
/// timer-less initialization of peripherals if and only if accurate timing is not essential. In
/// any other case please use a more accurate method to produce a delay.
#[inline]
#[allow(unused_variables)]
pub unsafe fn delay(cycles: u32) {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => {
            let real_cyc = 1 + cycles / 2;
            core::arch::asm!(
            "1:",
            "addi {0}, {0}, -1",
            "bne {0}, zero, 1b",
            in(reg) real_cyc
            )
        }

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __delay(cycles: u32);
            }

            __delay(cycles);
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}
