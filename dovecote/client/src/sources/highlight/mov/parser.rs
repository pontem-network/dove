use crate::sources::highlight::mov::lexer::{Lexer, Tok};
use crate::sources::Error;

pub fn parse_num_value<'l, 'input>(lexer: &'l mut Lexer<'input>, line: &'input str) -> Result<&'input str, Error> {
    let start = lexer.start_loc();
    let mut end = start + lexer.content().len();
    if lexer.content() == "1" {
        // pont addr.
        let next = lexer.lookahead()?;
        if next == Tok::IdentifierValue {
            lexer.advance()?;
            end = start + lexer.content().len() + 1;
        }
    }
    Ok(&line[start..end])
}