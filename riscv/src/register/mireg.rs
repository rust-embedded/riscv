//! 'mireg' register (Machine Indirect Register)
//!
//! CSR address: 0x351
//!
//! The `mireg` register provides access to an indirect register
//! selected by `miselect`.

const MASK: usize = usize::MAX;

read_write_csr! {
    /// `mireg` register.
    Mireg: 0x351,
    mask: MASK
}
