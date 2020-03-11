//! VexRiscv CSRs
//!
//! [VexRiscv](https://github.com/SpinalHDL/VexRiscv) is a RISC-V softcore
//! written in Scala.  It is highly configurable, and can be built with features
//! such as a dcache and an external interrupt controller.
//!
//! These features use vendor-specific CSRs, which are available using this
//! module.
pub mod dci;
pub mod mim;
pub mod mip;
pub mod sim;
pub mod sip;
