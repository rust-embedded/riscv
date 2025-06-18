//! Exception handling for targets that comply with the RISC-V exception handling standard.
//!
//! Exception dispatching is performed by the [`_dispatch_exception`] function.
//! This function is called by the [crate::start_trap_rust] whenever an exception is triggered.
//! This approach relies on the [`__EXCEPTIONS`] array, which sorts all the exception handlers
//! depending on their corresponding exception source code.
//!
//! # Note
//!
//! If your target has custom exception sources, the target PAC might provide equivalent
//! code to adapt for the target needs. In this case, you may need to opt out this module.
//! To do so, activate the `no-exceptions` feature of the `riscv-rt` crate.

use crate::TrapFrame;

unsafe extern "C" {
    fn InstructionMisaligned(trap_frame: &TrapFrame);
    fn InstructionFault(trap_frame: &TrapFrame);
    fn IllegalInstruction(trap_frame: &TrapFrame);
    fn Breakpoint(trap_frame: &TrapFrame);
    fn LoadMisaligned(trap_frame: &TrapFrame);
    fn LoadFault(trap_frame: &TrapFrame);
    fn StoreMisaligned(trap_frame: &TrapFrame);
    fn StoreFault(trap_frame: &TrapFrame);
    fn UserEnvCall(trap_frame: &TrapFrame);
    fn SupervisorEnvCall(trap_frame: &TrapFrame);
    fn MachineEnvCall(trap_frame: &TrapFrame);
    fn InstructionPageFault(trap_frame: &TrapFrame);
    fn LoadPageFault(trap_frame: &TrapFrame);
    fn StorePageFault(trap_frame: &TrapFrame);
}

/// Array with all the exception handlers sorted according to their exception source code.
#[unsafe(no_mangle)]
pub static __EXCEPTIONS: [Option<unsafe extern "C" fn(&TrapFrame)>; 16] = [
    Some(InstructionMisaligned),
    Some(InstructionFault),
    Some(IllegalInstruction),
    Some(Breakpoint),
    Some(LoadMisaligned),
    Some(LoadFault),
    Some(StoreMisaligned),
    Some(StoreFault),
    Some(UserEnvCall),
    Some(SupervisorEnvCall),
    None,
    Some(MachineEnvCall),
    Some(InstructionPageFault),
    Some(LoadPageFault),
    None,
    Some(StorePageFault),
];

/// It calls the corresponding exception handler depending on the exception source code.
///
/// # Safety
///
/// This function must be called only from the [`crate::start_trap_rust`] function.
/// Do **NOT** call this function directly.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _dispatch_exception(trap_frame: &TrapFrame, code: usize) {
    unsafe extern "C" {
        unsafe fn ExceptionHandler(trap_frame: &TrapFrame);
    }
    match __EXCEPTIONS.get(code) {
        Some(Some(handler)) => unsafe { handler(trap_frame) },
        _ => unsafe { ExceptionHandler(trap_frame) },
    }
}
