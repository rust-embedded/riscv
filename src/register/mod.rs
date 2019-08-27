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

// User Trap Setup
// TODO: sedeleg, sideleg
pub mod ustatus;
pub mod uie;
pub mod utvec;

// User Trap Handling
pub mod uscratch;
pub mod uepc;
pub mod ucause;
pub mod utval;
pub mod uip;

// User Floating-Point CSRs
// TODO: frm, fflags
pub mod fcsr;


// User Counter/Timers
// TODO: cycle[h], instret[h]
pub mod time;
mod hpmcounterx;
pub use self::hpmcounterx::*;
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


// Machine Protection and Translation
mod pmpcfgx;
pub use self::pmpcfgx::*;
mod pmpaddrx;
pub use self::pmpaddrx::*;


// Machine Counter/Timers
pub mod mcycle;
pub mod minstret;
mod mhpmcounterx;
pub use self::mhpmcounterx::*;
pub mod mcycleh;
pub mod minstreth;


// Machine Counter Setup
mod mhpmeventx;
pub use self::mhpmeventx::*;


// TODO: Debug/Trace Registers (shared with Debug Mode)


// TODO: Debug Mode Registers
