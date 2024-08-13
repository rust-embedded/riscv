#[riscv_rt::core_interrupt(riscv::interrupt::Exception::LoadMisaligned)]
fn my_interrupt() {}

fn main() {}
