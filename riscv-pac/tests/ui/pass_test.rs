use riscv_pac::*;

#[repr(u16)]
#[pac_enum(unsafe ExceptionNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Exception {
    E1 = 1,
    E3 = 3,
}

#[repr(u16)]
#[pac_enum(unsafe InterruptNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Interrupt {
    I1 = 1,
    I2 = 2,
    I4 = 4,
    I7 = 7,
}

#[repr(u8)]
#[pac_enum(unsafe PriorityNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Priority {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
}

#[repr(u16)]
#[pac_enum(unsafe HartIdNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HartId {
    H0 = 0,
    H1 = 1,
    H2 = 2,
}

fn main() {
    assert_eq!(Exception::E1.number(), 1);
    assert_eq!(Exception::E3.number(), 3);

    assert_eq!(Exception::from_number(0), Err(0));
    assert_eq!(Exception::from_number(1), Ok(Exception::E1));
    assert_eq!(Exception::from_number(2), Err(2));
    assert_eq!(Exception::from_number(3), Ok(Exception::E3));
    assert_eq!(Exception::from_number(4), Err(4));

    assert_eq!(Interrupt::I1.number(), 1);
    assert_eq!(Interrupt::I2.number(), 2);
    assert_eq!(Interrupt::I4.number(), 4);
    assert_eq!(Interrupt::I7.number(), 7);

    assert_eq!(Interrupt::from_number(0), Err(0));
    assert_eq!(Interrupt::from_number(1), Ok(Interrupt::I1));
    assert_eq!(Interrupt::from_number(2), Ok(Interrupt::I2));
    assert_eq!(Interrupt::from_number(3), Err(3));
    assert_eq!(Interrupt::from_number(4), Ok(Interrupt::I4));
    assert_eq!(Interrupt::from_number(5), Err(5));
    assert_eq!(Interrupt::from_number(6), Err(6));
    assert_eq!(Interrupt::from_number(7), Ok(Interrupt::I7));

    assert_eq!(Priority::P0.number(), 0);
    assert_eq!(Priority::P1.number(), 1);
    assert_eq!(Priority::P2.number(), 2);
    assert_eq!(Priority::P3.number(), 3);

    assert_eq!(Priority::from_number(0), Ok(Priority::P0));
    assert_eq!(Priority::from_number(1), Ok(Priority::P1));
    assert_eq!(Priority::from_number(2), Ok(Priority::P2));
    assert_eq!(Priority::from_number(3), Ok(Priority::P3));
    assert_eq!(Priority::from_number(4), Err(4));

    assert_eq!(HartId::H0.number(), 0);
    assert_eq!(HartId::H1.number(), 1);
    assert_eq!(HartId::H2.number(), 2);

    assert_eq!(HartId::from_number(0), Ok(HartId::H0));
    assert_eq!(HartId::from_number(1), Ok(HartId::H1));
    assert_eq!(HartId::from_number(2), Ok(HartId::H2));
    assert_eq!(HartId::from_number(3), Err(3));
}
