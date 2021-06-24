use crate::parser::types::Call;
use move_lang::errors::Error;
use move_lang::parser::syntax::{
    parse_module_access, parse_optional_type_args, consume_token, parse_comma_list,
};
use move_lang::parser::lexer::{Lexer, Tok};
use crate::parser::value::parse_value;

pub fn parse_call(tokens: &mut Lexer) -> Result<Call, Error> {
    let name = parse_module_access(tokens, || {
        panic!("parse_call with something other than a ModuleAccess")
    })?;

    let mut tys = None;
    if tokens.peek() == Tok::Less {
        tys = parse_optional_type_args(tokens)?;
    }

    dbg!(tokens.peek());
    let args = parse_comma_list(
        tokens,
        Tok::LParen,
        Tok::RParen,
        parse_value,
        "a call argument expression",
    )?;
    consume_token(tokens, Tok::Semicolon)?;

    Ok(Call {
        name,
        t_params: tys,
        params: args,
    })
}
