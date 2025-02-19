use trybuild::TestCases;

#[test]
fn test_macro() {
    let t = TestCases::new();
    t.compile_fail("tests/fail/*.rs");
    t.pass("tests/success/*.rs");
}
