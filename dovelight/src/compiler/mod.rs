use std::str::FromStr;

use anyhow::Error;
use move_lang::shared::Flags;

use interact::CompilerInteract;
use lang::compiler::dialects::DialectName;

use crate::alert;
use crate::compiler::source_map::SourceMap;
use crate::compiler::storage::web::WebStorage;
use crate::compiler::deps::resolver::DependencyResolver;
use crate::compiler::loader::Loader;

pub mod deps;
pub mod interact;
mod intern_table;
pub mod loader;
pub mod source_map;
pub mod storage;

pub fn build(source_map: SourceMap, dialect: &str, sender: &str) -> Result<(), Error> {
    let cache = WebStorage::new_in_family("dove_cache_").unwrap();
    let loader = Loader {};
    let resolver = DependencyResolver::new(loader, cache);

    let ids = source_map.keys();
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let mut interact = CompilerInteract::new(dialect.as_ref(), sender, source_map, resolver);
    let res = move_lang::move_compile(&ids, &[], None, Flags::empty(), &mut interact)
        .map(|(_, m)| m.map(|mut m| m.remove(0).serialize()));

    alert(&format!("{:?}", res));
    Ok(())
}
