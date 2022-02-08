use anyhow::Error;
use crate::context::Context;
use crate::call::cmd::{CallDeclaration, CallDeclarationCmd};
use crate::call::fn_call::{Config, make_function_call, make_script_call};
use crate::call::model::EnrichedTransaction;
use crate::call::parser::Call;

/// Bytecode.
pub mod bytecode;
/// Command helper.
pub mod cmd;
/// Function call.
pub mod fn_call;
/// Transaction model.
pub mod model;
/// Call parser.
pub mod parser;

/// Make transaction with given call declaration.
pub fn make_transaction(
    ctx: &Context,
    cmd: CallDeclarationCmd,
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let address_decl = ctx.address_declarations();
    let declaration = CallDeclaration::try_from((&address_decl, cmd))?;
    match declaration.call {
        Call::Function {
            address,
            module,
            func,
            type_tag,
            args,
        } => make_function_call(
            ctx,
            &address_decl,
            address,
            module,
            func,
            type_tag,
            args,
            declaration.package,
            cfg,
        ),
        Call::Script {
            name,
            type_tag,
            args,
        } => make_script_call(
            ctx,
            &address_decl,
            name,
            type_tag,
            args,
            declaration.package,
            cfg,
        ),
    }
}
