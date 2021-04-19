use std::str::FromStr;

use anyhow::{Result, Error, bail, anyhow};

use move_ir_types::location::Loc;
use move_core_types::language_storage::{TypeTag, StructTag};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_ir_types::ast::Type;
use lang::lexer::unwrap_spanned_ty;

#[derive(Debug)]
pub struct TypeTagQuery {
    tt: TypeTag,

    /// Index of vector
    /// e.g.: `0x0::Mod::Res[i]`
    i: Option<u128>,
}

impl FromStr for TypeTagQuery {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        self::parse(s)
    }
}

impl From<TypeTagQuery> for (TypeTag, Option<u128>) {
    fn from(query: TypeTagQuery) -> Self {
        (query.tt, query.i)
    }
}

impl TypeTagQuery {
    pub fn into_inner(self) -> (TypeTag, Option<u128>) {
        self.into()
    }
}

pub fn parse(s: &str) -> Result<TypeTagQuery, Error> {
    let map_err = |err: Vec<(Loc, String)>| {
        anyhow!("Query parsing error:\n\t{:}", {
            let strs: Vec<_> = err
                .into_iter()
                .map(|(loc, msg)| format!("{}: {}", loc.span(), msg))
                .collect();
            strs.join("\n\t")
        })
    };

    let q = {
        #[cfg(feature = "ps_address")]
        {
            let res = lang::compiler::address::ss58::replace_ss58_addresses(
                &s,
                &mut Default::default(),
            );
            log::debug!("in-query address decoded: {:}", res);
            res
        }
        #[cfg(not(feature = "ps_address"))]
        {
            s
        }
    };

    let mut lexer = Lexer::new(&q, "query", Default::default());
    lexer.advance().map_err(map_err)?;

    let ty = parse_type(&mut lexer).map_err(map_err)?;
    let tt = unwrap_spanned_ty(ty)?;

    let mut i = None;
    while lexer.peek() != Tok::EOF {
        let tok = lexer.peek();
        lexer.advance().map_err(map_err)?;

        match tok {
            Tok::LBracket => i = parse_num(&mut lexer).ok(),
            _ => break,
        }
    }

    Ok(TypeTagQuery { tt, i })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(parse("0x1::Foo").is_err());

        let inputs = [
            "0x1::Foo::Res",
            "0x1::Foo::Res<Bar::Struct>",
            "0x1::Foo::Res<0x1::Bar::Struct>",
            "0x1::Foo::Res<0x1::Bar::T>[42]",
            "0x1::Foo::Res<0x1::Bar::T<u128>>[42]",
            "0x1::Foo::Res<Bar::T<u128>>",
            "0x1::Foo::Res<Vec<u128>>",
            "0x1::Foo::Res<Vec<u128>>[42]",
            "0x1::Foo::Bar::Ignored<Parts>",
        ];

        inputs
            .iter()
            .cloned()
            .map(|inp| (inp, parse(inp)))
            .for_each(|(inp, res)| {
                assert!(res.is_ok(), "failed on '{}'", inp);
                println!("{:?}", res.unwrap());
            });
    }

    #[test]
    fn test_parse_ss58() {
        // //Alice/ pub: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY =>
        // 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D
        const ADDR: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        assert!(parse("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::Foo").is_err());

        let inputs = [
            &format!("{}::Foo::Res", ADDR),
            &format!("{}::Foo::Res<Bar::Struct>", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::Struct>", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::T>[42]", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::T<u128>>[42]", ADDR),
            &format!("{}::Foo::Res<Bar::T<u128>>", ADDR),
            &format!("{}::Foo::Res<Vec<u128>>", ADDR),
            &format!("{}::Foo::Res<Vec<u128>>[42]", ADDR),
            &format!("{}::Foo::Bar::Ignored<Parts>", ADDR),
        ];

        inputs
            .iter()
            .cloned()
            .map(|inp| (inp, parse(inp)))
            .for_each(|(inp, res)| {
                assert!(res.is_ok(), "failed on '{}'", inp);
                println!("{:?}", res.unwrap());
            });
    }
}
