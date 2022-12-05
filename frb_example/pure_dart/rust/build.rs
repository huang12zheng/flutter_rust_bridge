use lib_flutter_rust_bridge_codegen::{config_parse, frb_codegen, run_template::*, RawOpts};

/// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
/// Path of output generated Dart code
const DART_OUTPUT: &str = "../dart/lib/bridge_generated.dart";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", RUST_INPUT);
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT.to_string()],
        wasm: true,
        dart_decl_output: Some("../dart/lib/bridge_definitions.dart".into()),
        dart_format_line_length: 120,
        // for other options use defaults
        ..Default::default()
    };
    // get opts from raw opts
    let configs = config_parse(raw_opts);

    // generation of rust api for ffi
    let mut otps = OptArray::new(&configs);
    otps.run_impl_trait_enum();
    // let all_symbols = otps.get_symbols_if_no_duplicates().unwrap();
    // for config in configs.iter() {
    //     frb_codegen(config, &all_symbols).unwrap();
    // }
}

// fn main() {}
