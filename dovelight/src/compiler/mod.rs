use std::str::FromStr;
use std::collections::HashMap;

use anyhow::Error;
use move_lang::compiled_unit;
use move_lang::errors::report_errors_to_color_buffer;
use move_lang::shared::Flags;
use move_lang::compiled_unit::CompiledUnit;

use interact::CompilerInteract;
use lang::compiler::dialects::DialectName;

use crate::compiler::source_map::SourceMap;
use crate::deps::{DependencyLoader, Store};
use crate::deps::resolver::DependencyResolver;

pub mod interact;
mod intern_table;
pub mod source_map;

pub fn build<L: DependencyLoader, S: Store>(
    loader: L,
    store: S,
    source_map: SourceMap,
    dialect: &str,
    sender: &str,
) -> Result<Vec<(String, Vec<u8>)>, Error> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let resolver = DependencyResolver::new(dialect.as_ref(), loader, store);
    let mut interact =
        CompilerInteract::new(dialect.as_ref(), sender, source_map.clone(), resolver);

    let result = build_base(&mut interact, source_map)?;
    result
        .into_iter()
        .map(|unit| {
            let mut bytecode = unit.serialize();
            dialect
                .adapt_to_target(&mut bytecode)
                .map(|_| (unit.name(), bytecode))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn build_base<L: DependencyLoader, S: Store>(
    interact: &mut CompilerInteract<L, S>,
    source_map: SourceMap,
) -> Result<Vec<CompiledUnit>, Error> {
    let ids = source_map.keys();
    let (_, units_res) = move_lang::move_compile(&ids, &[], None, Flags::empty(), interact)?;

    let sources = interact.sources();
    match units_res {
        Ok(compiled_units) => {
            let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);
            if !ice_errors.is_empty() {
                let error =
                    report_errors_to_color_buffer(sources, interact.transform(ice_errors));
                let err = String::from_utf8_lossy(&error).to_string();
                return Err(Error::msg(err));
            }
            Ok(compiled_units)
        }
        Err(errors) => {
            let error = report_errors_to_color_buffer(sources, interact.transform(errors));
            let err = String::from_utf8_lossy(&error).to_string();
            return Err(Error::msg(err));
        }
    }
}
