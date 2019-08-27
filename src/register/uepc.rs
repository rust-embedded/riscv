//! uepc register

read_csr_as_usize!(0x041, __read_mepc);
write_csr_as_usize!(0x041, __write_mepc);
