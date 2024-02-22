#[riscv_pac::pac_enum(InterruptNumber)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Interrupt {
    I1 = 1,
    I2 = 2,
    I4 = 4,
}

fn main() {}
