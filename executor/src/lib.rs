use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::parser::parse_transaction_argument;
use move_core_types::transaction_argument::TransactionArgument;
use move_vm_types::values::Value;
use utils::MoveFile;

use crate::explain::PipelineExecutionResult;
use crate::session::{ExecutionSession, init_execution_session};
use lang::compiler::errors::into_exec_compiler_error;
use lang::compiler::parser::compile_to_prebytecode_program;
use lang::compiler::dialects::{DialectName, Dialect};
use lang::file::MvFile;

pub mod execution;
pub mod explain;
pub mod meta;
pub mod oracles;
pub mod session;

pub fn compile_and_initialize_file_execution_session(
    file: MvFile,
    deps: &[MvFile],
    sender: &str,
    dialect: &dyn Dialect,
) -> Result<ExecutionSession> {
    let provided_sender_address = dialect
        .normalize_account_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;

    let (program, comments, project_source_map) =
        compile_to_prebytecode_program(dialect, file, deps, provided_sender_address.clone())?;
    init_execution_session(program, comments, provided_sender_address).map_err(|errors| {
        anyhow::Error::new(into_exec_compiler_error(errors, project_source_map))
    })
}

/// Convert the transaction arguments into move values.
pub fn convert_txn_arg(arg: TransactionArgument) -> Value {
    match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => unimplemented!(),
    }
}

pub fn parse_script_arguments(passed_args: Vec<String>) -> Result<Vec<Value>> {
    let mut script_args = Vec::with_capacity(passed_args.len());
    for passed_arg in &passed_args {
        let transaction_argument = parse_transaction_argument(passed_arg)?;
        let script_arg = convert_txn_arg(transaction_argument);
        script_args.push(script_arg);
    }
    Ok(script_args)
}

pub fn compile_and_run_scripts_in_file(
    script: MvFile,
    deps: &[MvFile],
    dialect: &str,
    sender: &str,
    args: Vec<String>,
) -> Result<PipelineExecutionResult> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let file_execution_session =
        compile_and_initialize_file_execution_session(script, deps, sender, dialect.as_ref())?;

    let script_args = parse_script_arguments(args)?;
    if !file_execution_session.is_executable() {
        return Err(anyhow::anyhow!("No scripts found"));
    }

    file_execution_session.execute(script_args, dialect.cost_table())
}

pub fn compile_and_run_file_as_test(
    file: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
) -> Result<PipelineExecutionResult> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let file_execution_session =
        compile_and_initialize_file_execution_session(file, deps, sender, dialect.as_ref())?;

    file_execution_session.execute(vec![], dialect.cost_table())
}
