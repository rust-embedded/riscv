#[test]
fn riscv_rt() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/riscv-rt/*/fail_*.rs");
    t.pass("tests/riscv-rt/*/pass_*.rs");
}
