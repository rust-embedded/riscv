#[test]
fn riscv() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/riscv/fail_*.rs");
    t.pass("tests/riscv/pass_*.rs");
}

#[test]
fn riscv_rt() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/riscv-rt/*/fail_*.rs");
    t.pass("tests/riscv-rt/*/pass_*.rs");
}
