//! cycle register
//! Shadow of mcycle register
//! must have `scounter::cy` or `mcounteren::cy` bit enabled depending on whether
//! S-mode is implemented or not

read_csr_as_usize!(0xC00, __read_cycle);
read_composite_csr!(super::cycleh::read(), read());
