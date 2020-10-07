use std::str::FromStr;

use anyhow::{Context, Result};

use dialects::DialectName;
use dialects::lang::{into_exec_compiler_error};
use dialects::shared::{AddressMap};
use lang::compiler::compile_to_prebytecode_program;
use utils::MoveFile;

use crate::session::init_execution_session;

use crate::explain::{PipelineExecutionResult, StepExecutionResult};
use crate::execution::{FakeRemoteCache, execute_script};
use move_vm_types::gas_schedule::CostStrategy;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};

pub mod execution;
pub mod explain;
pub mod oracles;
pub mod session;

pub fn compile_and_run_scripts_in_file(
    file: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    args: Vec<String>,
) -> Result<PipelineExecutionResult> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();

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
    let execution_session =
        init_execution_session(program, comments, provided_sender_address, args)
            .map_err(|errors| into_exec_compiler_error(errors, project_source_map))?;

    let mut data_store = FakeRemoteCache::new(execution_session.modules())?;
    let mut script_args = execution_session.arguments()?;

    if execution_session.scripts().is_empty() {
        return Err(anyhow::anyhow!("No scripts found"));
    }

    let mut overall_gas_spent = 0;
    let mut step_results = vec![];
    for (name, script, meta) in execution_session.scripts() {
        let total_gas = 1_000_000;
        let cost_table = dialect.cost_table();
        let mut cost_strategy = CostStrategy::transaction(&cost_table, GasUnits::new(total_gas));
        let step_result = execute_script(
            meta,
            &mut data_store,
            script,
            script_args,
            &mut cost_strategy,
        )?;
        script_args = vec![];

        let gas_spent = total_gas - cost_strategy.remaining_gas().get();
        overall_gas_spent += gas_spent;

        let is_error = matches!(step_result, StepExecutionResult::Error(_));
        step_results.push((name, step_result));
        if is_error {
            break;
        }
    }
    Ok(PipelineExecutionResult::new(
        step_results,
        overall_gas_spent,
    ))
}
