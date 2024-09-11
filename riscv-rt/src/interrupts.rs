//! Interrupt handling for targets that comply with the RISC-V interrupt handling standard.
//!
//! In direct mode (i.e., `v-trap` feature disabled), interrupt dispatching is performed by the
//! [`_dispatch_core_interrupt`] function. This function is called by the [crate::start_trap_rust]
//! whenever an interrupt is triggered. This approach relies on the [`__CORE_INTERRUPTS`] array,
//! which sorts all the interrupt handlers depending on their corresponding interrupt source code.
//!
//! In vectored mode (i.e., `v-trap` feature enabled), interrupt dispatching is handled by hardware.
//! To support this mode, we provide inline assembly code that defines the interrupt vector table.
//!
//! # Note
//!
//! If your target has custom core interrupt sources, the target PAC might provide equivalent
//! code to adapt for the target needs. In this case, you may need to opt out this module.
//! To do so, activate the `no-interrupts` feature of the `riscv-rt` crate.

#[cfg(not(feature = "v-trap"))]
extern "C" {
    fn SupervisorSoft();
    fn MachineSoft();
    fn SupervisorTimer();
    fn MachineTimer();
    fn SupervisorExternal();
    fn MachineExternal();
}

/// Array with all the core interrupt handlers sorted according to their interrupt source code.
///
/// # Note
///
/// This array is necessary only in direct mode (i.e., `v-trap` feature disabled).
#[cfg(not(feature = "v-trap"))]
#[no_mangle]
pub static __CORE_INTERRUPTS: [Option<unsafe extern "C" fn()>; 12] = [
    None,
    Some(SupervisorSoft),
    None,
    Some(MachineSoft),
    None,
    Some(SupervisorTimer),
    None,
    Some(MachineTimer),
    None,
    Some(SupervisorExternal),
    None,
    Some(MachineExternal),
];

/// It calls the corresponding interrupt handler depending on the interrupt source code.
///
/// # Note
///
/// This function is only required in direct mode (i.e., `v-trap` feature disabled).
/// In vectored mode, interrupt handler dispatching is performed directly by hardware.
///
/// # Safety
///
/// This function must be called only from the [`crate::start_trap_rust`] function.
/// Do **NOT** call this function directly.
#[cfg(not(feature = "v-trap"))]
#[inline]
#[no_mangle]
pub unsafe extern "C" fn _dispatch_core_interrupt(code: usize) {
    extern "C" {
        fn DefaultHandler();
    }
    match __CORE_INTERRUPTS.get(code) {
        Some(Some(handler)) => handler(),
        _ => DefaultHandler(),
    }
}

// In vectored mode, we also must provide a vector table
#[cfg(all(riscv, feature = "v-trap"))]
core::arch::global_asm!(
    r#" .section .trap, "ax"
        .weak _vector_table
        .type _vector_table, @function
        
        .option push
        .balign 0x4 // TODO check if this is the correct alignment
        .option norelax
        .option norvc
        
        _vector_table:
            j _start_trap                     // Interrupt 0 is used for exceptions
            j _start_SupervisorSoft_trap
            j _start_DefaultHandler_trap      // Interrupt 2 is reserved
            j _start_MachineSoft_trap
            j _start_DefaultHandler_trap      // Interrupt 4 is reserved
            j _start_SupervisorTimer_trap
            j _start_DefaultHandler_trap      // Interrupt 6 is reserved
            j _start_MachineTimer_trap
            j _start_DefaultHandler_trap      // Interrupt 8 is reserved
            j _start_SupervisorExternal_trap
            j _start_DefaultHandler_trap      // Interrupt 10 is reserved
            j _start_MachineExternal_trap
        
        .option pop"#
);
