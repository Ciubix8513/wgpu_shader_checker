use std::path::PathBuf;

use proc_macro::TokenStream;
use syn::parse_macro_input;

fn compile_error(error: &str) -> TokenStream {
    //Sanitize the string just in case
    let error = error.replace('\\', "\\\\");
    let error = error.replace('"', "\\\"");

    format!("compile_error!(\"{error}\")").parse().unwrap()
}

#[proc_macro]
///Load WGSL source code from a file at compile time. If the file can not be compiled it will throw
///a compilation error.
///
///The macro uses absolute filenames, relative to `CARGO_MANIFEST_PATH`. It's currently impossible
///to get a relative file location, until the `proc_macro::Span` inspection API feature is stabilized
///
///# Examples
/// ```rs
///let shader = include_wgsl!("shaders/shader.wgsl")
///
/// ```
///
#[allow(clippy::missing_panics_doc)]
pub fn include_wgsl(input: TokenStream) -> TokenStream {
    // Not sure if this is a good solution, but it works
    let p = std::env::vars().find(|i| i.0 == "CARGO_MANIFEST_PATH");

    if p.is_none() {
        return compile_error("Could not determine CARGO_MANIFEST_PATH");
    }

    let p = p.unwrap_or_default().1;

    // filename
    let inp = parse_macro_input!(input as syn::LitStr).value();

    let mut p = p
        .parse::<PathBuf>()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    p.push(&inp);

    if !p.exists() {
        return compile_error("No such file or directory");
    }

    if !p.is_file() {
        return compile_error(&format!("{inp} Is a directory"));
    }

    let f = std::fs::read_to_string(p);

    if let Err(e) = f {
        return compile_error(&format!("Unable to read file: {e}"));
    }

    let f = f.unwrap();

    if f.is_empty() {
        return compile_error(&format!("{inp} is empty"));
    }

    let res = naga::front::wgsl::parse_str(&f);

    if let Err(e) = res {
        let msg = e.message();
        //I don't care about it being good, only it being bad
        return compile_error(msg);
    }

    //Return a shader module descriptor, the same as include_wgsl
    format!(
        "
    wgpu::ShaderModuleDescriptor {{
        label : Some(\"{inp}\"),
        source : wgpu::ShaderSource::Wgsl(\"{f}\".into())
    }}"
    )
    .parse()
    .unwrap()
}
