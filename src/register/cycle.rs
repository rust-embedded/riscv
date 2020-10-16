//! cycle register
//! Shadow of mcycle register
//! must have mcounteren::cy bit enabled for use in supervisor mode (if implemented)
//! if supervisor mode is not implemented this register will control user mode access

read_csr_as_usize!(0xC00, __read_cycle);
read_composite_csr!(super::cycleh::read(), read());