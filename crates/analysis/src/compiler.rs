use move_lang::errors::{Error, Errors};
use move_lang::parser as libra_parser;
use move_lang::parser::ast::Definition;
use move_lang::shared::Address;
use move_lang::strip_comments_and_verify;

use crate::db::FilePath;

pub fn parse_file(fname: FilePath, text: &str) -> Result<Vec<Definition>, Error> {
    let no_comments_source = strip_comments_and_verify(fname, text)?;
    libra_parser::syntax::parse_file_string(fname, &no_comments_source)
}

pub fn check_parsed_program(
    current_file_defs: Vec<Definition>,
    dependencies: Vec<Definition>,
    sender_opt: Address,
) -> Result<(), Errors> {
    let ast_program = libra_parser::ast::Program {
        source_definitions: current_file_defs,
        lib_definitions: dependencies,
    };
    move_lang::check_program(Ok(ast_program), Some(sender_opt))?;
    Ok(())
}
