//! cycle register
//! Shadow of mcycle register that can be read in user mode
//! must have mcounteren::cy bit enabled for user mode access

read_csr_as_usize!(0xC00, __read_cycle);
read_composite_csr!(super::cycleh::read(), read());