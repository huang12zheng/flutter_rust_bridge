use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrImpl {
    pub name: String,
}
// pub struct IrImpl<'a> {
//     pub trait_: &'a IrImplRef,
//     pub structs: Vec<&'a IrStruct>,
// }

impl IrImpl {
    // pub fn get<'a>(&self, file: &'a IrFile) -> &'a IrImpl {
    //     &IrImpl {
    //         trait_: self,
    //         structs: file.impl_trait_to_struct_pool[&self.name]
    //             .iter()
    //             .map(|t| file.struct_pool.get(t).unwrap())
    //             .collect(),
    //     }
    // }
}

impl std::fmt::Display for IrImpl {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.name)
    }
}

impl IrImpl {
    pub fn new(raw: String) -> IrImpl {
        IrImpl { name: raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.name
    }

    pub fn dart_style(&self) -> String {
        self.name.to_case(Case::Camel)
    }
}

impl IrTypeTrait for IrImpl {
    fn visit_children_types<F: FnMut(&super::IrType) -> bool>(
        &self,
        f: &mut F,
        ir_file: &super::IrFile,
    ) {
    }

    fn safe_ident(&self) -> String {
        self.name.to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        // format!("{}Enum", self.raw)
        self.name.to_string()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.name)
        }
    }
}
