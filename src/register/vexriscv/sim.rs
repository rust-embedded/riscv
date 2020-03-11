//! vexriscv sim register -- supervisor irq mask

read_csr_as_usize!(0x9C0, __read_vsim);
write_csr_as_usize!(0x9C0, __write_vsim);
