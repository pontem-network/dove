use move_lang::errors::Errors;
use move_lang::expansion as libra_expansion;
use move_lang::parser as libra_parser;
use move_lang::shared::Address;

pub fn check_parsed_program(
    prog: libra_parser::ast::Program,
    sender_opt: Option<Address>,
) -> Result<(), Errors> {
    let (_, errors) = libra_expansion::translate::program(prog, sender_opt);
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}
