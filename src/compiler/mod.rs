use std::fs;

use lsp_types::Diagnostic;
use move_lang::errors::FilesSourceText;
use move_lang::parser as libra_parser;
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax::parse_file_string;
use move_lang::shared::Address;

use crate::compiler::utils::convert_error_into_diags;
use std::path::PathBuf;

pub mod check;
pub mod parser;
pub mod utils;

pub type CompilerCheckResult<P> = Result<P, Vec<Diagnostic>>;

pub fn check_with_compiler(
    current_file: &'static str,
    source_text: &str,
    module_files: &FilesSourceText,
) -> CompilerCheckResult<()> {
    let parsed_file = parser::parse_source_file(current_file, source_text)?;

    // TODO: skip this step by making ModuleDefinition Clone'able, and move it to after expansion
    let current_file = match fs::canonicalize(current_file) {
        Ok(file) => file,
        Err(_) => {
            log::error!("Passed current file path is not a valid fs path");
            PathBuf::new()
        }
    };
    let module_definitions: Vec<FileDefinition> = module_files
        .iter()
        .filter(|(fname, _)| **fname != current_file.to_str().unwrap())
        .map(|(fname, text)| parse_file_string(fname, text).unwrap())
        .collect();
    let parsed_program = libra_parser::ast::Program {
        source_definitions: vec![parsed_file],
        lib_definitions: module_definitions,
    };
    let sender_opt = Some(Address::parse_str("0x8572f83cee01047effd6e7d0b5c19743").unwrap());

    let check_res = check::check_parsed_program(parsed_program, sender_opt);
    check_res.map_err(|libra_errors| {
        let libra_error = libra_errors.get(0).unwrap().clone();
        let diagnostics = convert_error_into_diags(libra_error, source_text);
        // get first one
        vec![diagnostics.get(0).unwrap().clone()]
    })
}
