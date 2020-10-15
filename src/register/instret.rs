//! instret register
//! shadow of minstret register for user mode
//! mcounteren::ir must be enabled to use in user mode

read_csr_as_usize!(0xC02, __read_instret);
read_composite_csr!(super::instreth::read(), read());