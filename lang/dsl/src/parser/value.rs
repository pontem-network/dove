use move_lang::errors::Error;
use move_lang::expansion::{byte_string, hex_string};
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{
    consume_token, make_loc, parse_address, parse_comma_list, parse_identifier, parse_num,
    unexpected_token_error,
};
use move_lang::shared::Name;

use crate::parser::types::{Struct, Value, Value_};

pub fn parse_value(tokens: &mut Lexer) -> Result<Value, Error> {
    let start_loc = tokens.start_loc();

    let val = match tokens.peek() {
        Tok::IdentifierValue => {
            let var = Value_::Var(tokens.content().to_owned());
            tokens.advance()?;
            var
        }
        Tok::AddressValue => {
            Value_::Address(parse_address(tokens)?)
        }
        Tok::False => {
            tokens.advance()?;
            Value_::Bool(false)
        }
        Tok::True => {
            tokens.advance()?;
            Value_::Bool(true)
        }
        Tok::NumValue => {
            Value_::Num(parse_num(tokens)?)
        }
        Tok::ByteStringValue => {
            let s = tokens.content();

            let text = s[2..s.len() - 1].to_owned();

            let bytes = if s.starts_with("x\"") {
                tokens.advance()?;
                let loc = make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc());
                hex_string::decode(loc, &text).map_err(|mut errors| errors.remove(0))?
            } else {
                tokens.advance()?;
                let loc = make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc());
                byte_string::decode(loc, &text).map_err(|mut errors| errors.remove(0))?
            };

            Value_::Bytes(bytes)
        }
        Tok::LBracket => {
            Value_::Vec(parse_comma_list(
                tokens,
                Tok::LBracket,
                Tok::RBracket,
                parse_value,
                "a vector item expression",
            )?)
        }
        Tok::LBrace => {
            Value_::Struct(Struct {
                fields: parse_comma_list(
                    tokens,
                    Tok::LBrace,
                    Tok::RBrace,
                    parse_field,
                    "a field expression",
                )?
            })
        }
        _ => return Err(unexpected_token_error(tokens, "one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`"))
    };

    Ok(Value::new(
        make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc()),
        val,
    ))
}

fn parse_field(tokens: &mut Lexer) -> Result<(Name, Value), Error> {
    let name = parse_identifier(tokens)?;
    consume_token(tokens, Tok::Colon)?;
    let value = parse_value(tokens)?;
    Ok((name, value))
}

#[cfg(test)]
mod tests {
    use move_ir_types::location::{Loc, Span};
    use move_lang::errors::Error;
    use move_lang::parser::lexer::Lexer;
    use move_lang::shared::{Address, Name};

    use crate::parser::types::{Struct, Value, Value_};
    use crate::parser::value::parse_value;

    #[test]
    fn test_parse_empty() {
        assert_eq!(fail(""), vec![
            (loc(0, 0), "Unexpected end-of-file".to_owned()),
            (loc(0, 0), "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned())
        ]);
    }

    #[test]
    fn test_parse_ident() {
        assert_eq!(success("test_val"), val(Value_::Var("test_val".to_owned())));
        assert_eq!(success("init"), val(Value_::Var("init".to_owned())));
        assert_eq!(success("init_1"), val(Value_::Var("init_1".to_owned())));
    }

    #[test]
    fn test_parse_address() {
        assert_eq!(success("0x1"), val(Value_::Address(addr("0x1"))));
        assert_eq!(success("0x0"), val(Value_::Address(addr("0x0"))));
        assert_eq!(
            success("0x1111111111111111111111111111111111111111111111111111111111111111"),
            val(Value_::Address(addr(
                "0x1111111111111111111111111111111111111111111111111111111111111111"
            )))
        );
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(success("true"), val(Value_::Bool(true)));
        assert_eq!(success("false"), val(Value_::Bool(false)));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(success("0"), val(Value_::Num(0)));
        assert_eq!(success("13"), val(Value_::Num(13)));
        assert_eq!(success("42"), val(Value_::Num(42)));
        assert_eq!(fail("9999999999999999999999999999999999999999999"), vec![
            (loc(0, 43), "Invalid number literal. The given literal is too large to fit into the largest number type 'u128'".to_owned())
        ]);
    }

    #[test]
    fn test_bytes_string() {
        assert_eq!(
            success("b\"PONT\""),
            val(Value_::Bytes("PONT".as_bytes().to_vec()))
        );
        assert_eq!(success("x\"00\""), val(Value_::Bytes(vec![0x0])));
        assert_eq!(fail("x\"0\""), vec![
            (loc(0, 4), "Odd number of characters in hex string. Expected 2 hexadecimal digits for each byte".to_owned())
        ]);
    }

    #[test]
    fn test_parse_vec() {
        assert_eq!(success("[]"), val(Value_::Vec(vec![])));

        assert_eq!(
            success("[true, false, true, ]"),
            val(Value_::Vec(vec![
                val(Value_::Bool(true)),
                val(Value_::Bool(false)),
                val(Value_::Bool(true))
            ]))
        );

        assert_eq!(
            success("[0, 42]"),
            val(Value_::Vec(vec![val(Value_::Num(0)), val(Value_::Num(42))]))
        );

        assert_eq!(
            success("[x\"00\", x\"00\"]"),
            val(Value_::Vec(vec![
                val(Value_::Bytes(vec![0x0])),
                val(Value_::Bytes(vec![0x0]))
            ]))
        );

        assert_eq!(
            success("[[], []]"),
            val(Value_::Vec(vec![
                val(Value_::Vec(vec![])),
                val(Value_::Vec(vec![]))
            ]))
        );

        assert_eq!(
            success("[0x1]"),
            val(Value_::Vec(vec![val(Value_::Address(Address::DIEM_CORE))]))
        );

        assert_eq!(
            success("[{}]"),
            val(Value_::Vec(vec![val(Value_::Struct(Struct {
                fields: vec![]
            }))]))
        );

        assert_eq!(fail("["), vec![
            (loc(1, 1), "Unexpected end-of-file".to_owned()),
            (loc(1, 1), "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned())
        ]);

        assert_eq!(
            fail("[,]"),
            vec![(loc(1, 1), "Expected a vector item expression".to_owned()),]
        );
    }

    #[test]
    fn test_parse_struct() {
        assert_eq!(
            success("{}"),
            val(Value_::Struct(Struct { fields: vec![] }))
        );
        assert_eq!(
            success(
                "{\
                    foo: {
                        bar: []
                    }
                 }"
            ),
            val(Value_::Struct(Struct {
                fields: vec![(
                    name("foo"),
                    val(Value_::Struct(Struct {
                        fields: vec![(name("bar"), val(Value_::Vec(vec![])))]
                    }))
                )]
            }))
        );

        assert_eq!(
            fail("{"),
            vec![
                (loc(1, 1), "Unexpected end-of-file".to_owned()),
                (loc(1, 1), "Expected an identifier".to_owned())
            ]
        );

        assert_eq!(
            fail("{13"),
            vec![
                (loc(1, 3), "Unexpected '13'".to_owned()),
                (loc(1, 3), "Expected an identifier".to_owned())
            ]
        );

        assert_eq!(
            fail("{t"),
            vec![
                (loc(2, 2), "Unexpected end-of-file".to_owned()),
                (loc(2, 2), "Expected ':'".to_owned())
            ]
        );

        assert_eq!(
            fail("{t:"),
            vec![
                (loc(3, 3), "Unexpected end-of-file".to_owned()),
                (loc(3, 3), "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned())
            ]
        );

        assert_eq!(
            fail("{t:t"),
            vec![
                (loc(4, 4), "Expected '}'".to_owned()),
                (loc(0, 0), "To match this '{'".to_owned())
            ]
        );
    }

    fn success(val: &str) -> Value {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        parse_value(&mut lexer).unwrap()
    }

    fn fail(val: &str) -> Error {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        parse_value(&mut lexer).unwrap_err()
    }

    fn loc(start: u32, end: u32) -> Loc {
        Loc::new("dsl", Span::new(start, end))
    }

    fn val(val: Value_) -> Value {
        Value::new(loc(0, 0), val)
    }

    fn addr(addr: &str) -> Address {
        Address::parse_str(addr).unwrap()
    }

    fn name(name: &str) -> Name {
        Name::new(loc(0, 0), name.to_owned())
    }
}
