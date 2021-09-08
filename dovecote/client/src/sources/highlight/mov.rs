use crate::sources::highlight::lexer::Lexer;
use crate::sources::highlight::Line;
use crate::sources::Error;

pub struct MoveParser {

}

pub fn mark_code<'input>(_lexer: Lexer<'input>) -> Result<Vec<Line<'input>>, Error> {
    todo!()
}

