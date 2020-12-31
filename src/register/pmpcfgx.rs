/// Physical memory protection configuration

pub mod pmpcfg0 {
    use bit_field::BitField;

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

    #[derive(Clone, Copy, Debug)]
    pub enum Range {
        OFF = 0,
        TOR = 1,
        NA4 = 2,
        NAPOT = 3,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpconfig {
        pub permission: Permission,
        pub range_type: Range,
        pub locked: bool,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct PmpByte {
        byte: u8,
    }

    impl PmpByte {
        #[inline]
        fn range(&self) -> Range {
            match self.byte.get_bits(4..=5) {
                0 => Range::OFF,
                1 => Range::TOR,
                2 => Range::NA4,
                3 => Range::NAPOT,
                _ => unreachable!(),
            }
        }

        #[inline]
        fn permission(&self) -> Option<Permission> {
            match self.byte.get_bits(0..=2) {
                0 => None,
                1 => Some(Permission::R),
                2 => Some(Permission::W),
                3 => Some(Permission::RW),
                4 => Some(Permission::X),
                5 => Some(Permission::RX),
                6 => Some(Permission::WX),
                7 => Some(Permission::RWX),
                _ => None,
            }
        }

        #[inline]
        fn locked(&self) -> bool {
            self.byte.get_bit(7)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Pmpcfg0 {
        bits: usize,
    }

    impl Pmpcfg0 {
        ///Returns the pmp byte associated with the index
        #[inline]
        fn pmp_byte(&self, index: usize) -> PmpByte {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            PmpByte { byte:self.bits.get_bits(8 * index..8 * (index + 1)) as u8 }
        }



        ///Returns pmp[x]cfg configuration structure
        #[inline]
        pub fn pmp_cfg(&self, index: usize) -> Pmpconfig {
            let byte = self.pmp_byte(index);
            let p = byte.permission().unwrap();
            let r = byte.range();
            let l = byte.locked();

            Pmpconfig {
                permission: p,
                range_type: r,
                locked: l,
            }
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

/// Physical memory protection configuration, RV32 only
pub mod pmpcfg1 {
    read_csr_as_usize_rv32!(0x3A1, __read_pmpcfg1);
    write_csr_as_usize_rv32!(0x3A1, __write_pmpcfg1);
}

/// Physical memory protection configuration
pub mod pmpcfg2 {
    read_csr_as_usize!(0x3A2, __read_pmpcfg2);
    write_csr_as_usize!(0x3A2, __write_pmpcfg2);
}

/// Physical memory protection configuration, RV32 only
pub mod pmpcfg3 {
    read_csr_as_usize_rv32!(0x3A3, __read_pmpcfg3);
    write_csr_as_usize_rv32!(0x3A3, __write_pmpcfg3);
}
