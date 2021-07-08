use move_lang::errors::Error;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{
    consume_token, parse_address, parse_identifier, parse_type, unexpected_token_error,
};

use crate::parser::types::ResourceStore;
use crate::parser::value::parse_value;

pub const MOVE_TO: &str = "move_to";
pub const DROP: &str = "drop";

pub fn parse_move_to(tokens: &mut Lexer) -> Result<ResourceStore, Error> {
    let name = parse_identifier(tokens)?;
    if name.value.as_str() != MOVE_TO {
        return Err(unexpected_token_error(tokens, "an `move_to`"));
    }
    consume_token(tokens, Tok::LParen)?;
    let addr = parse_address(tokens)?;
    consume_token(tokens, Tok::Comma)?;

    let tp = parse_type(tokens)?;
    let value = parse_value(tokens)?;
    consume_token(tokens, Tok::RParen)?;
    consume_token(tokens, Tok::Semicolon)?;
    Ok(ResourceStore {
        address: addr,
        tp,
        value: Some(value),
    })
}

pub fn parse_drop(tokens: &mut Lexer) -> Result<ResourceStore, Error> {
    let name = parse_identifier(tokens)?;
    if name.value.as_str() != DROP {
        return Err(unexpected_token_error(tokens, "an `drop`"));
    }
    consume_token(tokens, Tok::Less)?;
    let tp = parse_type(tokens)?;
    consume_token(tokens, Tok::Greater)?;

    consume_token(tokens, Tok::LParen)?;
    let addr = parse_address(tokens)?;
    consume_token(tokens, Tok::RParen)?;
    consume_token(tokens, Tok::Semicolon)?;

    Ok(ResourceStore {
        address: addr,
        tp,
        value: None,
    })
}

#[cfg(test)]
pub mod tests {
    use move_lang::errors::Error;
    use move_lang::parser::ast::{ModuleAccess, ModuleAccess_, ModuleIdent, Type, Type_};
    use move_lang::parser::lexer::Lexer;
    use move_lang::parser::syntax::spanned;
    use move_lang::shared::Address;

    use crate::parser::store::{MOVE_TO, parse_drop, parse_move_to};
    use crate::parser::types::{ResourceStore, Struct, Value_};
    use crate::parser::value::tests::{addr, loc, name, val};

    #[test]
    pub fn test_parse_move_to() {
        assert_eq!(
            success("move_to(0x1, 0x1::Block::Block {value: 0});"),
            ResourceStore {
                address: Address::DIEM_CORE,
                tp: tp_mod_access(
                    access_addr_mod_name(13, 23, 13, 23, 18, 23, "0x1", "Block", "Block"),
                    vec![]
                ),
                value: Some(val(Value_::Struct(Struct {
                    fields: vec![(name("value"), val(Value_::Num(0)))]
                }))),
            }
        );

        assert_eq!(
            fail("move_to"),
            vec![
                (loc(7, 7), "Unexpected end-of-file".to_owned()),
                (loc(7, 7), "Expected '('".to_owned()),
            ]
        );

        assert_eq!(
            fail("move_to("),
            vec![
                (loc(8, 8), "Unexpected end-of-file".to_owned()),
                (loc(8, 8), "Expected an account address value".to_owned()),
            ]
        );

        assert_eq!(
            fail("move_to(1);"),
            vec![
                (loc(8, 9), "Unexpected '1'".to_owned()),
                (loc(8, 9), "Expected an account address value".to_owned()),
            ]
        );

        assert_eq!(
            fail("move_to(0x1);"),
            vec![
                (loc(11, 12), "Unexpected ')'".to_owned()),
                (loc(11, 12), "Expected ','".to_owned()),
            ]
        );

        assert_eq!(
            fail("move_to(0x1,);"),
            vec![
                (loc(12, 13), "Unexpected ')'".to_owned()),
                (loc(12, 13), "Expected a type name".to_owned()),
            ]
        );

        assert_eq!(
            fail("move_to(0x1, Block);"),
            vec![
                (loc(18, 19),  "Unexpected ')'".to_owned()),
                (loc(18, 19),   "Expected one of `identifier`, `address`, `bool`, `number`, `vector`, `struct`, `byte string` or `byte string`".to_owned()),
            ]);

        assert_eq!(
            fail("move_to(0x1, Block 1, 1);"),
            vec![
                (loc(20, 21), "Unexpected ','".to_owned()),
                (loc(20, 21), "Expected ')'".to_owned()),
            ]
        );
    }

    #[test]
    pub fn test_parse_drop() {
        assert_eq!(
            success("drop<0x1::Block::Block>(0x1);"),
            ResourceStore {
                address: Address::DIEM_CORE,
                tp: tp_mod_access(
                    access_addr_mod_name(13, 23, 13, 23, 18, 23, "0x1", "Block", "Block"),
                    vec![]
                ),
                value: None,
            }
        );

        assert_eq!(
            fail("drop"),
            vec![
                (loc(4, 4), "Unexpected end-of-file".to_owned()),
                (loc(4, 4), "Expected '<'".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop("),
            vec![
                (loc(4, 5), "Unexpected '('".to_owned()),
                (loc(4, 5), "Expected '<'".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop<u8"),
            vec![
                (loc(7, 7), "Unexpected end-of-file".to_owned()),
                (loc(7, 7), "Expected '>'".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop<u8>"),
            vec![
                (loc(8, 8), "Unexpected end-of-file".to_owned()),
                (loc(8, 8), "Expected '('".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop<u8>("),
            vec![
                (loc(9, 9), "Unexpected end-of-file".to_owned()),
                (loc(9, 9), "Expected an account address value".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop<u8>(1"),
            vec![
                (loc(9, 10), "Unexpected '1'".to_owned()),
                (loc(9, 10), "Expected an account address value".to_owned()),
            ]
        );

        assert_eq!(
            fail("drop<u8>(0x1"),
            vec![
                (loc(12, 12), "Unexpected end-of-file".to_owned()),
                (loc(12, 12), "Expected ')'".to_owned()),
            ]
        );
    }

    fn success(val: &str) -> ResourceStore {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        if val.starts_with(MOVE_TO) {
            parse_move_to(&mut lexer).unwrap()
        } else {
            parse_drop(&mut lexer).unwrap()
        }
    }

    fn fail(val: &str) -> Error {
        let mut lexer = Lexer::new(val, "dsl", Default::default());
        lexer.advance().unwrap();
        if val.starts_with(MOVE_TO) {
            parse_move_to(&mut lexer).unwrap_err()
        } else {
            parse_drop(&mut lexer).unwrap_err()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn access_addr_mod_name(
        start: u32,
        end: u32,
        start_1: u32,
        end_1: u32,
        start_2: u32,
        end_2: u32,
        a: &str,
        m: &str,
        n: &str,
    ) -> ModuleAccess {
        spanned(
            "dsl",
            start as usize,
            end as usize,
            ModuleAccess_::QualifiedModuleAccess(
                module(start_1, end_1, start_2, end_2, a, m),
                name(n),
            ),
        )
    }

    pub fn tp_mod_access(access: ModuleAccess, tps: Vec<Type>) -> Type {
        spanned("dsl", 0, 0, Type_::Apply(Box::new(access), tps))
    }

    pub fn module(
        start_1: u32,
        end_1: u32,
        start_2: u32,
        end_2: u32,
        adr: &str,
        name: &str,
    ) -> ModuleIdent {
        ModuleIdent {
            locs: (loc(start_1, end_1), loc(start_2, end_2)),
            value: (addr(adr), name.to_owned()),
        }
    }
}
