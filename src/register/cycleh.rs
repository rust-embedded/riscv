//! cycleh register
//! Shadow of mcycleh register that can be read in user mode
//! must have mcounteren::cy bit enabled for user mode access

read_csr_as_usize_rv32!(0xC80, __read_cycleh);