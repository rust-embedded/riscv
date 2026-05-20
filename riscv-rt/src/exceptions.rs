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
//! To do so, activate the `custom-exceptions` feature of the `riscv-rt` crate.

#[riscv::pac_enum(unsafe ExceptionNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(dead_code)] // otherwise compiler complains about Exception not being used
enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    MachineEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}
