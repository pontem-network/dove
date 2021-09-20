pub mod dependency;
mod extractor;
pub mod interact;
mod intern_table;
pub mod source_map;

use move_lang::shared::Flags;
use interact::CompilerInteract;
use anyhow::Error;
use lang::compiler::dialects::DialectName;
use std::str::FromStr;
use std::collections::HashMap;
use crate::compiler::source_map::SourceMap;
use crate::alert;

pub fn build(source_map: SourceMap, dialect: &str, sender: &str) -> Result<(), Error> {
    let ids = source_map.keys();
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let mut interact = CompilerInteract::new(dialect.as_ref(), sender, source_map);
    let res = move_lang::move_compile(&ids, &[], None, Flags::empty(), &mut interact)
        .map(|(st, m)| m.map(|mut m| m.remove(0).serialize()));
    alert(&format!("{:?}", res));
    Ok(())
}
