//! Interrupt handling for targets that comply with the RISC-V interrupt handling standard.
//!
//! In direct mode (i.e., `v-trap` feature disabled), interrupt dispatching is performed by the
//! [`_dispatch_core_interrupt`] function. This function is called by the [crate::start_trap_rust]
//! whenever an interrupt is triggered. This approach relies on the [`__CORE_INTERRUPTS`] array,
//! which sorts all the interrupt handlers depending on their corresponding interrupt source code.
//!
//! In vectored mode (i.e., `v-trap` feature enabled), interrupt dispatching is handled by hardware.
//! To support this mode, we provide inline assembly code that defines the interrupt vector table.
//! Since the alignment constraint of this vector table is implementation-specific, it can be
//! changed by setting the `RISCV_MTVEC_ALIGN` environment variable (the default is 4).
//!
//! # Note
//!
//! If your target has custom core interrupt sources, the target PAC might provide equivalent code
//! to adapt for the target needs (and is responsible for any alignment constraint). In this case,
//! you may need to opt out this module. To do so, activate the `no-interrupts` feature of the
//! `riscv-rt` crate.

// In vectored mode, we also must provide a vector table
#[riscv::pac_enum(unsafe CoreInterruptNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Interrupt {
    SupervisorSoft = 1,
    MachineSoft = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
}
