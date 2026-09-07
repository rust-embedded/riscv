//! stimecmp register

read_csr_as_usize!(0x14D);
write_csr_as_usize!(0x14D);
read_composite_csr!(super::stimecmph::read(), read());
write_composite_csr!(super::stimecmph::write, write);
