use move_lang::errors::{Error, FilesSourceText};
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax::parse_file_string;
use move_lang::strip_comments_and_verify;

use crate::compiler::CompilerCheckResult;
use crate::utils::diagnostics::libra_error_into_diagnostic;

fn parse_file(fname: &'static str, source_text: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, source_text)?;
    let parsed = parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}

pub fn parse_source_file(
    canonical_fname: &'static str,
    source_text: &str,
) -> CompilerCheckResult<FileDefinition> {
    parse_file(canonical_fname, source_text).map_err(|error| {
        let mut files = FilesSourceText::with_capacity(1);
        files.insert(canonical_fname, source_text.to_string());

        let parsing_diagnostic = libra_error_into_diagnostic(&files, error);
        vec![parsing_diagnostic]
    })
}
