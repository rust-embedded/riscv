use crate::TrapFrame;

extern "C" {
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
    fn ExceptionHandler(trap_frame: &TrapFrame);
}

#[doc(hidden)]
#[no_mangle]
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

#[export_name = "_dispatch_exception"]
#[inline]
unsafe extern "C" fn dispatch_exception(trap_frame: &TrapFrame, code: usize) {
    match __EXCEPTIONS.get(code) {
        Some(Some(handler)) => handler(trap_frame),
        _ => ExceptionHandler(trap_frame),
    }
}
