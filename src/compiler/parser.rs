use move_lang::errors::Error;
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax::parse_file_string;
use move_lang::strip_comments_and_verify;

use crate::compiler::utils::convert_error_into_diags;
use crate::compiler::CompilerCheckResult;

fn parse_file(fname: &'static str, source_text: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, source_text)?;
    let parsed = parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}

pub fn parse_source_file(
    fname: &'static str,
    source_text: &str,
) -> CompilerCheckResult<FileDefinition> {
    parse_file(fname, source_text).map_err(|error| {
        let diagnostics = convert_error_into_diags(error, source_text);
        // get first one
        vec![diagnostics.get(0).unwrap().clone()]
    })
}
