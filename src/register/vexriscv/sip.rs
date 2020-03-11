//! vexriscv sip register -- supervisor irq pending

read_csr_as_usize!(0xDC0, __read_vsip);
