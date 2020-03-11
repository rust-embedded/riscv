//! vexriscv dci register -- dcache info
//!
//! This register is only available if the core was built with
//! `DBusCachedPlugin` enabled and `csrInfo` set to `true`.
//!
//! See
//! [DBusCachedPlugin.scala](https://github.com/SpinalHDL/VexRiscv/blob/95237b23ea2d658cb3e0aa77680ca2851ef5d882/src/main/scala/vexriscv/plugin/DBusCachedPlugin.scala#L358)
//! for more information.

read_csr_as_usize!(0xCC0, __read_vdci);
