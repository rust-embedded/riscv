use core::arch::global_asm;

/// Parse cfg attributes inside a global_asm call.
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] { $($y:tt)* } $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)*], $($y)* $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// Provisional patch to avoid LLVM spurious errors when compiling in release mode.
// This patch is somewhat hardcoded and relies on the fact that the rustc compiler
// only supports a limited combination of ISA extensions. This patch should be
// removed when LLVM fixes the issue. Alternatively, it must be updated when rustc
// supports more ISA extension combinations.
//
// Related issues:
// - https://github.com/rust-embedded/riscv/issues/175
// - https://github.com/rust-lang/rust/issues/80608
// - https://github.com/llvm/llvm-project/issues/61991
riscv_rt_macros::llvm_arch_patch!();

// Entry point of all programs (_start). It initializes DWARF call frame information,
// the stack pointer, the frame pointer (needed for closures to work in start_rust)
// and the global pointer. Then it calls _start_rust.
cfg_global_asm!(
    ".section .init, \"ax\"
    .global _start

_start:",
    #[cfg(target_arch = "riscv32")]
    "lui ra, %hi(_abs_start)
     jr %lo(_abs_start)(ra)",
    #[cfg(target_arch = "riscv64")]
    ".option push
    .option norelax // to prevent an unsupported R_RISCV_ALIGN relocation from being generated
1:
    auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  3
1:
    .dword _abs_start
    .option pop",
    "
_abs_start:
    .option norelax
    .cfi_startproc
    .cfi_undefined ra",
    // Disable interrupts
    #[cfg(feature = "s-mode")]
    "csrw sie, 0
    csrw sip, 0",
    #[cfg(not(feature = "s-mode"))]
    "csrw mie, 0
    csrw mip, 0
    csrr a0, mhartid", // Make sure that the hart ID is in a0 in M-mode
    // Set pre-init trap vector
    "la t0, _pre_init_trap",
    #[cfg(feature = "s-mode")]
    "csrw stvec, t0",
    #[cfg(not(feature = "s-mode"))]
    "csrw mtvec, t0",
    // If multi-hart, assert that hart ID is valid
    #[cfg(not(feature = "single-hart"))]
    "lui t0, %hi(_max_hart_id)
    add t0, t0, %lo(_max_hart_id)
    bgeu t0, a0, 1f
    la t0, abort // If hart_id > _max_hart_id, jump to abort
    jr t0
1:", // only valid harts reach this point

// INITIALIZE GLOBAL POINTER, STACK POINTER, AND FRAME POINTER
    ".option push
    .option norelax
    la gp, __global_pointer$
    .option pop",
#[cfg(not(feature = "single-hart"))]
{
    "mv t2, a0
    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)",
    #[cfg(riscvm)]
    "mul t0, t2, t0",
    #[cfg(not(riscvm))]
    "beqz t2, 2f  // skip if hart ID is 0
    mv t1, t0
1:
    add t0, t0, t1
    addi t2, t2, -1
    bnez t2, 1b
2:  ",
}
    "la t1, _stack_start",
    #[cfg(not(feature = "single-hart"))]
    "sub t1, t1, t0",
    "andi sp, t1, -16 // align stack to 16-bytes
    add s0, sp, zero",
// STORE A0..A2 IN THE STACK, AS THEY WILL BE NEEDED LATER BY main
    #[cfg(target_arch = "riscv32")]
    "addi sp, sp, -4 * 4 // we must keep stack aligned to 16-bytes
    sw a0, 4 * 0(sp)
    sw a1, 4 * 1(sp)
    sw a2, 4 * 2(sp)",
    #[cfg(target_arch = "riscv64")]
    "addi sp, sp, -8 * 4 // we must keep stack aligned to 16-bytes
    sd a0, 8 * 0(sp)
    sd a1, 8 * 1(sp)
    sd a2, 8 * 2(sp)",

// CALL __pre_init (IF ENABLED) AND INITIALIZE RAM
    #[cfg(not(feature = "single-hart"))]
    // Skip RAM initialization if current hart is not the boot hart
    "call _mp_hook
    beqz a0, 4f",
    #[cfg(feature = "pre-init")]
    "call __pre_init",
    "// Copy .data from flash to RAM
    la t0, __sdata
    la a0, __edata
    la t1, __sidata
    bgeu t0, a0, 2f
1:  ",
    #[cfg(target_arch = "riscv32")]
    "lw t2, 0(t1)
    addi t1, t1, 4
    sw t2, 0(t0)
    addi t0, t0, 4
    bltu t0, a0, 1b",
    #[cfg(target_arch = "riscv64")]
    "ld t2, 0(t1)
    addi t1, t1, 8
    sd t2, 0(t0)
    addi t0, t0, 8
    bltu t0, a0, 1b",
    "
2:  // Zero out .bss
    la t0, __sbss
    la t2, __ebss
    bgeu  t0, t2, 4f
3:  ",
    #[cfg(target_arch = "riscv32")]
    "sw  zero, 0(t0)
    addi t0, t0, 4
    bltu t0, t2, 3b",
    #[cfg(target_arch = "riscv64")]
    "sd zero, 0(t0)
    addi t0, t0, 8
    bltu t0, t2, 3b",
    "
4: // RAM initialized",

// INITIALIZE FLOATING POINT UNIT
#[cfg(any(riscvf, riscvd))]
{
    "
    li t0, 0x4000 // bit 14 is FS most significant bit
    li t2, 0x2000 // bit 13 is FS least significant bit
    ",
    #[cfg(feature = "s-mode")]
    "csrrc x0, sstatus, t0
    csrrs x0, sstatus, t2",
    #[cfg(not(feature = "s-mode"))]
    "csrrc x0, mstatus, t0
    csrrs x0, mstatus, t2",
    "fscsr x0",
}

// SET UP INTERRUPTS, RESTORE a0..a2, AND JUMP TO MAIN RUST FUNCTION
    "call _setup_interrupts",
    #[cfg(target_arch = "riscv32")]
    "lw a0, 4 * 0(sp)
    lw a1, 4 * 1(sp)
    lw a2, 4 * 2(sp)
    addi sp, sp, 4 * 4",
    #[cfg(target_arch = "riscv64")]
    "ld a0, 8 * 0(sp)
    ld a1, 8 * 1(sp)
    ld a2, 8 * 2(sp)
    addi sp, sp, 8 * 4",
    "la t0, main
    jr t0
    .cfi_endproc",

    #[cfg(not(feature = "single-hart"))]
    // Default implementation of `_mp_hook` wakes hart 0 and busy-loops all the other harts.
    // Users can override this function by defining their own `_mp_hook`.
    // This function is only used when the `single-hart` feature is not enabled.
    ".global _default_mp_hook
_default_mp_hook:
    beqz a0, 2f // if hartid is 0, return true
1:  wfi // Otherwise, wait for interrupt in a loop
    j 1b
2:  li a0, 1
    ret",
    // Default implementation of `_setup_interrupts` sets the trap vector to `_start_trap` in direct mode.
    // In vectored mode, it sets the trap vector to `_vector_table`.
    // Users can override this function by defining their own `_setup_interrupts`
    ".global _default_setup_interrupts
_default_setup_interrupts:",
    #[cfg(not(feature = "v-trap"))]
    "la t0, _start_trap", // _start_trap is 4-byte aligned, so it corresponds to the Direct trap mode
    #[cfg(feature = "v-trap")]
    "la t0, _vector_table
    ori t0, t0, 0x1", // _vector_table is at least 4-byte aligned, so we must set the bit 0 to activate the Vectored trap mode
    #[cfg(feature = "s-mode")]
    "csrw stvec, t0",
    #[cfg(not(feature = "s-mode"))]
    "csrw mtvec, t0",
    "ret",
);

riscv_rt_macros::default_start_trap!();

#[cfg(feature = "v-trap")]
riscv_rt_macros::vectored_interrupt_trap!();

#[rustfmt::skip]
global_asm!(
    ".section .text.abort
.align 4
.global _default_abort
_default_abort:  // make sure there is an abort symbol when linking
    j _default_abort"
);
