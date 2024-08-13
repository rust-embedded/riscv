#[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::SupervisorSoft)]
fn my_interrupt(code: usize) {}

#[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::SupervisorTimer)]
fn my_other_interrupt() -> usize {}

#[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::SupervisorExternal)]
async fn my_async_interrupt() {}

fn main() {}
