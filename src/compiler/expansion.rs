use move_lang::parser as libra_parser;

use crate::compiler::CompilerCheckResult;

pub fn expand_program(_program: libra_parser::ast::Program) -> CompilerCheckResult<()> {
    Ok(())
}
