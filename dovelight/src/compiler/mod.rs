mod interact;
mod dependency;
mod intern_table;
mod extractor;
mod source_map;

use move_lang::shared::Flags;
use interact::CompilerInteract;
use anyhow::Error;
use lang::compiler::dialects::DialectName;
use std::str::FromStr;
use std::collections::HashMap;
use crate::compiler::source_map::SourceMap;

pub fn build(source_map: SourceMap, dialect: &str, sender: &str) -> Result<(), Error> {
    let ids = source_map.keys();
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let mut interact = CompilerInteract::new(dialect.as_ref(), sender, source_map);
    move_lang::move_compile(&ids, &[], None, Flags::empty(), &mut interact);
    Ok(())
}