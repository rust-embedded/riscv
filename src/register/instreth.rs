//! instreth register
//! Shadow of minstreth register (rv32)
//! must have mcounteren::ir bit enabled for use in supervisor mode (if implemented)
//! if supervisor mode is not implemented this register will control user mode access

read_csr_as_usize!(0xC82, __read_instreth);
