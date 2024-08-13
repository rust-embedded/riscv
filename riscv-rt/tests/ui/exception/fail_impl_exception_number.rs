#[riscv_rt::exception(riscv::interrupt::Interrupt::SupervisorSoft)]
fn my_exception() {}

fn main() {}
