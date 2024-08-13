#[riscv_rt::external_interrupt(riscv::interrupt::Interrupt::SupervisorSoft)]
fn my_interrupt() {}

fn main() {}
