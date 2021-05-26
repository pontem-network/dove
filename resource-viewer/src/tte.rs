use std::str::FromStr;

use anyhow::{Result, Error, anyhow};

use move_ir_types::location::Loc;
use move_core_types::language_storage::TypeTag;
use lang::lexer::unwrap_spanned_ty;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{parse_type, parse_num};

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
