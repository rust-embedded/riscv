//! hstatus register

pub use super::misa::XLEN;

#[cfg(target_arch = "riscv32")]
use crate::bits::bf_extract;
#[cfg(not(target_arch = "riscv32"))]
use crate::bits::{bf_extract, bf_insert};

#[cfg(not(target_arch = "riscv32"))]
read_write_csr! {
    /// hstatus register
    Hstatus: 0x600,
    mask: 0x0000_0003_0073_F3E0,
}

#[cfg(target_arch = "riscv32")]
read_write_csr! {
    /// hstatus register
    Hstatus: 0x600,
    mask: 0x0073F3E0,
}

csr_field_enum! {
    /// Guest Virtual Address
    GVA {
        default: NoTranslation,
        NoTranslation = 0,
        GuestAddress = 1,
    }
}

csr_field_enum! {
    /// Supervisor Previous Virtualization Mode
    SPV {
        default: NoVirtualization,
        NoVirtualization = 0,
        Virtualization = 1,
    }
}

csr_field_enum! {
    /// Supervisor Previous Virtual Privilege
    SPVP {
        default: VirtualUser,
        VirtualUser = 0,
        VirtualSupervisor = 1,
    }
}

csr_field_enum! {
    /// Hypervisor User Mode Handling
    HU {
        default: HandlingOff,
        HandlingOff = 0,
        HandlingOn = 1,
    }
}

csr_field_enum! {
    /// Virtual Trap Virtual Memory
    VTVM {
        default: TrapMemoryDisabled,
        TrapMemoryDisabled = 0,
        TrapMemoryEnabled = 1,
    }
}

csr_field_enum! {
    /// Virtual Timeout Wait
    VTW {
        default: IndefiniteWFI,
        IndefiniteWFI = 0,
        TrapOnWFI = 1,
    }
}

csr_field_enum! {
    /// Virtual Trap SRET
    VTSR {
        default: TrapResumeInU,
        TrapResumeInU = 0,
        TrapCapturedInHS = 1,
    }
}

csr_field_enum! {
    /// Non-instruction-fetch memory endianness
    Endianness {
        default: LittleEndian,
        LittleEndian = 0,
        BigEndian = 1,
    }
}

impl From<bool> for Endianness {
    fn from(val: bool) -> Self {
        match val {
            false => Self::LittleEndian,
            true => Self::BigEndian,
        }
    }
}

read_only_csr_field! {
    Hstatus,
    /// Guest Virtual Address
    gva: 6,
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtualization
    spv: 7,
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtual Privilege
    spvp: 8,
}

read_write_csr_field! {
    Hstatus,
    /// Hypervisor User HLV HLVX HSV Enable
    hu: 9,
}

read_write_csr_field! {
    Hstatus,
    /// Virtual Guest External Interrupt Number
    vgein: [12:17],
}

read_write_csr_field! {
    Hstatus,
    /// Virtual Trap Virtual Memory
    vtvm: 20,
}

read_write_csr_field! {
    Hstatus,
    /// Virtual Timeout Wait
    vtw: 21,
}

read_write_csr_field! {
    Hstatus,
    /// Virtual Trap SRET
    vtsr: 22,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Hstatus,
    /// Hypervisor User Pointer Masking Mode
    hupmm: [48:49],
}

impl Hstatus {
    /// Effective xlen in VS-mode (i.e., `VSXLEN`).
    ///
    /// In RISCV-32, VSXL does not exist, and `VSXLEN` is always [`XLEN::XLEN32`].
    #[inline]
    pub fn vsxl(&self) -> XLEN {
        match () {
            #[cfg(not(target_arch = "riscv32"))]
            () => XLEN::try_from(bf_extract(self.bits, 32, 2)).unwrap_or_default(),
            #[cfg(target_arch = "riscv32")]
            () => XLEN::XLEN32,
        }
    }

    /// Update Effective xlen in VS-mode (i.e., `VSXLEN`).
    ///
    /// Note this updates a previously read [`Hstatus`] value, but does not
    /// affect the hstatus CSR itself.
    ///
    /// # Note
    ///
    /// In RISCV-32, `VSXL` does not exist, and `VSXLEN` is always [`XLEN::XLEN32`].
    #[inline]
    #[cfg(not(target_arch = "riscv32"))]
    pub fn set_vsxl(&mut self, uxl: XLEN) {
        self.bits = bf_insert(self.bits, 32, 2, uxl as usize);
    }

    /// VS-mode non-instruction-fetch memory endianness.
    pub fn vsbe(&self) -> Endianness {
        Endianness::from(bf_extract(self.bits, 5, 1) != 0)
    }

    /// Update VS-mode non-instruction-fetch memory endianness
    ///
    /// Note this updates a previously read [`Hstatus`] value, but does not
    /// affect the hstatus CSR itself. See [`set_vsbe`] to directly update the
    /// CSR.
    #[inline]
    #[cfg(not(target_arch = "riscv32"))]
    pub fn set_vsbe(&mut self, endianness: Endianness) {
        self.bits = bf_insert(self.bits, 5, 1, endianness as usize);
    }
}

/// Bitfield constants for the `hstatus` fields that are implemented manually
/// (rather than via the field macros) because they only exist on 64-bit targets.
///
/// These complement the macro-generated `<FIELD>_SHIFT`/`_WIDTH`/`_MASK`
/// constants for the remaining `hstatus` fields.
/// HUPMM left unimplemented
#[cfg(not(target_arch = "riscv32"))]
impl Hstatus {
    /// Bit shift of the `vsxl` (VS-mode XLEN) field.
    pub const VSXL_SHIFT: usize = 32;
    /// Bit width of the `vsxl` field.
    pub const VSXL_WIDTH: usize = 2;
    /// Field-shifted bitmask of the `vsxl` field.
    pub const VSXL_MASK: usize = ((1 << Self::VSXL_WIDTH) - 1) << Self::VSXL_SHIFT;
}

set!(0x600);
clear!(0x600);

set_clear_csr!(
    /// Virtual Trap Virtual Memory
    , set_vtvm, clear_tvm, 1 << 20);
set_clear_csr!(
    /// Virtual Timeout Wait
    , set_vtw, clear_vtw, 1 << 21);
set_clear_csr!(
    /// Virtual Trap SRET
    , set_vtsr, clear_vtsr, 1 << 22);

/// Set VS-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_vsbe(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 5),
        Endianness::LittleEndian => _clear(1 << 5),
    }
}

/// Supervisor Previous Virtualization Mode
#[inline]
pub unsafe fn set_spv(spv: SPV) {
    match spv {
        SPV::NoVirtualization => _set(1 << 7),
        SPV::Virtualization => _clear(1 << 7),
    }
}

/// Supervisor Previous Virtual Privilege
#[inline]
pub unsafe fn set_spvp(spvp: SPVP) {
    match spvp {
        SPVP::VirtualSupervisor => _set(1 << 8),
        SPVP::VirtualUser => _clear(1 << 8),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hstatus() {
        let mut hstatus = Hstatus { bits: 0 };

        test_csr_field!(hstatus, spv);
        test_csr_field!(hstatus, spvp);
        test_csr_field!(hstatus, hu);
        test_csr_field!(hstatus, vtvm);
        test_csr_field!(hstatus, vtsr);

        #[cfg(not(target_arch = "riscv32"))]
        {
            [XLEN::XLEN32, XLEN::XLEN64, XLEN::XLEN128]
                .into_iter()
                .for_each(|xlen| {
                    let mut h = Hstatus::from_bits(0);
                    h.set_vsxl(xlen);
                    assert_eq!(h.vsxl(), xlen);
                });

            [Endianness::LittleEndian, Endianness::BigEndian]
                .into_iter()
                .for_each(|endianness| {
                    let mut h = Hstatus::from_bits(0);
                    h.set_vsbe(endianness);
                    assert_eq!(h.vsbe(), endianness);
                });
        }
    }
}
