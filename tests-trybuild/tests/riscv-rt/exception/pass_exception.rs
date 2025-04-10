use riscv::interrupt::Exception::*;

#[riscv_rt::exception(LoadMisaligned)]
fn simple_exception() {}

#[riscv_rt::exception(LoadFault)]
fn unmutable_exception(_trap_frame: &riscv_rt::TrapFrame) {}

#[riscv_rt::exception(StoreMisaligned)]
fn mutable_exception(_trap_frame: &mut riscv_rt::TrapFrame) {}

#[riscv_rt::exception(StoreFault)]
fn no_return_exception(_trap_frame: &mut riscv_rt::TrapFrame) -> ! {
    loop {}
}

fn main() {}
