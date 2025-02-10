use std::path::PathBuf;

use proc_macro::TokenStream;
use syn::parse_macro_input;

fn compile_error(error: &str) -> TokenStream {
    //Sanitize the string just in case
    let error = error.replace("\\", "\\\\");
    let error = error.replace("\"", "\\\"");
    return format!("compile_error!(\"{error}\")").parse().unwrap();
}

#[proc_macro]
pub fn include_wgsl_checked(input: TokenStream) -> TokenStream {
    //Get the root of the crate
    let p = std::env::vars()
        .find(|i| i.0 == "CARGO_MANIFEST_PATH")
        .unwrap()
        .1;

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
        return compile_error(&format!("{} Is a directory", inp));
    }

    let f = std::fs::read_to_string(p);

    if let Err(e) = f {
        return compile_error(&format!("Unable to read file: {e}"));
    }

    let f = f.unwrap();

    let res = naga::front::wgsl::parse_str(&f);

    if let Err(e) = res {
        let msg = e.message();
        println!("{msg}");
        //I don't care about it being good, only it being bad
        return compile_error(&format!("{msg}"));
    }

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
