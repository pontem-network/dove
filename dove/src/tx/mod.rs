use crate::tx::cmd::{CallDeclarationCmd, CallDeclaration};
use crate::tx::model::EnrichedTransaction;
use anyhow::Error;
use std::convert::TryFrom;
use crate::context::Context;
use crate::tx::parser::Call;
use crate::tx::fn_call::{make_function_call, Config, make_script_call};

/// Command helper.
pub mod cmd;
/// Function call.
pub mod fn_call;
/// Transaction model.
pub mod model;
/// Call parser.
pub mod parser;
/// Execution unit resolver.
pub mod resolver;

/// Make transaction with given call declaration.
pub fn make_transaction(
    ctx: &Context,
    cmd: CallDeclarationCmd,
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let declaration = CallDeclaration::try_from(cmd)?;
    let addr = ctx.account_address()?;
    match declaration.call {
        Call::Function {
            address,
            module,
            func,
            type_tag,
            args,
        } => make_function_call(
            ctx,
            address.unwrap_or(addr),
            module,
            func,
            type_tag,
            args,
            declaration.file_name,
            cfg,
        ),
        Call::Script {
            name,
            type_tag,
            args,
        } => make_script_call(ctx, addr, name, type_tag, args, declaration.file_name, cfg),
    }
}
