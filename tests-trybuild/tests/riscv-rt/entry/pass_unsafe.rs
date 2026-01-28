#[riscv_rt::entry]
unsafe fn entry(_a0: usize, _a1: usize, _a2: usize) -> ! {
    loop {}
}
fn main() {}
