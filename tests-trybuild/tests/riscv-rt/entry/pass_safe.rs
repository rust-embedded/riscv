#[riscv_rt::entry]
fn entry(_a0: usize, _a1: usize, _a2: usize) -> ! {
    loop {}
}

fn main() {}
