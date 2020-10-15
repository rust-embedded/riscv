//! instreth register
//! shadow of minstreth register for user mode
//! mcounteren::ir must be enabled to use in user mode

read_csr_as_usize!(0xC82, __read_instreth);
