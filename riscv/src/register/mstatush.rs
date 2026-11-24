//! mstatush register (RISCV-32 only)

pub use super::mstatus::Endianness;

read_write_csr! {
    /// mstatush register
    Mstatush: 0x310,
    mask: 0x30,
}

csr_field_enum! {
    /// Machine Previous Virtual Mode
    MPV {
        default: VirtualizationDisabled,
        VirtualizationDisabled = 0,
        VirtualizationEnabled = 1,
    }
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Mstatush,
    /// Machine Previous Virtualization
    mpv: 7,
}

csr_field_enum! {
    /// Guest Virtual Address
    GVA {
        default: NoTranslation,
        NoTranslation = 0,
        GuestAddress = 1,
    }
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Mstatush,
    /// Guest Virtual Address
    gva: 6,
}

read_write_csr_field! {
    Mstatush,
    /// S-mode non-instruction-fetch memory endianness
    sbe,
    Endianness: [4:4],
}

read_write_csr_field! {
    Mstatush,
    /// M-mode non-instruction-fetch memory endianness
    mbe,
    Endianness: [5:5],
}

set_rv32!(0x310);
clear_rv32!(0x310);

/// Set S-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_sbe(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 4),
        Endianness::LittleEndian => _clear(1 << 4),
    }
}

/// Set M-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_mbe(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 5),
        Endianness::LittleEndian => _clear(1 << 5),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mstatush() {
        let mut m = Mstatush::from_bits(0);

        [Endianness::LittleEndian, Endianness::BigEndian]
            .into_iter()
            .for_each(|endianness| {
                test_csr_field!(m, sbe: endianness);
                test_csr_field!(m, mbe: endianness);
            });
    }
}
