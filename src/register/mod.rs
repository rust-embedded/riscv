//! RISC-V CSR's
//!
//! The following registers are not available on 64-bit implementations.
//!
//! - cycleh
//! - timeh
//! - instreth
//! - hpmcounter[3-31]h
//! - mcycleh
//! - minstreth
//! - mhpmcounter[3-31]h

#[macro_use]
mod macros;

// TODO: User Trap Setup


// TODO: User Trap Handling


// User Floating-Point CSRs
// TODO: frm, fflags
pub mod fcsr;


// User Counter/Timers
// TODO: cycle[h], instret[h], hpmcounter*[h]
pub mod time;
pub mod timeh;


// Supervisor Trap Setup
// TODO: sedeleg, sideleg
pub mod sstatus;
pub mod sie;
pub mod stvec;
// TODO: scounteren


// Supervisor Trap Handling
pub mod sscratch;
pub mod sepc;
pub mod scause;
pub mod stval;
pub mod sip;


// Supervisor Protection and Translation
pub mod satp;


// Machine Information Registers
pub mod mvendorid;
pub mod marchid;
pub mod mimpid;
pub mod mhartid;


// Machine Trap Setup
pub mod mstatus;
pub mod misa;
// TODO: medeleg, mideleg
pub mod mie;
pub mod mtvec;
// TODO: mcounteren


// Machine Trap Handling
pub mod mscratch;
pub mod mepc;
pub mod mcause;
pub mod mtval;
pub mod mip;


// TODO: Machine Protection and Translation

// Machine Counter/Timers
pub mod mcycle;
pub mod minstret;
// TODO: mhpmcounter*
pub mod mcycleh;
pub mod minstreth;
// TODO: mhpmcounter*h


// TODO: Machine Counter Setup


// TODO: Debug/Trace Registers (shared with Debug Mode)


// TODO: Debug Mode Registers
