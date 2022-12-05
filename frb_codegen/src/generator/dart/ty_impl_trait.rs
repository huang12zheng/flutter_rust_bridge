use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeImplTraitGenerator, IrTypeImplTrait);

impl TypeDartGeneratorTrait for TypeImplTraitGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!("")),
            wasm: Some("".to_owned()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("".into())
    }

    fn wire2api_body(&self) -> String {
        format!("")
    }

    fn structs(&self) -> String {
        format!("")
    }
}
