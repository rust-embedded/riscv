#[riscv_macros::setup_interrupts(arg)]
pub const async extern "Rust" fn setup_interrupts<'a, T>(_h: u32, _d: &'a T, _v: ...) -> !
where
    T: Copy,
{
}

fn main() {}
