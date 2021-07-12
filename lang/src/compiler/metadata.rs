use crate::compiler::dialects::Dialect;
use move_core_types::account_address::AccountAddress;
use anyhow::Error;
use move_lang::{find_move_filenames, leak_str, parse_file};
use crate::compiler::preprocessor::BuilderPreprocessor;
use move_lang::errors::{FilesSourceText, Errors};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub parameters: Vec<(String, String)>,
}

pub fn script_metadata(targets: &[String], dialect: &dyn Dialect, sender: Option<AccountAddress>) -> Result<Vec<Meta>, Error> {
    let targets = find_move_filenames(targets, true)?
        .iter()
        .map(|s| leak_str(s))
        .collect::<Vec<&'static str>>();

    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);

    let mut files: FilesSourceText = HashMap::new();
    let mut errors: Errors = Vec::new();
    let mut source_definitions = Vec::new();

    for fname in targets {
        let (defs, _, mut es) = parse_file(&mut files, fname, &mut preprocessor)?;
        source_definitions.extend(defs);
        errors.append(&mut es);
    }
    preprocessor.into_offset_map().transform(errors);



    todo!()
}