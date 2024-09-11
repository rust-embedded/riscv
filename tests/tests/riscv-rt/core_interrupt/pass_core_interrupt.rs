use riscv::interrupt::Interrupt::*;

#[riscv_rt::core_interrupt(SupervisorSoft)]
fn simple_interrupt() {}

#[riscv_rt::core_interrupt(SupervisorTimer)]
unsafe fn no_return_interrupt() -> ! {
    loop {}
}

fn main() {}
