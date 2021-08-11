/// Physical memory protection configuration
use bit_field::BitField;

/// Permission enum contains all possible permission modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Permission {
    NONE = 0b000,
    R = 0b001,
    W = 0b010,
    RW = 0b011,
    X = 0b100,
    RX = 0b101,
    WX = 0b110,
    RWX = 0b111,
}

/// Range enum contains all possible addressing modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Range {
    OFF = 0b00,
    TOR = 0b01,
    NA4 = 0b10,
    NAPOT = 0b11,
}

/// RV64 Pmpcfg holds the entire register (8 Pmp configurations)
#[derive(Clone, Copy, Debug)]
pub struct Pmp {
    /// raw bits
    pub byte: u8,
    /// Current PMP Permission
    pub permission: Permission,
    /// Current PMP Range
    pub range: Range,
    /// Is PMP locked?
    pub locked: bool,
}

pub struct Pmpcsr {
    /// Holds the raw contents of the PMP CSR
    pub bits: usize,
}

impl Pmpcsr {
    /// Take the register contents and translate into configurations held in a Pmpcfg struct
    /// Takes `self` as a parameter and destroys the current Pmpcsr struct
    #[inline]
    pub fn into_config(&self, index: usize) -> Pmp {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let byte = self.bits.get_bits(8 * index..=8 * index + 7) as u8;
        Pmp {
            byte,
            permission: match byte.get_bits(0..=2) {
                0 => Permission::NONE,
                1 => Permission::R,
                2 => Permission::W,
                3 => Permission::RW,
                4 => Permission::X,
                5 => Permission::RX,
                6 => Permission::WX,
                7 => Permission::RWX,
                _ => unreachable!(),
            },
            range: match byte.get_bits(3..=4) {
                0 => Range::OFF,
                1 => Range::TOR,
                2 => Range::NA4,
                3 => Range::NAPOT,
                _ => unreachable!(),
            },
            locked: byte.get_bit(7) as bool,
        }
    }
}

/// Physical memory protection configuration
/// pmpcfg0 struct contains pmp0cfg - pmp3cfg for RV32, and pmp0cfg - pmp7cfg for RV64
pub mod pmpcfg0 {
    use super::{Permission, Pmpcsr, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcsr, 0x3A0, __read_pmpcfg0);
    write_csr_as_usize!(0x3A0, __write_pmpcfg0);
    set!(0x3A0, __set_pmpcfg0);
    clear!(0x3A0, __clear_pmpcfg0);

    /// set the configuration for pmp(x)
    pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
        value.set_bits(8 * index..=8 * index + 7, byte);
        _set(value)
    }
}

/// Physical memory protection configuration
/// pmpcfg1 struct contains pmp4cfg - pmp7cfg for RV32 only
#[cfg(riscv32)]
pub mod pmpcfg1 {
    use super::{Permission, Pmpcsr, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcsr, 0x3A1, __read_pmpcfg1);
    write_csr_as_usize_rv32!(0x3A1, __write_pmpcfg1);
    set!(0x3A1, __set_pmpcfg1);
    clear!(0x3A1, __clear_pmpcfg1);

    /// set the configuration for pmp(x)
    pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
        assert!(index < 4);

        let mut value = _read();
        let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
        value.set_bits(8 * index..=8 * index + 7, byte);
        _set(value)
    }
}

/// Physical memory protection configuration
/// pmpcfg2 struct contains pmp8cfg - pmp11cfg for RV32, or pmp8cfg - pmp15cfg for RV64
pub mod pmpcfg2 {
    use super::{Permission, Pmpcsr, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcsr, 0x3A2, __read_pmpcfg2);
    write_csr_as_usize!(0x3A2, __write_pmpcfg2);
    set!(0x3A2, __set_pmpcfg2);
    clear!(0x3A2, __clear_pmpcfg2);

    /// set the configuration for pmp(x)
    pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
        #[cfg(riscv32)]
        assert!(index < 4);

        #[cfg(riscv64)]
        assert!(index < 8);

        let mut value = _read();
        let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
        value.set_bits(8 * index..=8 * index + 7, byte);
        _set(value)
    }
}

/// Physical memory protection configuration
/// pmpcfg3 struct contains pmp12cfg - pmp15cfg for RV32 only
#[cfg(riscv32)]
pub mod pmpcfg3 {
    use super::{Permission, Pmpcsr, Range};
    use bit_field::BitField;

    read_csr_as!(Pmpcsr, 0x3A3, __read_pmpcfg3);
    write_csr_as_usize_rv32!(0x3A3, __read_pmpcfg3);
    set!(0x3A3, __set_pmpcfg3);
    clear!(0x3A3, __clear_pmpcfg3);

    /// set the configuration for pmp(x)
    pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
        assert!(index < 4);

        let mut value = _read();
        let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
        value.set_bits(8 * index..=8 * index + 7, byte);
        _set(value)
    }
}
