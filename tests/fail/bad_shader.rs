use wgpu_shader_checker::include_wgsl;

fn main() {
    //I have to do this big thing bc try build compiles things a little weird
    include_wgsl!("../../shaders/fail.wgsl");
}
