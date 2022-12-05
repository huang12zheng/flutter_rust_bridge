use std::collections::HashSet;

use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};
use syn::{Ident, TypeImplTrait};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeImplTrait {
    pub trait_bounds: Vec<String>,
}

impl IrTypeImplTrait {
    pub fn join(&self) -> String {
        self.trait_bounds.join("_")
    }

    pub fn to_enum(&self) -> String {
        format!("{}Enum", self.join()).to_case(Case::Pascal)
    }
}

impl std::fmt::Display for IrTypeImplTrait {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.join())
    }
}

impl IrTypeImplTrait {
    pub fn new(raw: Vec<Ident>) -> IrTypeImplTrait {
        IrTypeImplTrait {
            trait_bounds: raw.iter().map(|e| e.to_string()).collect(),
        }
    }
    pub fn new2(raw: Vec<String>) -> IrTypeImplTrait {
        IrTypeImplTrait { trait_bounds: raw }
    }
}

impl IrTypeTrait for IrTypeImplTrait {
    fn visit_children_types<F: FnMut(&super::IrType) -> bool>(
        &self,
        f: &mut F,
        ir_file: &super::IrFile,
    ) {
    }

    fn safe_ident(&self) -> String {
        self.to_enum().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        self.rust_api_type()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.to_enum()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.to_enum())
        }
    }
}
