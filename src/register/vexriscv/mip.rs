//! vexriscv mip register -- machine irq pending

read_csr_as_usize!(0xFC0, __read_vmip);
