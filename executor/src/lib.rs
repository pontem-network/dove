use std::str::FromStr;

use anyhow::{Context, Result};
use libra_types::transaction::parse_transaction_argument;

use dialects::base::Dialect;
use dialects::DialectName;
use dialects::lang::into_exec_compiler_error;
use dialects::shared::{AddressMap, ProvidedAccountAddress};
use lang::compiler::compile_to_prebytecode_program;
use utils::MoveFile;

use crate::execution::{convert_txn_arg, execute_script, ExecutionResult, FakeRemoteCache};
use crate::session::init_execution_session;

pub mod execution;
pub mod explain;
pub mod session;

pub fn compile_and_execute_script(
    script: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    args: Vec<String>,
) -> Result<ExecutionResult> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    // let initial_genesis_changes = serde_json::from_value::<Vec<ResourceChange>>(
    //     genesis_json_contents,
    // )
    // .with_context(|| {
    //     "Genesis JSON data is in invalid format (list of genesis resource objects required)"
    // })?;

    // let mut lowered_genesis_changes = Vec::with_capacity(initial_genesis_changes.len());
    // for (i, change) in initial_genesis_changes.into_iter().enumerate() {
    //     let provided_address = dialect
    //         .normalize_account_address(&change.account)
    //         .with_context(|| format!("Invalid genesis entry {}: Account address is invalid for the selected dialect", i))?;
    //     address_map.insert(provided_address);
    //     // lowered_genesis_changes.push(change.with_replaced_addresses(&address_map.forward()));
    // }

    let mut address_map = AddressMap::default();

    let provided_sender_address = dialect
        .normalize_account_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;
    address_map.insert(provided_sender_address.clone());

    let res = compile_and_run(
        dialect.as_ref(),
        script,
        deps,
        provided_sender_address,
        // lowered_genesis_changes,
        args,
    )?;

    // let ChainStateChanges {
    //     resource_changes,
    //     gas_spent,
    //     events,
    // } = Chain;
    // let normalized_changes: Vec<_> = resource_changes
    //     .into_iter()
    //     // .map(|change| change.with_replaced_addresses(&address_map.reversed()))
    //     .collect();
    Ok(res)
}

fn compile_and_run(
    dialect: &dyn Dialect,
    script_file: MoveFile,
    deps: &[MoveFile],
    provided_sender: ProvidedAccountAddress,
    args: Vec<String>,
) -> Result<ExecutionResult> {
    let (program, comments, project_source_map) =
        compile_to_prebytecode_program(dialect, script_file, deps, provided_sender.clone())?;
    let execution = init_execution_session(program, comments, provided_sender)
        .map_err(|errors| into_exec_compiler_error(errors, project_source_map))?;

    let data_store = FakeRemoteCache::new(execution.modules())?;

    let mut script_args = Vec::with_capacity(args.len());
    for passed_arg in args {
        let transaction_argument = parse_transaction_argument(&passed_arg)?;
        let script_arg = convert_txn_arg(transaction_argument);
        script_args.push(script_arg);
    }

    let (script, meta) = execution.into_script()?;
    execute_script(meta, &data_store, script, script_args, dialect.cost_table())
}
