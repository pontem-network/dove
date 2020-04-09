use std::fs;

use lsp_types::Diagnostic;
use move_lang::parser as libra_parser;
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax::parse_file_string;

use crate::analysis::Analysis;
use crate::compiler::utils::leak_str;
use crate::utils::diagnostics::libra_error_into_diagnostic;

pub mod check;
pub mod parser;
pub mod utils;

pub type CompilerCheckResult<P> = Result<P, Vec<Diagnostic>>;

pub fn check_with_compiler(
    current_file: &'static str,
    source_text: &str,
    analysis: &Analysis,
) -> CompilerCheckResult<()> {
    let parsed_file = parser::parse_source_file(current_file, source_text)?;

    // TODO: skip this step by making ModuleDefinition Clone'able, and move it to after expansion
    let canonical_fname = match fs::canonicalize(current_file) {
        Ok(file) => file,
        Err(_) => {
            log::error!("Not a valid filesystem path {:?}", current_file);
            return Err(vec![]);
        }
    }
    .into_os_string()
    .into_string()
    .unwrap();
    let canonical_fname = leak_str(&canonical_fname);

    let module_definitions: Vec<FileDefinition> = analysis
        .available_module_files()
        .iter()
        .filter(|(fname, _)| **fname != canonical_fname)
        .map(|(fname, text)| parse_file_string(fname, text).unwrap())
        .collect();
    let parsed_program = libra_parser::ast::Program {
        source_definitions: vec![parsed_file],
        lib_definitions: module_definitions,
    };
    let sender_opt = Some(analysis.sender_address());

    let check_res = check::check_parsed_program(parsed_program, sender_opt);
    check_res.map_err(|libra_errors| {
        let mut all_files = analysis.available_module_files().clone();
        all_files.insert(canonical_fname, source_text.to_string());
        libra_errors
            .into_iter()
            .map(|error| libra_error_into_diagnostic(&all_files, error))
            .collect()
    })
}
