#[riscv_rt::entry]
fn entry_point(_hart_id: usize) -> ! {
    loop {}
}

fn main() {} // TEST OK
