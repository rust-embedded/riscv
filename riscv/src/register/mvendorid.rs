//! mvendorid register

read_only_csr! {
    /// `mvendorid` register
    Mvendorid: 0xF11,
    mask: 0xffff_ffff,
    sentinel: 0,
}

read_only_csr_field! {
    Mvendorid,
    /// Represents the number of continuation bytes (`0x7f`) in the JEDEC manufacturer ID.
    bank: [7:31],
}

read_only_csr_field! {
    Mvendorid,
    /// Represents the final offset field in the JEDEC manufacturer ID.
    ///
    /// # Note
    ///
    /// The encoded value returned by `offset` does not include the odd parity bit (`0x80`).
    offset: [0:6],
}
