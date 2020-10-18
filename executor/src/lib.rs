use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::parser::parse_transaction_argument;
use move_core_types::transaction_argument::TransactionArgument;
use move_vm_types::values::Value;

use crate::explain::PipelineExecutionResult;
use crate::session::SessionBuilder;
use lang::compiler::dialects::DialectName;
use lang::compiler::file::MoveFile;

pub mod execution;
pub mod explain;
pub mod meta;
pub mod oracles;
pub mod session;

/// Convert the transaction arguments into move values.
fn convert_txn_arg(arg: TransactionArgument) -> Result<Value> {
    Ok(match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => {
            return Err(anyhow::Error::msg(format!(
                "Unexpected transaction argument: {:?}",
                arg
            )));
        }
    })
}

pub fn parse_script_arguments(passed_args: Vec<String>) -> Result<Vec<Value>> {
    passed_args
        .into_iter()
        .map(|arg| parse_transaction_argument(&arg).and_then(convert_txn_arg))
        .collect()
}

pub fn execute_script(
    script: MoveFile,
    deps: Vec<MoveFile>,
    dialect: &str,
    sender: &str,
    args: Vec<String>,
) -> Result<PipelineExecutionResult> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let sender = dialect
        .normalize_account_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;
    let script_args = parse_script_arguments(args)?;

    // todo fix module reloading.
    let mut sources = Vec::with_capacity(deps.len() + 1);
    sources.push(script);
    sources.extend(deps);
    let session = SessionBuilder::new(dialect.as_ref(), &sender)
        .build(sources, vec![])?;

    if !session.is_executable() {
        return Err(anyhow::anyhow!("No scripts found"));
    }

    session.execute(script_args, dialect.cost_table())
}
