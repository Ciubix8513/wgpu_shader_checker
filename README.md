# wgpu_shader_checker
This crate includes a single macro, `include_wgsl`, that replaces the default wgpu macro of the same name. 
It has a similar functionality to the wgpu one, however this macro performs a compile time check on the shader, throwing a compile issue if the shader does not compile.

# Usage
```rs
use wgpu_shader_checker::include_wgsl;

fn main() {
  let shader = include_wgsl!("shaders/shader.wgsl")
}
```

## Usage note

The macro expects a path relative to `CARGO_MANIFEST_DIR` as opposed to relative to the way the original macro works.

This is due to it currently being impossible to implement that behaviour, until the `proc_macro::Span` inspection API feature is stabilized.
