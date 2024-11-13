//! mstatus register

pub use super::misa::XLEN;
use crate::bits::{bf_extract, bf_insert};

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

impl From<usize> for Mstatus {
    #[inline]
    fn from(bits: usize) -> Self {
        Self { bits }
    }
}

/// Additional extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XS {
    /// All off
    AllOff = 0,

    /// None dirty or clean, some on
    NoneDirtyOrClean = 1,

    /// None dirty, some clean
    NoneDirtySomeClean = 2,

    /// Some dirty
    SomeDirty = 3,
}

/// Floating-point extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

/// Vector extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

/// Machine Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

/// Non-instruction-fetch memory endianness
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endianness {
    BigEndian = 1,
    LittleEndian = 0,
}

impl From<bool> for Endianness {
    fn from(value: bool) -> Self {
        match value {
            true => Self::BigEndian,
            false => Self::LittleEndian,
        }
    }
}

impl Mstatus {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        bf_extract(self.bits, 1, 1) != 0
    }

    /// Update Supervisor Interrupt Enable
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_sie`]/[`clear_sie`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_sie(&mut self, sie: bool) {
        self.bits = bf_insert(self.bits, 1, 1, sie as usize);
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        bf_extract(self.bits, 3, 1) != 0
    }

    /// Update Machine Interrupt Enable
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_mie`]/[`clear_mie`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_mie(&mut self, mie: bool) {
        self.bits = bf_insert(self.bits, 3, 1, mie as usize);
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        bf_extract(self.bits, 5, 1) != 0
    }

    /// Update Supervisor Previous Interrupt Enable
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_spie`]` to directly update the
    /// CSR.
    #[inline]
    pub fn set_spie(&mut self, spie: bool) {
        self.bits = bf_insert(self.bits, 5, 1, spie as usize);
    }

    /// U-mode non-instruction-fetch memory endianness
    #[inline]
    pub fn ube(&self) -> Endianness {
        Endianness::from(bf_extract(self.bits, 6, 1) != 0)
    }

    /// Update U-mode non-instruction-fetch memory endianness
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_ube`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_ube(&mut self, endianness: Endianness) {
        self.bits = bf_insert(self.bits, 6, 1, endianness as usize);
    }

    /// Machine Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        bf_extract(self.bits, 7, 1) != 0
    }

    /// Update Machine Previous Interrupt Enable
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_mpie`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_mpie(&mut self, mpie: bool) {
        self.bits = bf_insert(self.bits, 7, 1, mpie as usize);
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match bf_extract(self.bits, 8, 1) != 0 {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Update Supervisor Previous Privilege Mode
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_spp`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_spp(&mut self, spp: SPP) {
        self.bits = bf_insert(self.bits, 8, 1, spp as usize);
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        let mpp = bf_extract(self.bits, 11, 2); // bits 11-12
        match mpp {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    /// Update Machine Previous Privilege Mode
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_mpp`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_mpp(&mut self, mpp: MPP) {
        self.bits = bf_insert(self.bits, 11, 2, mpp as usize);
    }

    /// Floating-point extension state
    ///
    /// Encodes the status of the floating-point unit, including the CSR `fcsr`
    /// and floating-point data registers `f0–f31`.
    #[inline]
    pub fn fs(&self) -> FS {
        let fs = bf_extract(self.bits, 13, 2); // bits 13-14
        match fs {
            0b00 => FS::Off,
            0b01 => FS::Initial,
            0b10 => FS::Clean,
            0b11 => FS::Dirty,
            _ => unreachable!(),
        }
    }

    /// Vector extension state
    #[inline]
    pub fn vs(&self) -> VS {
        let fs = bf_extract(self.bits, 9, 2); // bits 9-10
        match fs {
            0b00 => VS::Off,
            0b01 => VS::Initial,
            0b10 => VS::Clean,
            0b11 => VS::Dirty,
            _ => unreachable!(),
        }
    }

    /// Update Floating-point extension state
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_fs`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_fs(&mut self, fs: FS) {
        self.bits = bf_insert(self.bits, 13, 2, fs as usize);
    }

    /// Update vector extension state
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_vs`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_vs(&mut self, vs: VS) {
        self.bits = bf_insert(self.bits, 9, 2, vs as usize);
    }

    /// Additional extension state
    ///
    /// Encodes the status of additional user-mode extensions and associated
    /// state.
    #[inline]
    pub fn xs(&self) -> XS {
        let xs = bf_extract(self.bits, 15, 2); // bits 15-16
        match xs {
            0b00 => XS::AllOff,
            0b01 => XS::NoneDirtyOrClean,
            0b10 => XS::NoneDirtySomeClean,
            0b11 => XS::SomeDirty,
            _ => unreachable!(),
        }
    }

    /// Update Additional extension state
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself.
    #[inline]
    pub fn set_xs(&mut self, xs: XS) {
        self.bits = bf_insert(self.bits, 15, 2, xs as usize);
    }

    /// Modify Memory PRiVilege
    #[inline]
    pub fn mprv(&self) -> bool {
        bf_extract(self.bits, 17, 1) != 0
    }

    /// Update Modify Memory PRiVilege
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_mprv`]/[`clear_mprv`] to
    /// directly update the CSR.
    #[inline]
    pub fn set_mprv(&mut self, mprv: bool) {
        self.bits = bf_insert(self.bits, 17, 1, mprv as usize);
    }

    /// Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        bf_extract(self.bits, 18, 1) != 0
    }

    /// Update Permit Supervisor User Memory access
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_sum`]/[`clear_sum`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_sum(&mut self, sum: bool) {
        self.bits = bf_insert(self.bits, 18, 1, sum as usize);
    }

    /// Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        bf_extract(self.bits, 19, 1) != 0
    }

    /// Update Make eXecutable Readable
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not affect
    /// the mstatus CSR itself. See [`set_mxr`]/[`clear_mxr`] to directly update
    /// the CSR.
    #[inline]
    pub fn set_mxr(&mut self, mxr: bool) {
        self.bits = bf_insert(self.bits, 19, 1, mxr as usize);
    }

    /// Trap Virtual Memory
    ///
    /// If this bit is set, reads or writes to `satp` CSR or execute `sfence.vma`
    /// instruction when in S-mode will raise an illegal instruction exception.
    ///
    /// TVM is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tvm(&self) -> bool {
        bf_extract(self.bits, 20, 1) != 0
    }

    /// Update Trap Virtual Memory
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_tvm`]/[`clear_tvm`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_tvm(&mut self, tvm: bool) {
        self.bits = bf_insert(self.bits, 20, 1, tvm as usize);
    }

    /// Timeout Wait
    ///
    /// Indicates that if WFI instruction should be intercepted.
    ///
    /// If this bit is set, when WFI is executed in S-mode, and it does not complete
    /// within an implementation specific, bounded time limit, the WFI instruction will cause
    /// an illegal instruction trap; or could always cause trap then the time limit is zero.
    ///
    /// TW is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tw(&self) -> bool {
        bf_extract(self.bits, 21, 1) != 0
    }

    /// Update Timeout Wait
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_tw`]/[`clear_tw`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_tw(&mut self, tw: bool) {
        self.bits = bf_insert(self.bits, 21, 1, tw as usize);
    }

    /// Trap SRET
    ///
    /// Indicates that if SRET instruction should be trapped to raise illegal
    /// instruction exception.
    ///
    /// If S-mode is not supported, TSR bit is hard-wired to 0.
    #[inline]
    pub fn tsr(&self) -> bool {
        bf_extract(self.bits, 22, 1) != 0
    }

    /// Update Trap SRET
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_tsr`]/[`clear_tsr`] to directly
    /// update the CSR.
    #[inline]
    pub fn set_tsr(&mut self, tsr: bool) {
        self.bits = bf_insert(self.bits, 22, 1, tsr as usize);
    }

    /// Effective xlen in U-mode (i.e., `UXLEN`).
    ///
    /// In RISCV-32, UXL does not exist, and `UXLEN` is always [`XLEN::XLEN32`].
    #[inline]
    pub fn uxl(&self) -> XLEN {
        match () {
            #[cfg(riscv32)]
            () => XLEN::XLEN32,
            #[cfg(not(riscv32))]
            () => XLEN::try_from(bf_extract(self.bits, 32, 2)).unwrap_or_default(),
        }
    }

    /// Update Effective xlen in U-mode (i.e., `UXLEN`).
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself.
    #[inline]
    pub fn set_uxl(&mut self, uxl: XLEN) {
        #[cfg(not(riscv32))]
        {
            self.bits = bf_insert(self.bits, 32, 2, uxl as usize);
        }
    }

    /// Effective xlen in S-mode (i.e., `SXLEN`).
    ///
    /// In RISCV-32, SXL does not exist, and SXLEN is always [`XLEN::XLEN32`].
    #[inline]
    pub fn sxl(&self) -> XLEN {
        match () {
            #[cfg(riscv32)]
            () => XLEN::XLEN32,
            #[cfg(not(riscv32))]
            () => XLEN::try_from(bf_extract(self.bits, 34, 2)).unwrap_or_default(),
        }
    }

    /// Update Effective xlen in S-mode (i.e., `SXLEN`).
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself.
    #[inline]
    pub fn set_sxl(&mut self, sxl: XLEN) {
        #[cfg(not(riscv32))]
        {
            self.bits = bf_insert(self.bits, 34, 2, sxl as usize);
        }
    }

    /// S-mode non-instruction-fetch memory endianness.
    ///
    /// In RISCV-32, this field is read from the [`crate::register::mstatush`] register.
    pub fn sbe(&self) -> Endianness {
        match () {
            #[cfg(riscv32)]
            () => super::mstatush::read().sbe(),
            #[cfg(not(riscv32))]
            () => Endianness::from(bf_extract(self.bits, 36, 1) != 0),
        }
    }

    /// Update S-mode non-instruction-fetch memory endianness
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_sbe`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_sbe(&mut self, endianness: Endianness) {
        #[cfg(not(riscv32))]
        {
            self.bits = bf_insert(self.bits, 36, 1, endianness as usize);
        }
    }

    /// M-mode non-instruction-fetch memory endianness
    ///
    /// In RISCV-32, this field is read from the [`crate::register::mstatush`] register
    pub fn mbe(&self) -> Endianness {
        match () {
            #[cfg(riscv32)]
            () => super::mstatush::read().mbe(),
            #[cfg(not(riscv32))]
            () => Endianness::from(bf_extract(self.bits, 37, 1) != 0),
        }
    }
    /// Update M-mode non-instruction-fetch memory endianness
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself. See [`set_mbe`] to directly update the
    /// CSR.
    #[inline]
    pub fn set_mbe(&mut self, endianness: Endianness) {
        #[cfg(not(riscv32))]
        {
            self.bits = bf_insert(self.bits, 37, 1, endianness as usize);
        }
    }

    /// Whether either the FS field or XS field signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        bf_extract(self.bits, usize::BITS as usize - 1, 1) != 0
    }

    /// Update whether either the FS field or XS field signals the presence of
    /// some dirty state
    ///
    /// Note this updates a previously read [`Mstatus`] value, but does not
    /// affect the mstatus CSR itself.
    #[inline]
    pub fn set_sd(&mut self, sd: bool) {
        self.bits = bf_insert(self.bits, usize::BITS as usize - 1, 1, sd as usize);
    }
}

read_csr_as!(Mstatus, 0x300);
write_csr_as!(Mstatus, 0x300);
set!(0x300);
clear!(0x300);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);
set_clear_csr!(
    /// Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);
set_clear_csr!(
    /// Machine Interrupt Enable
    , set_mie, clear_mie, 1 << 3);
set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
set_csr!(
    /// Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);
set_csr!(
    /// Machine Previous Interrupt Enable
    , set_mpie, 1 << 7);
set_clear_csr!(
    /// Modify Memory PRiVilege
    , set_mprv, clear_mprv, 1 << 17);
set_clear_csr!(
    /// Permit Supervisor User Memory access
    , set_sum, clear_sum, 1 << 18);
set_clear_csr!(
    /// Make eXecutable Readable
    , set_mxr, clear_mxr, 1 << 19);
set_clear_csr!(
    /// Trap Virtual Memory
    , set_tvm, clear_tvm, 1 << 20);
set_clear_csr!(
    /// Timeout Wait
    , set_tw, clear_tw, 1 << 21);
set_clear_csr!(
    /// Trap SRET
    , set_tsr, clear_tsr, 1 << 22);

/// Set U-mode non-instruction-fetch memory endianness
#[inline]
pub unsafe fn set_ube(endianness: Endianness) {
    match endianness {
        Endianness::BigEndian => _set(1 << 6),
        Endianness::LittleEndian => _clear(1 << 6),
    }
}

/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    match spp {
        SPP::Supervisor => _set(1 << 8),
        SPP::User => _clear(1 << 8),
    }
}

/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    let mut value = _read();
    value &= !(0x3 << 11); // clear previous value
    value |= (mpp as usize) << 11;
    _write(value);
}

/// Floating-point extension state
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value &= !(0x3 << 13); // clear previous value
    value |= (fs as usize) << 13;
    _write(value);
}

/// Vector extension state
#[inline]
pub unsafe fn set_vs(vs: VS) {
    let mut value = _read();
    value &= !(0x3 << 9); // clear previous value
    value |= (vs as usize) << 9;
    _write(value);
}

/// Set S-mode non-instruction-fetch memory endianness
///
/// # Note
///
/// In RISCV-32, this function calls [`crate::register::mstatush::set_sbe`]
#[inline]
pub unsafe fn set_sbe(endianness: Endianness) {
    match () {
        #[cfg(riscv32)]
        () => super::mstatush::set_sbe(endianness),
        #[cfg(not(riscv32))]
        () => match endianness {
            Endianness::BigEndian => _set(1 << 36),
            Endianness::LittleEndian => _clear(1 << 36),
        },
    }
}

/// Set M-mode non-instruction-fetch memory endianness
///
/// # Note
///
/// In RISCV-32, this function calls [`crate::register::mstatush::set_mbe`]
#[inline]
pub unsafe fn set_mbe(endianness: Endianness) {
    match () {
        #[cfg(riscv32)]
        () => super::mstatush::set_mbe(endianness),
        #[cfg(not(riscv32))]
        () => match endianness {
            Endianness::BigEndian => _set(1 << 37),
            Endianness::LittleEndian => _clear(1 << 37),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mpp() {
        let mut mstatus = Mstatus { bits: 0 };
        mstatus.set_mpp(MPP::User);
        assert_eq!(mstatus.mpp(), MPP::User);
        mstatus.set_mpp(MPP::Machine);
        assert_eq!(mstatus.mpp(), MPP::Machine);
        mstatus.set_mpp(MPP::Supervisor);
        assert_eq!(mstatus.mpp(), MPP::Supervisor);
    }
}
