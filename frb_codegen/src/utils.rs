use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::iter::FromIterator;
use std::path::Path;
use std::process::Command;

use anyhow::anyhow;
use syn::ItemFn;

use crate::ir::IrTypeImplTrait;
use crate::source_graph::{Crate, Impl};

use self::parse_sig_from_lib::CallFn;
mod parse_sig_from_lib;
pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file<F: FnOnce() -> anyhow::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(path)?;
    fs::write(path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(path, content_original)?)
}

pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|x| !uniq.insert(*x))
        .cloned()
        .collect::<Vec<_>>()
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitily by frb
pub fn get_symbols_if_no_duplicates(configs: &[crate::Opts]) -> Result<Vec<String>, anyhow::Error> {
    Command::new("sh")
        .args([
            "-c",
            format!("sed -i '' '/.*mod bridge_generated.*/d' src/lib.rs").as_str(),
        ])
        .spawn()
        .ok();
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();

    let mut explicit_src_impl_pool: HashMap<String, Vec<Impl>> = HashMap::new();
    let mut explicit_parsed_impl_traits: HashSet<IrTypeImplTrait> = HashSet::new();
    let mut explicit_api_path: HashSet<String> = HashSet::new();

    let root_src_file = Crate::new(&configs[0].manifest_path).root_src_file;
    let source_rust_content = fs::read_to_string(&root_src_file).unwrap();
    let trait_sig_pool = parse_sig_from_lib::parse_file(&source_rust_content);

    for config in configs {
        let raw_ir_file = config.get_ir_file();

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));

        collect_impl_trait(
            &raw_ir_file,
            &mut explicit_src_impl_pool,
            &mut explicit_parsed_impl_traits,
        );

        explicit_api_path.insert(
            config
                .rust_input_path
                .split('/')
                .last()
                .map(|s| s.split('.').next())
                .unwrap()
                .unwrap()
                .to_owned(),
        );

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_file.get_all_symbols(config));
    }
    let bound_oject_pool =
        get_bound_oject_pool(explicit_src_impl_pool, explicit_parsed_impl_traits);

    generate_impl_file(trait_sig_pool, explicit_api_path, bound_oject_pool);
    let duplicates = find_all_duplicates(&explicit_raw_symbols);
    if !duplicates.is_empty() {
        let duplicated_symbols = duplicates.join(",");

        let (symbol_str, verb_str) = if duplicates.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        return Err(anyhow!(
            "{} [{}] {} already been defined",
            symbol_str,
            duplicated_symbols,
            verb_str
        ));
    }

    Ok(all_symbols)
}

fn generate_impl_file(
    trait_sig_pool: HashMap<String, CallFn>,
    explicit_api_path: HashSet<String>,
    bound_oject_pool: HashMap<Vec<String>, HashSet<String>>,
) {
    let mut lines = format!("");
    for super_ in explicit_api_path.iter() {
        lines += format!("use crate::{super_}::*;").as_str();
    }
    for (_, call_fn) in trait_sig_pool.iter() {
        let impl_ = call_fn.impl_.clone();
        lines += format!("{impl_}").as_str();
    }
    for (k, v) in bound_oject_pool.iter() {
        lines += format!("pub enum {}Enum {{", k.join("")).as_str();
        for struct_ in v.iter() {
            lines += format!("{}({}),", struct_, struct_).as_str();
        }
        lines += format!("}}").as_str();
    }

    for (k, v) in bound_oject_pool.iter() {
        let enum_ = format!("{}Enum", k.join(""));
        for trait_ in k.iter() {
            lines += format!("impl {trait_} for {enum_} {{").as_str();
            let call_fn = trait_sig_pool
                .get(trait_)
                .expect(&format!("Error: {:?} with {:?}", trait_sig_pool, trait_));

            lines += format!("{}{{", call_fn.sig).as_str();
            lines += format!("match *self {{").as_str();
            for sub_enum in v.iter() {
                lines += format!(
                    "{enum_}::{sub_enum}(ref __field0) => __field0.{}({}),",
                    call_fn.fn_name, call_fn.args
                )
                .as_str();
            }
            lines += format!("}}").as_str();
            lines += format!("}}").as_str();
            lines += format!("}}").as_str();
        }
    }

    fs::write("src/bridge_generated_bound.rs", lines).unwrap();
    Command::new("sh")
        .args([
            "-c",
            format!("echo 'mod bridge_generated_bound;' >> src/lib.rs").as_str(),
        ])
        .spawn()
        .ok();
}

fn collect_impl_trait(
    raw_ir_file: &crate::ir::IrFile,
    explicit_src_impl_pool: &mut HashMap<String, Vec<Impl>>,
    explicit_parsed_impl_traits: &mut HashSet<IrTypeImplTrait>,
) {
    // for checking relationship between trait and self_ty with all files
    for impl_ in raw_ir_file.src_impl_pool.iter().clone() {
        if let Some(v) = explicit_src_impl_pool.get_mut(impl_.0) {
            v.extend(impl_.1.to_owned());
        } else {
            explicit_src_impl_pool.insert(impl_.0.to_owned(), impl_.1.to_owned());
        }
    }
    // for getting all trait bound defined in func
    explicit_parsed_impl_traits.extend(raw_ir_file.parsed_impl_traits.clone());
}

fn get_bound_oject_pool(
    explicit_src_impl_pool: HashMap<String, Vec<Impl>>,
    explicit_parsed_impl_traits: HashSet<IrTypeImplTrait>,
) -> HashMap<Vec<String>, HashSet<String>> {
    // get a map from bound to all struct meet
    let mut bound_oject_pool = HashMap::new();
    for type_impl_trait in explicit_parsed_impl_traits.into_iter() {
        let raw = type_impl_trait.trait_bounds;

        raw.iter().for_each(|bound| {
            // Check whether the trait bound is capable of being used
            // ~~return None if param unoffical~~
            if !explicit_src_impl_pool.contains_key(bound) {
                panic!("loss impl {} for some self_ty", bound);
            }
        });

        let sets = raw.iter().map(|bound| {
            let impls = explicit_src_impl_pool.get(bound).unwrap();
            let iter = impls.iter().map(|impl_| impl_.self_ty.to_string());
            HashSet::from_iter(iter)
        });

        let mut iter = sets;

        let intersection_set = iter
            .next()
            .map(|set: HashSet<String>| iter.fold(set, |set1, set2| &set1 & &set2))
            .unwrap();
        bound_oject_pool.insert(raw, intersection_set);
    }
    bound_oject_pool
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[extend::ext]
impl std::path::Path {
    #[inline]
    fn file_name_str(&self) -> Option<&str> {
        self.file_name().and_then(OsStr::to_str)
    }
}
