use move_lang::errors::{Error, Errors};
use move_lang::parser as libra_parser;
use move_lang::parser::ast::FileDefinition;
use move_lang::shared::Address;
use move_lang::strip_comments_and_verify;

use crate::db::FilePath;

pub fn parse_file(fname: FilePath, text: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, text)?;
    let parsed = libra_parser::syntax::parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}

pub fn translate_parsed_program(
    main_file: FileDefinition,
    dependencies: Vec<FileDefinition>,
    sender_opt: Option<Address>,
) -> Result<move_lang::cfgir::ast::Program, Errors> {
    let program = libra_parser::ast::Program {
        source_definitions: vec![main_file],
        lib_definitions: dependencies,
    };
    move_lang::check_program(Ok(program), sender_opt)
}
