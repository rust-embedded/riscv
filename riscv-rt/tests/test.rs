#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*/fail_*.rs");
    t.pass("tests/ui/*/pass_*.rs");
}
