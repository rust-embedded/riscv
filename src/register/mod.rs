//! RISC-V CSR's
//!
//! The following registers are not available on 64-bit implementations.
//!
//! - cycleh
//! - timeh
//! - instreth
//! - hpmcounter<3-31>h
//! - mcycleh
//! - minstreth
//! - mhpmcounter<3-31>h

#[macro_use]
mod macros;

// User Trap Setup
pub mod uie;
pub mod ustatus;
pub mod utvec;

// User Trap Handling
pub mod ucause;
pub mod uepc;
pub mod uip;
pub mod uscratch;
pub mod utval;

// User Floating-Point CSRs
// TODO: frm, fflags
pub mod fcsr;

// User Counter/Timers
pub mod cycle;
pub mod cycleh;
mod hpmcounterx;
pub use self::hpmcounterx::*;
pub mod instret;
pub mod instreth;
pub mod time;
pub mod timeh;

// Supervisor Trap Setup
// TODO: sedeleg, sideleg
pub mod scounteren;
pub mod sie;
pub mod sstatus;
pub mod stvec;

// Supervisor Trap Handling
pub mod scause;
pub mod sepc;
pub mod sip;
pub mod sscratch;
pub mod stval;

// Supervisor Protection and Translation
pub mod satp;

// Machine Information Registers
pub mod marchid;
pub mod mhartid;
pub mod mimpid;
pub mod mvendorid;

// Machine Trap Setup
pub mod mcounteren;
pub mod medeleg;
pub mod mideleg;
pub mod mie;
pub mod misa;
pub mod mstatus;
pub mod mtvec;

// Machine Trap Handling
pub mod mcause;
pub mod mepc;
pub mod mip;
pub mod mscratch;
pub mod mtval;

// Machine Protection and Translation
mod pmpcfgx;
pub use self::pmpcfgx::*;
mod pmpaddrx;
pub use self::pmpaddrx::*;

// Machine Counter/Timers
pub mod mcycle;
pub mod mcycleh;
mod mhpmcounterx;
pub use self::mhpmcounterx::*;
pub mod minstret;
pub mod minstreth;

// Machine Counter Setup
mod mhpmeventx;
pub use self::mhpmeventx::*;

// TODO: Debug/Trace Registers (shared with Debug Mode)

// TODO: Debug Mode Registers
