use move_lang::errors::Error;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{consume_token, parse_identifier};

use crate::parser::types::Var;
use crate::parser::value::parse_value;

pub fn parse_var(tokens: &mut Lexer) -> Result<Var, Error> {
    let name = parse_identifier(tokens)?;
    consume_token(tokens, Tok::Equal)?;
    let value = parse_value(tokens)?;
    consume_token(tokens, Tok::Semicolon)?;
    Ok(Var { name, value })
}

#[cfg(test)]
mod tests {
    use move_ir_types::location::{Loc, Span};
    use move_lang::errors::Error;
    use move_lang::parser::lexer::Lexer;

    use crate::parser::types::{Var, Value_};
    use crate::parser::var::parse_var;
    use move_lang::shared::Name;
    use crate::parser::types::Value;

    #[test]
    pub fn test_parse_var() {
        assert_eq!(
            fail("1=1"),
            vec![
                (loc(0, 1), "Unexpected '1'".to_owned()),
                (loc(0, 1), "Expected an identifier".to_owned()),
            ]
        );

        assert_eq!(
            fail("foo"),
            vec![
                (loc(3, 3), "Unexpected end-of-file".to_owned()),
                (loc(3, 3), "Expected '='".to_owned()),
            ]
        );

        assert_eq!(
            fail("foo:"),
            vec![
                (loc(3, 4), "Unexpected ':'".to_owned()),
                (loc(3, 4), "Expected '='".to_owned()),
            ]
        );

        assert_eq!(
            fail("foo="),
            vec![
                (loc(4, 4), "Unexpected end-of-file".to_owned()),
                (loc(4, 4), "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned()),
            ]
        );

        assert_eq!(
            fail("foo=[]"),
            vec![
                (loc(6, 6), "Unexpected end-of-file".to_owned()),
                (loc(6, 6), "Expected ';'".to_owned()),
            ]
        );

        assert_eq!(
            success("foo=1;"),
            Var {
                name: name("foo"),
                value: val(Value_::Num(1))
            }
        );
    }

    fn success(val: &str) -> Var {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        parse_var(&mut lexer).unwrap()
    }

    fn fail(val: &str) -> Error {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        parse_var(&mut lexer).unwrap_err()
    }

    fn loc(start: u32, end: u32) -> Loc {
        Loc::new("dsl", Span::new(start, end))
    }

    fn name(name: &str) -> Name {
        Name::new(loc(0, 0), name.to_owned())
    }

    fn val(val: Value_) -> Value {
        Value::new(loc(0, 0), val)
    }
}
