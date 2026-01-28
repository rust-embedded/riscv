#[riscv_rt::entry(arg)]
pub const async extern "Rust" fn entry<'a, T>(_a: u32, _b: &'a T, _c: String, _d: usize, _e: ...)
where
    T: Copy,
{
}

fn main() {}
