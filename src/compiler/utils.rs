use lsp_types::{Diagnostic, Position, Range};
use move_ir_types::location::Loc;
use move_lang::errors::Error;

fn count_line_col(source: &str, pos: usize) -> (u64, u64) {
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

fn location_into_range(loc: Loc, source: &str) -> Range {
    let (line_start, col_start) = count_line_col(source, loc.span().start().to_usize());
    let (line_end, col_end) = count_line_col(source, loc.span().end().to_usize());
    Range::new(
        Position::new(line_start, col_start),
        Position::new(line_end, col_end),
    )
}

pub fn convert_error_into_diags(error: Error, source: &str) -> Vec<Diagnostic> {
    error
        .into_iter()
        .map(|(loc, message)| {
            let range = location_into_range(loc, source);
            Diagnostic::new_simple(range, message)
        })
        .collect()
}
