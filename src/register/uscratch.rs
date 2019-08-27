//! uscratch register

read_csr_as_usize!(0x040, __read_mscratch);
write_csr_as_usize!(0x040, __write_mscratch);
