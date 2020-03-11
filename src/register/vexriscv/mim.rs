//! vexriscv mim register -- machine irq mask

read_csr_as_usize!(0xBC0, __read_vmim);
write_csr_as_usize!(0xBC0, __write_vmim);
