use crate::compiler::dialects::Dialect;
use move_core_types::account_address::AccountAddress;
use anyhow::Error;
use move_lang::{find_move_filenames, leak_str};
use crate::compiler::parser::parse_file;
use crate::compiler::preprocessor::BuilderPreprocessor;

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

    let preprocessor = BuilderPreprocessor::new(dialect, sender);

    for fname in targets {
        let (defs, comments, mut es) = parse_file(&mut files, fname, preprocessor)?;
        source_definitions.extend(defs);
        source_comments.insert(fname, comments);
        errors.append(&mut es);
    }

}