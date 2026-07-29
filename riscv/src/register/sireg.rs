//! 'sireg' register (Supervisor Indirect Register Alias)
//!
//! CSR address: 0x151
//!
//! The `sireg` register provides access to an indirect register
//! selected by `siselect`.

const MASK: usize = usize::MAX;

read_write_csr! {
    /// `sireg` register.
    Sireg: 0x151,
    mask: MASK
}
