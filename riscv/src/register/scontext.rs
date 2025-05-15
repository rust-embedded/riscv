//! `scontext` register.

#[cfg(target_arch = "riscv32")]
const MASK: usize = 0xffff;
#[cfg(not(target_arch = "riscv32"))]
const MASK: usize = 0xffff_ffff;

read_write_csr! {
    /// `scontext` register.
    Scontext: 0x5a8,
    mask: MASK,
}

set!(0x5a8);
clear!(0x5a8);

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Scontext,
    /// Represents the `data` context number of the `scontext` CSR.
    data: [0:15],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Scontext,
    /// Represents the `data` context number of the `scontext` CSR.
    data: [0:31],
}
