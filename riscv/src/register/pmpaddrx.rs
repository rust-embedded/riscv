macro_rules! reg {
    (
        $addr:expr, $csr:ident
    ) => {
        /// Physical memory protection address register
        pub mod $csr {
            read_csr_as_usize!($addr);
            write_csr_as_usize!($addr);
        }
    };
}

reg!(0x3B0, pmpaddr0);
reg!(0x3B1, pmpaddr1);
reg!(0x3B2, pmpaddr2);
reg!(0x3B3, pmpaddr3);
reg!(0x3B4, pmpaddr4);
reg!(0x3B5, pmpaddr5);
reg!(0x3B6, pmpaddr6);
reg!(0x3B7, pmpaddr7);
reg!(0x3B8, pmpaddr8);
reg!(0x3B9, pmpaddr9);
reg!(0x3BA, pmpaddr10);
reg!(0x3BB, pmpaddr11);
reg!(0x3BC, pmpaddr12);
reg!(0x3BD, pmpaddr13);
reg!(0x3BE, pmpaddr14);
reg!(0x3BF, pmpaddr15);
reg!(0x3C0, pmpaddr16);
reg!(0x3C1, pmpaddr17);
reg!(0x3C2, pmpaddr18);
reg!(0x3C3, pmpaddr19);
reg!(0x3C4, pmpaddr20);
reg!(0x3C5, pmpaddr21);
reg!(0x3C6, pmpaddr22);
reg!(0x3C7, pmpaddr23);
reg!(0x3C8, pmpaddr24);
reg!(0x3C9, pmpaddr25);
reg!(0x3CA, pmpaddr26);
reg!(0x3CB, pmpaddr27);
reg!(0x3CC, pmpaddr28);
reg!(0x3CD, pmpaddr29);
reg!(0x3CE, pmpaddr30);
reg!(0x3CF, pmpaddr31);
reg!(0x3D0, pmpaddr32);
reg!(0x3D1, pmpaddr33);
reg!(0x3D2, pmpaddr34);
reg!(0x3D3, pmpaddr35);
reg!(0x3D4, pmpaddr36);
reg!(0x3D5, pmpaddr37);
reg!(0x3D6, pmpaddr38);
reg!(0x3D7, pmpaddr39);
reg!(0x3D8, pmpaddr40);
reg!(0x3D9, pmpaddr41);
reg!(0x3DA, pmpaddr42);
reg!(0x3DB, pmpaddr43);
reg!(0x3DC, pmpaddr44);
reg!(0x3DD, pmpaddr45);
reg!(0x3DE, pmpaddr46);
reg!(0x3DF, pmpaddr47);
reg!(0x3E0, pmpaddr48);
reg!(0x3E1, pmpaddr49);
reg!(0x3E2, pmpaddr50);
reg!(0x3E3, pmpaddr51);
reg!(0x3E4, pmpaddr52);
reg!(0x3E5, pmpaddr53);
reg!(0x3E6, pmpaddr54);
reg!(0x3E7, pmpaddr55);
reg!(0x3E8, pmpaddr56);
reg!(0x3E9, pmpaddr57);
reg!(0x3EA, pmpaddr58);
reg!(0x3EB, pmpaddr59);
reg!(0x3EC, pmpaddr60);
reg!(0x3ED, pmpaddr61);
reg!(0x3EE, pmpaddr62);
reg!(0x3EF, pmpaddr63);

