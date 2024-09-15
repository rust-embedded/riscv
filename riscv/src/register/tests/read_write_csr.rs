use crate::result::{Error, Result};

read_write_csr! {
    /// test CSR register type
    Mtest: 0x000,
    mask: 0b1111_1111_1111,
    field: {
        /// test single-bit field
        single,
        /// try-get test single-bit field
        try_single,
        /// setter test single-bit field
        set_single,
        /// try-setter test single-bit field
        try_set_single,
        bit: 0,
    }
}

read_write_csr_field! {
    Mtest,
    /// multiple single-bit field range
    multi_range,
    /// try-get multiple single-bit field range
    try_multi_range,
    /// setter multiple single-bit field range
    set_multi_range,
    /// try-setter multiple single-bit field range
    try_set_multi_range,
    range: 1..=3,
}

read_write_csr_field!(
    Mtest,
    /// multi-bit field
    multi_field,
    /// try-get multi-bit field
    try_multi_field,
    /// setter multi-bit field
    set_multi_field,
    /// try-setter multi-bit field
    try_set_multi_field,
    range: [4:7],
);

read_write_csr_field!(
    Mtest,
    /// multi-bit field
    field_enum,
    /// try-get multi-bit field
    try_field_enum,
    /// setter multi-bit field
    set_field_enum,
    /// try-setter multi-bit field
    try_set_field_enum,
    /// field enum type with valid field variants
    MtestFieldEnum {
        range: [7:11],
        default: Field1,
        Field1 = 1,
        Field2 = 2,
        Field3 = 3,
        Field4 = 15,
    }
);

// we don't test the `read` and `write` functions, we are only testing in-memory functions.
#[allow(unused)]
pub fn _read_csr() -> Mtest {
    read()
}

#[allow(unused)]
pub fn _try_read_csr() -> Result<Mtest> {
    try_read()
}

#[allow(unused)]
pub fn _write_csr(csr: Mtest) {
    write(csr);
}

#[allow(unused)]
pub fn _try_write_csr(csr: Mtest) {
    try_write(csr);
}

#[test]
fn test_mtest_read_write() {
    let mut mtest = Mtest::from_bits(0);

    assert_eq!(mtest.bitmask(), Mtest::BITMASK);
    assert_eq!(mtest.bits(), 0);

    // check that single bit field getter/setters work.
    assert_eq!(mtest.single(), false);

    mtest.set_single(true);
    assert_eq!(mtest.single(), true);

    mtest.set_single(false);
    assert_eq!(mtest.single(), false);

    // check that single bit range field getter/setters work.
    for i in 1..=3 {
        assert_eq!(mtest.multi_range(i), false);

        mtest.set_multi_range(i, true);
        assert_eq!(mtest.multi_range(i), true);

        mtest.set_multi_range(i, false);
        assert_eq!(mtest.multi_range(i), false);
    }

    // check that multi-bit field getter/setters work.
    assert_eq!(mtest.multi_field(), 0);

    mtest.set_multi_field(0xf);
    assert_eq!(mtest.multi_field(), 0xf);
    assert_eq!(mtest.try_multi_field(), Ok(0xf));

    mtest.set_multi_field(0x3);
    assert_eq!(mtest.multi_field(), 0x3);
    assert_eq!(mtest.try_multi_field(), Ok(0x3));

    // check that only bits in the field are set.
    mtest.set_multi_field(0xff);
    assert_eq!(mtest.multi_field(), 0xf);
    assert_eq!(mtest.try_multi_field(), Ok(0xf));
    assert_eq!(mtest.bits(), 0xf << 4);

    mtest.set_multi_field(0x0);
    assert_eq!(mtest.multi_field(), 0x0);
    assert_eq!(mtest.try_multi_field(), Ok(0x0));

    assert_eq!(
        mtest.try_field_enum(),
        Err(Error::InvalidFieldVariant {
            field: "field_enum",
            value: 0,
        })
    );

    [
        MtestFieldEnum::Field1,
        MtestFieldEnum::Field2,
        MtestFieldEnum::Field3,
        MtestFieldEnum::Field4,
    ]
    .into_iter()
    .for_each(|variant| {
        mtest.set_field_enum(variant);
        assert_eq!(mtest.field_enum(), variant);
        assert_eq!(mtest.try_field_enum(), Ok(variant));
    });

    // check that setting an invalid variant returns `None`
    mtest = Mtest::from_bits(0xbad << 7);
    assert_eq!(
        mtest.try_field_enum(),
        Err(Error::InvalidFieldVariant {
            field: "field_enum",
            value: 13,
        })
    );
}
