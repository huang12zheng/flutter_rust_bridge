#![cfg(test)]
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, source_graph::Crate, transformer,
    Opts, RawOpts,
};

pub fn get_opts() -> Vec<Opts> {
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec!["../frb_example/pure_dart/rust/src/api.rs".to_string()],
        // Path of output generated Dart code
        dart_output: vec!["../frb_example/pure_dart/dart/lib/bridge_generated.dart".to_string()],
        wasm: true,
        dart_decl_output: Some("../frb_example/pure_dart/dart/lib/bridge_definitions.dart".into()),
        dart_format_line_length: 120,
        // for other options use defaults
        ..Default::default()
    };
    config_parse(raw_opts)
}

pub fn get_opt() -> Opts {
    get_opts().remove(0)
}

// `frb_codegen/src/lib.rs:54`
#[test]
fn get_raw_ir() {
    insta::assert_debug_snapshot!(get_opt().get_ir_file());
}

// `frb_codegen/src/lib.rs:57`
#[test]
fn get_ir() {
    insta::assert_debug_snapshot!(transformer::transform(get_opt().get_ir_file()));
}
// use std::fs::read_to_string;
// `frb_codegen/src/config.rs:401`
#[test]
fn get_irx() {
    let opts = get_opt();
    let source_rust_content = std::fs::read_to_string(&opts.rust_input_path).unwrap();
    let file_ast = syn::parse_file(&source_rust_content).unwrap();

    // for item in file_ast.items.iter() {

    // }
    let output = Crate::new(&opts.manifest_path).root_module;
    // pure_dart/rust/Cargo.toml

    insta::assert_debug_snapshot!(opts.manifest_path, file_ast.items);
    insta::assert_debug_snapshot!(output);
}
