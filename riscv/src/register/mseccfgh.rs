//! mseccfgh register

use super::mseccfg::PMM;

read_write_csr! {
    /// mseccfgh register
    Mseccfgh: 0x757,
    mask: 0x3,
}

read_write_csr_field! {
    Mseccfgh,
    /// Pointer Masking Machine-mode
    ///
    /// # Note
    ///
    /// Defined in in the [Smmpm](https://github.com/riscv/riscv-j-extension/releases/download/pointer-masking-ratified/pointer-masking-ratified.pdf) extension.
    pmm,
    PMM: [0:1],
}
