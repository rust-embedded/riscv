#[riscv_rt::exception(riscv::interrupt::Exception::LoadMisaligned)]
fn my_exception(code: usize) {}

#[riscv_rt::exception(riscv::interrupt::Exception::StoreMisaligned)]
fn my_other_exception(trap_frame: &riscv_rt::TrapFrame, code: usize) {}

#[riscv_rt::exception(riscv::interrupt::Exception::LoadFault)]
async fn my_async_exception(trap_frame: &riscv_rt::TrapFrame, code: usize) {}

fn main() {}
