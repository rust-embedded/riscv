extern "C" {
    fn SupervisorSoft();
    fn MachineSoft();
    fn SupervisorTimer();
    fn MachineTimer();
    fn SupervisorExternal();
    fn MachineExternal();
    fn DefaultHandler();
}

#[doc(hidden)]
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

#[export_name = "_dispatch_core_interrupt"]
#[inline]
unsafe extern "C" fn dispatch_core_interrupt(code: usize) {
    match __CORE_INTERRUPTS.get(code) {
        Some(Some(handler)) => handler(),
        _ => DefaultHandler(),
    }
}

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
