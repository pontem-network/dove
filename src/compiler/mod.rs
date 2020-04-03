use lsp_types::Diagnostic;
use move_lang::parser as libra_parser;

pub mod expansion;
pub mod parser;

pub type CompilerCheckResult<P> = Result<P, Vec<Diagnostic>>;

pub fn check_with_compiler(fname: &'static str, new_source_text: &str) -> CompilerCheckResult<()> {
    let parsed_file = parser::parse_source_file(fname, new_source_text)?;
    let parsed_program = libra_parser::ast::Program {
        source_definitions: vec![parsed_file],
        lib_definitions: vec![],
    };
    expansion::expand_program(parsed_program)?;
    Ok(())
}
