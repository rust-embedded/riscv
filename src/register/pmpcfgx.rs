/// Physical memory protection configuration
use bit_field::BitField;

/// Permission enum contains all possible permission modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Permission {
    NONE = 0b_000,
    R = 0b_001,
    W = 0b_010,
    RW = 0b_011,
    X = 0b_100,
    RX = 0b_101,
    WX = 0b_110,
    RWX = 0b_111,
}

/// Range enum contains all possible addressing modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Range {
    OFF = 0b_00,
    TOR = 0b_01,
    NA4 = 0b_10,
    NAPOT = 0b_11,
}

#[derive(Clone, Copy, Debug)]
pub struct Pmpcfg {
    pub bits: usize,
}

/// PmpByte holds the a single pmp configuration
#[derive(Clone, Copy, Debug)]
pub struct PmpEntry {
    pub byte: usize,
    pub index: usize,
    pub permission: Option<Permission>,
    pub range: Option<Range>,
    pub locked: bool
}

impl Pmpcfg {
    #[inline]
    pub fn get_config(&self, index: usize) -> PmpEntry {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        PmpEntry {
            byte: self.get_byte(index),
            index,
            permission: self.get_permission(index),
            range: self.get_range(index),
            locked: self.is_locked(index)
        }
    }

    /// PmpByte methods to get a pmp configuration attributes
    pub fn get_byte(&self, index: usize) -> usize { self.bits.get_bits(8 * index..=(8 * index) + 7) as usize }

    #[inline]
    pub fn is_locked(&self, index: usize) -> bool {
        self.bits.get_bit(7 + (8 * index))
    }

    #[inline]
    pub fn get_permission(&self, index: usize) -> Option<Permission> {
        match self.bits.get_bits(8 * index..=8 * index + 2) {
            0 => Some(Permission::NONE),
            1 => Some(Permission::R),
            2 => Some(Permission::W),
            3 => Some(Permission::RW),
            4 => Some(Permission::X),
            5 => Some(Permission::RX),
            6 => Some(Permission::WX),
            7 => Some(Permission::RWX),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn get_range(&self, index: usize) -> Option<Range> {
        match self.bits.get_bits(8 * index + 3..=8 * index + 4) {
            0 => Some(Range::OFF),
            1 => Some(Range::TOR),
            2 => Some(Range::NA4),
            3 => Some(Range::NAPOT),
            _ => unreachable!(),
        }
    }
}
/// Physical memory protection configuration
/// Pmpcfg0 struct contains pmp0cfg - pmp3cfg for RV32, or pmp0cfg - pmp7cfg for RV64
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg0 {
    use super::{Permission, Pmpcfg, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcfg, 0x3A0, __read_pmpcfg0);
    write_csr!(0x3A0, __write_pmpcfg0);
    set!(0x3A0, __set_pmpcfg0);
    clear!(0x3A0, __clear_pmpcfg0);

    #[inline]
    pub unsafe fn set_permissions(permission: Permission, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index..=8 * index + 2,permission as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_range(range: Range, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index + 3..=8 * index + 4,range as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_lock(index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        _set(1 << (7 + index * 8));
    }
}

/// Physical memory protection configuration
/// Pmpcfg1 struct contains pmp4cfg - pmp7cfg for RV32 only
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg1 {
    use super::{Permission, Pmpcfg, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcfg, 0x3A1, __read_pmpcfg1);
    write_csr!(0x3A1, __write_pmpcfg1);
    set!(0x3A1, __set_pmpcfg1);
    clear!(0x3A1, __clear_pmpcfg1);

    #[inline]
    pub unsafe fn set_permissions(permission: Permission, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index..=8 * index + 2,permission as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_range(range: Range, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index + 3..=8 * index + 4,range as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_lock(index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        _set(1 << (7 + index * 8));
    }
}

/// Physical memory protection configuration
/// Pmpcfg0 struct contains pmp8cfg - pmp11cfg for RV32, or pmp8cfg - pmp15cfg for RV64
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg2 {
    use super::{Permission, Pmpcfg, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcfg, 0x3A2, __read_pmpcfg2);
    write_csr!(0x3A2, __write_pmpcfg2);
    set!(0x3A2, __set_pmpcfg2);
    clear!(0x3A2, __clear_pmpcfg2);

    #[inline]
    pub unsafe fn set_permissions(permission: Permission, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index..=8 * index + 2,permission as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_range(range: Range, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index + 3..=8 * index + 4,range as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_lock(index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        _set(1 << (7 + index * 8));
    }
}

/// Physical memory protection configuration
/// Pmpcfg0 struct contains pmp12cfg - pmp15cfg for RV32 only
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg3 {
    use super::{Permission, Pmpcfg, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcfg, 0x3A3, __read_pmpcfg3);
    write_csr!(0x3A3, __write_pmpcfg3);
    set!(0x3A3, __set_pmpcfg3);
    clear!(0x3A3, __clear_pmpcfg3);

    #[inline]
    pub unsafe fn set_permissions(permission: Permission, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index..=8 * index + 2,permission as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_range(range: Range, index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        value.set_bits(8 * index + 3..=8 * index + 4,range as usize);
        _write(value);
    }

    #[inline]
    pub unsafe fn set_lock(index: usize) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        _set(1 << (7 + index * 8));
    }
}
