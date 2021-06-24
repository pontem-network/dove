use move_lang::errors::Error;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{make_loc, parse_address, unexpected_token_error, parse_num, parse_byte_string, parse_comma_list};
use move_lang::expansion::{hex_string, byte_string};
use crate::parser::types::{Value, Value_};

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
            // Struct(Vec<Value>)
            todo!()
        }
        _ => return Err(unexpected_token_error(tokens, "one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`"))
    };

    Ok(Value::new(make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc()), val))
}


#[cfg(test)]
mod tests {
    use move_ir_types::location::{Loc, Span};
    use move_lang::errors::Error;
    use move_lang::parser::lexer::Lexer;
    use move_lang::shared::Address;

    use crate::parser::types::{Value, Value_};
    use crate::parser::value::parse_value;
    use move_core_types::account_address::AccountAddress;

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
        assert_eq!(success("0x1111111111111111111111111111111111111111111111111111111111111111"),
                   val(Value_::Address(addr("0x1111111111111111111111111111111111111111111111111111111111111111"))));
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
        assert_eq!(success("b\"PONT\""), val(Value_::Bytes("PONT".as_bytes().to_vec())));
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
            val(Value_::Vec(vec![val(Value_::Bool(true)), val(Value_::Bool(false)), val(Value_::Bool(true))]))
        );

        assert_eq!(
            success("[0, 42]"),
            val(Value_::Vec(vec![val(Value_::Num(0)), val(Value_::Num(42))]))
        );

        assert_eq!(
            success("[x\"00\", x\"00\"]"),
            val(Value_::Vec(vec![val(Value_::Bytes(vec![0x0])), val(Value_::Bytes(vec![0x0]))]))
        );

        assert_eq!(
            success("[[], []]"),
            val(Value_::Vec(vec![val(Value_::Vec(vec![])), val(Value_::Vec(vec![]))]))
        );

        assert_eq!(
            success("[0x1]"),
            val(Value_::Vec(vec![val(Value_::Address(Address::DIEM_CORE))]))
        );

        assert_eq!(fail("["), vec![
            (loc(1, 1), "Unexpected end-of-file".to_owned()),
            (loc(1, 1), "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned())
        ]);

        assert_eq!(fail("[,]"), vec![
            (loc(1, 1), "Expected a vector item expression".to_owned()),
        ]);
    }

}