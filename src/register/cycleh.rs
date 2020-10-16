//! cycleh register
//! Shadow of mcycleh register (rv32)
//! must have mcounteren::cy bit enabled for use in supervisor mode (if implemented)
//! if supervisor mode is not implemented this register will control user mode access

read_csr_as_usize_rv32!(0xC80, __read_cycleh);