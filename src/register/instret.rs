//! instret register
//! Shadow of minstret register
//! must have mcounteren::ir bit enabled for use in supervisor mode (if implemented)
//! if supervisor mode is not implemented this register will control user mode access

read_csr_as_usize!(0xC02, __read_instret);
read_composite_csr!(super::instreth::read(), read());