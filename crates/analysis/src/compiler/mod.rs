use move_lang::errors::Error;
use move_lang::parser as libra_parser;
use move_lang::parser::ast::FileDefinition;
use move_lang::strip_comments_and_verify;

use crate::db::FilePath;

pub mod check;

pub fn parse_file(fname: FilePath, text: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, text)?;
    let parsed = libra_parser::syntax::parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}
