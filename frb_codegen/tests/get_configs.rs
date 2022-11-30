#![cfg(test)]
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, Opts, RawOpts,
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

#[test]
fn get_ir() {
    insta::assert_debug_snapshot!(get_opt().get_ir_file());
}
