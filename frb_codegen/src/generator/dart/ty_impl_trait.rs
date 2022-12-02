use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeImplTraitGenerator, IrTypeImplTrait);

impl TypeDartGeneratorTrait for TypeImplTraitGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "final ptr = inner.new_{0}();
                _api_fill_to_wire_{0}(raw, ptr);
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some("return raw.share();".to_owned()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("wireObj.ptr = apiObj.share();".into())
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return {0}.fromRaw(raw[0], raw[1], this);",
            self.ir.dart_api_type()
        )
    }

    fn structs(&self) -> String {
        let field_bridge = format!(
            "final {} bridge;",
            self.context.config.dart_api_class_name(),
        );
        format!(
            "@sealed class {0} extends FrbImplTrait {{
                {field_bridge}
                    {0}.fromRaw(int ptr, int size, this.bridge) : super.unsafe(ptr, size);
                    @override
                    DropFnType get dropFn => bridge.dropImplTrait{0};
                    
                    @override
                    ShareFnType get shareFn => bridge.shareImplTrait{0};

                    @override
                    ImplTraitTypeFinalizer get staticFinalizer => bridge.{0}Finalizer;
            }}",
            self.ir.dart_api_type()
        )
    }
}
