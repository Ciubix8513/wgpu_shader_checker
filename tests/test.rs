use trybuild::TestCases;
use wgpu_shader_checker::include_wgsl_checked;
// use wgpu_shader_checker::meow;

#[test]
fn test_macro() {
    include_wgsl_checked!("shaders/meow.wgsl");

    // let t = TestCases::new();

    // t.compile_fail("tests/fail/*.rs");
    // t.pass("tests/success/*.rs");
}
