/// Physical memory protection configuration
use bit_field::BitField;

/// Permission enum contains all possible permission modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Permission {
    NONE = 0,
    R = 1,
    W = 2,
    RW = 3,
    X = 4,
    RX = 5,
    WX = 6,
    RWX = 7,
}

/// Range enum contains all possible addressing modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Range {
    OFF = 0,
    TOR = 1,
    NA4 = 2,
    NAPOT = 3,
}

/// PmpByte holds the a single pmp configuration
#[derive(Clone, Copy, Debug)]
pub struct PmpByte {
    pub byte: u8,
    //permission: Option<Permission>,
    //range: Option<Range>,
    //locked: bool
}
/// PmpByte methods to get a pmp configuration attributes
impl PmpByte {
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.byte.get_bit(7)
    }

    #[inline]
    pub fn get_range(&self) -> Option<Range> {
        match self.byte.get_bits(4..=5) {
            0 => Some(Range::OFF),
            1 => Some(Range::TOR),
            2 => Some(Range::NA4),
            3 => Some(Range::NAPOT),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn get_permission(&self) -> Option<Permission> {
        match self.byte.get_bits(0..=2) {
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
}

/// Physical memory protection configuration
/// Pmpcf0 struct contains pmp0cfg - pmp3cfg for 32-bit arch, or pmp0cfg - pmp7cfg for 64-bit arch
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg0 {
    use super::PmpByte;

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpcfg0 {
        pub bits: u32,
    }
    impl Pmpcfg0 {
        #[inline]
        pub fn get_byte(&self, index: usize) -> PmpByte {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            PmpByte {
                byte: self.bits.get_bits(8 * index..8 * (index + 1)) as u8,
            }
        }

        read_csr_as!(Pmpcfg0, 0x3A0, __read_pmpcfg0);
        write_csr!(0x3A0, __write_pmpcfg0);
        set!(0x3A0, __set_pmpcfg0);
        clear!(0x3A0, __clear_pmpcfg0);

        #[inline]
        pub unsafe fn set_permissions(permission: Permission, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((permission as usize) << (index * 8));
        }

        #[inline]
        pub unsafe fn set_range(range: Range, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((range as usize) << (3 + (index * 8)));
        }

        #[inline]
        pub unsafe fn set_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _set(1 << (7 + (index * 8)));
        }

        #[inline]
        pub unsafe fn clear_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _clear(1 << (7 + (index * 8)));
        }
    }
}

/// Physical memory protection configuration, RV32 only
/// Pmpcf1 struct contains pmp4cfg - pmp7cfg for 32-bit arch only
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg1 {
    use super::PmpByte;

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpcfg1 {
        pub bits: u32,
    }

    impl Pmpcfg1 {
        #[inline]
        pub fn get_byte(&self, index: usize) -> PmpByte {
            PmpByte {
                byte: self.bits.get_bits(8 * index..8 * (index + 1)) as u8,
            }
        }

        read_csr_as_usize_rv32!(0x3A1, __read_pmpcfg1);
        write_csr_as_usize_rv32!(0x3A1, __write_pmpcfg1);

        #[inline]
        pub unsafe fn set_permissions(permission: Permission, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((permission as usize) << (index * 8));
        }

        #[inline]
        pub unsafe fn set_range(range: Range, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((range as usize) << (3 + (index * 8)));
        }

        #[inline]
        pub unsafe fn set_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _set(1 << (7 + (index * 8)));
        }

        #[inline]
        pub unsafe fn clear_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _clear(1 << (7 + (index * 8)));
        }
    }
}

/// Physical memory protection configuration
/// Pmpcf0 struct contains pmp8cfg - pmp11cfg for 32-bit arch, or pmp8cfg - pmp15cfg for 64-bit arch
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg2 {
    use super::PmpByte;

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpcfg2 {
        pub bits: u32,
    }
    impl Pmpcfg2 {
        #[inline]
        pub fn get_byte(&self, index: usize) -> PmpByte {
            PmpByte {
                byte: self.bits.get_bits(8 * index..8 * (index + 1)) as u8,
            }
        }

        read_csr_as_usize!(0x3A2, __read_pmpcfg2);
        write_csr_as_usize!(0x3A2, __write_pmpcfg2);

        #[inline]
        pub unsafe fn set_permissions(permission: Permission, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((permission as usize) << (index * 8));
        }

        #[inline]
        pub unsafe fn set_range(range: Range, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((range as usize) << (3 + (index * 8)));
        }

        #[inline]
        pub unsafe fn set_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _set(1 << (7 + (index * 8)));
        }

        #[inline]
        pub unsafe fn clear_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _clear(1 << (7 + (index * 8)));
        }
    }
}

/// Physical memory protection configuration, RV32 only
/// Pmpcf0 struct contains pmp12cfg - pmp15cfg for 32-bit arch only
/// get_byte() method retrieves a single pmp<x>cfg held in a PmpByte struct
pub mod pmpcfg3 {
    use super::PmpByte;

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpcfg3 {
        pub bits: u32,
    }
    impl Pmpcfg3 {
        #[inline]
        pub fn get_byte(&self, index: usize) -> PmpByte {
            PmpByte {
                byte: self.bits.get_bits(8 * index..8 * (index + 1)) as u8,
            }
        }

        read_csr_as_usize_rv32!(0x3A3, __read_pmpcfg3);
        write_csr_as_usize_rv32!(0x3A3, __write_pmpcfg3);

        #[inline]
        pub unsafe fn set_permissions(permission: Permission, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((permission as usize) << (index * 8));
        }

        #[inline]
        pub unsafe fn set_range(range: Range, index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _write((range as usize) << (3 + (index * 8)));
        }

        #[inline]
        pub unsafe fn set_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _set(1 << (7 + (index * 8)));
        }

        #[inline]
        pub unsafe fn clear_lock(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            _clear(1 << (7 + (index * 8)));
        }
    }
}
