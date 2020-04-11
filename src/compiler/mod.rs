use move_lang::errors::Error;
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax;
use move_lang::strip_comments_and_verify;

use crate::ide::db::FilePath;

pub mod check;

pub fn parse_file(fname: FilePath, text: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, text)?;
    let parsed = syntax::parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}
