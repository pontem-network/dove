use std::collections::HashMap;
use anyhow::{Error, anyhow};
use lang::compiler::dialects::Dialect;
use lang::compiler::metadata::FuncMeta;
use lang::compiler::preprocessor::BuilderPreprocessor;
use move_lang::parser::ast::Definition;
use move_lang::errors::FilesSourceText;
use crate::move_langwasm::parse_file;
use move_lang::callback::Interact;

pub fn script_meta_source(
    name: &str,
    source: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<FuncMeta>, Error> {
    Ok(parse(name, source, dialect, sender)?
        .into_iter()
        .filter_map(|def| {
            if let Definition::Script(script) = def {
                None
                // @todo
                // make_script_meta(script).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

fn parse(
    name: &str,
    source: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<Definition>, Error> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);
    let mut files: FilesSourceText = HashMap::new();
    let (defs, _, errors) = parse_file(
        &mut files,
        preprocessor.static_str(name.to_string()),
        source.to_string(),
        &mut preprocessor,
    )?;
    if errors.is_empty() {
        Ok(defs)
    } else {
        Err(anyhow!("Could not compile scripts '{}'.", name))
    }
}
