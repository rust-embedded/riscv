#[riscv_rt::post_init(arg)]
pub const async extern "Rust" fn before_main<'a, T>(_h: u32, _d: &'a T, _v: ...) -> !
where
    T: Copy,
{
}

fn main() {}
