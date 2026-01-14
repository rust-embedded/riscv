#[riscv_rt::entry]
unsafe fn entry_point(_hart_id: usize) -> ! {
    loop {}
}

fn main() {} // TEST OK
