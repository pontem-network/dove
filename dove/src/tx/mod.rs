use anyhow::Error;
use std::convert::TryFrom;
use dove_lib::tx::parser::Call;
use dove_lib::tx::fn_call::Config;
use crate::tx::cmd::{CallDeclarationCmd, CallDeclaration};
use crate::context::Context;
use crate::tx::fn_call::{make_function_call, make_script_call};
use dove_lib::tx::model::EnrichedTransaction;

/// Tx builder.
pub mod builder;
/// Command helper.
pub mod cmd;
/// Function call.
pub mod fn_call;
/// Execution unit resolver.
pub mod resolver;

/// Make transaction with given call declaration.
pub fn make_transaction(
    ctx: &Context,
    cmd: CallDeclarationCmd,
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let declaration = CallDeclaration::try_from((ctx, cmd))?;
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
