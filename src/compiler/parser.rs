use lsp_types::{Diagnostic, Position, Range};
use move_ir_types::location::Loc;
use move_lang::errors::Error;
use move_lang::parser::ast::FileDefinition;
use move_lang::parser::syntax::parse_file_string;
use move_lang::strip_comments_and_verify;

pub fn count_line_col(source: &str, pos: usize) -> (u64, u64) {
    let chars_before_pos = &source[..pos];

    let line = chars_before_pos
        .chars()
        .map(|chr| (chr == '\n') as u64)
        .sum::<u64>();

    let last_newline_pos = chars_before_pos.rfind(|chr| chr == '\n').unwrap_or(0);

    let col;
    if last_newline_pos == 0 {
        col = pos as u64;
    } else {
        col = (pos - last_newline_pos - 1) as u64
    }
    (line, col)
}

pub fn location_into_range(loc: Loc, source: &str) -> Range {
    let (line_start, col_start) = count_line_col(source, loc.span().start().to_usize());
    let (line_end, col_end) = count_line_col(source, loc.span().end().to_usize());
    Range::new(
        Position::new(line_start, col_start),
        Position::new(line_end, col_end),
    )
}

fn convert_error_into_diags(error: Error, source: &str) -> Vec<Diagnostic> {
    error
        .into_iter()
        .map(|(loc, message)| {
            let range = location_into_range(loc, source);
            Diagnostic::new_simple(range, message)
        })
        .collect()
}

fn parse_file(fname: &'static str, source: &str) -> Result<FileDefinition, Error> {
    let stripped_source = strip_comments_and_verify(fname, source)?;
    let parsed = parse_file_string(fname, &stripped_source)?;
    Ok(parsed)
}

pub fn parse_and_extract_diagnostics(
    fname: &'static str,
    source: &str,
) -> Result<FileDefinition, Vec<Diagnostic>> {
    parse_file(fname, source).map_err(|error| {
        let diags = convert_error_into_diags(error, source);
        // first one, for now
        if !diags.is_empty() {
            vec![diags.get(0).unwrap().clone()]
        } else {
            vec![]
        }
    })
}
