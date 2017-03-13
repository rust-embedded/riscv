//! Interacting with debugging agent

pub enum Exception {
    // Hardware reason codes
    BranchThroughZero = 0x20000,
    UndefinedInstr = 0x20001,
    SoftwareInterrupt = 0x20002,
    PrefetchAbort = 0x20003,
    DataAbort = 0x20004,
    AddressException = 0x20005,
    IRQ = 0x20006,
    FIQ = 0x20007,
    // Software reason codes
    BreakPoint = 0x20020,
    WatchPoint = 0x20021,
    StepComplete = 0x20022,
    RunTimeErrorUnknown = 0x20023,
    InternalError = 0x20024,
    UserInterruption = 0x20025,
    ApplicationExit = 0x20026,
    StackOverflow = 0x20027,
    DivisionByZero = 0x20028,
    OSSpecific = 0x20029,
}

/// Reports to the debugger that the execution has completed.
///
/// If `status` is not 0 then an error is reported.
/// This call may not return.
///
pub fn exit(status: i8) {
    if status == 0 {
        report_exception(Exception::ApplicationExit);
    } else {
        report_exception(Exception::RunTimeErrorUnknown);
    }
}

/// Report an exception to the debugger directly.
///
/// This call may not return.
///
/// # Arguments
///
/// * `reason` - A reason code reported back to the debugger
///
pub fn report_exception(reason: Exception) {
    let code = reason as usize;
    unsafe {
        syscall!(REPORT_EXCEPTION, code);
    }
}
