use std::str::FromStr;

use anyhow::{Context, Result};

use dialects::base::Dialect;
use dialects::DialectName;
use dialects::lang::{into_exec_compiler_error};
use dialects::shared::{AddressMap};
use lang::compiler::compile_to_prebytecode_program;
use utils::MoveFile;

use crate::session::{init_execution_session, ExecutionSession};

use crate::explain::ExecutionResult;
use crate::execution::{FakeRemoteCache, execute_script};
use std::collections::BTreeMap;

pub mod execution;
pub mod explain;
pub mod session;

pub fn compile_and_run_first_script(
    file: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    args: Vec<String>,
) -> Result<ExecutionResult> {
    compile_and_execute(file, deps, dialect, sender, args, vec![], run_first_script)
}

pub fn compile_and_run_scripts(
    file: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    test_names: Vec<String>,
) -> Result<BTreeMap<String, ExecutionResult>> {
    compile_and_execute(
        file,
        deps,
        dialect,
        sender,
        vec![],
        test_names,
        run_all_scripts_as_tests,
    )
}

fn compile_and_execute<R, F>(
    file: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    args: Vec<String>,
    test_names: Vec<String>,
    execute: F,
) -> Result<R>
where
    F: FnOnce(ExecutionSession, Box<dyn Dialect>, Vec<String>) -> Result<R>,
{
    let dialect = DialectName::from_str(dialect)?.get_dialect();

    // let mut lowered_genesis_changes = Vec::with_capacity(initial_genesis_changes.len());
    // for (i, change) in initial_genesis_changes.into_iter().enumerate() {
    //     let provided_address = dialect
    //         .normalize_account_address(&change.account)
    //         .with_context(|| format!("Invalid genesis entry {}: Account address is invalid for the selected dialect", i))?;
    //     address_map.insert(provided_address);
    //     // lowered_genesis_changes.push(change.with_replaced_addresses(&address_map.forward()));
    // }

    let provided_sender_address = dialect
        .normalize_account_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;

    let mut address_map = AddressMap::default();
    address_map.insert(provided_sender_address.clone());

    let (program, comments, project_source_map) = compile_to_prebytecode_program(
        dialect.as_ref(),
        file,
        deps,
        provided_sender_address.clone(),
    )?;
    let session = init_execution_session(program, comments, provided_sender_address, args)
        .map_err(|errors| into_exec_compiler_error(errors, project_source_map))?;
    execute(session, dialect, test_names)

    // let ChainStateChanges {
    //     resource_changes,
    //     gas_spent,
    //     events,
    // } = Chain;
    // let normalized_changes: Vec<_> = resource_changes
    //     .into_iter()
    //     // .map(|change| change.with_replaced_addresses(&address_map.reversed()))
    //     .collect();
    // Ok(res)
}

fn run_first_script(
    execution_session: ExecutionSession,
    dialect: Box<dyn Dialect>,
    _test_names: Vec<String>,
) -> Result<ExecutionResult> {
    let data_store = FakeRemoteCache::new(execution_session.modules())?;
    let script_args = execution_session.arguments()?;

    let (_, script, meta) = execution_session.scripts().remove(0);
    execute_script(meta, &data_store, script, script_args, dialect.cost_table())
}

fn run_all_scripts_as_tests(
    session: ExecutionSession,
    dialect: Box<dyn Dialect>,
    test_names: Vec<String>,
) -> Result<BTreeMap<String, ExecutionResult>> {
    let mut results = BTreeMap::new();
    for (name, script, meta) in session.scripts() {
        if !test_names.is_empty() && !test_names.contains(&name) {
            continue;
        }
        let data_store = FakeRemoteCache::new(session.modules())?;
        let res = execute_script(meta, &data_store, script, vec![], dialect.cost_table())?;
        results.insert(name, res);
    }
    Ok(results)
}
