use anyhow::Error;
use move_cli::on_disk_state_view::OnDiskStateView;
use move_cli::commands::{explain_execution_error, explain_execution_effects, maybe_commit_effects};
use move_vm_runtime::{logging::NoContextLog, move_vm::MoveVM};
use move_vm_types::gas_schedule::GasStatus;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{GasUnits, GasAlgebra, CostTable};
use move_core_types::language_storage::ModuleId;
use move_core_types::gas_schedule::GasCarrier;
use dove_lib::tx::model::{Transaction, Call};
use crate::context::Context;

/// Execute transaction.
pub fn execute_transaction(
    ctx: &Context,
    signers: Vec<AccountAddress>,
    transaction: Transaction,
    dependencies: Vec<CompiledModule>,
    verbose: bool,
    dry_run: bool,
) -> Result<(), Error> {
    let state = prepare_state(ctx, dependencies)?;
    let vm = MoveVM::new();
    let log_context = NoContextLog::new();
    let mut session = vm.new_session(&state);

    let cost_table = ctx.dialect.cost_table();
    let tx = transaction.inner();
    let (budget, mut status) = get_gas_status(&cost_table);
    let res = match tx.call {
        Call::Script { code } => session.execute_script(
            code,
            tx.type_args.clone(),
            tx.args.clone(),
            signers.clone(),
            &mut status,
            &log_context,
        ),
        Call::ScriptFunction {
            mod_address,
            mod_name,
            func_name,
        } => session.execute_script_function(
            &ModuleId::new(mod_address, mod_name),
            &func_name,
            tx.type_args.clone(),
            tx.args.clone(),
            signers.clone(),
            &mut status,
            &log_context,
        ),
    };

    if let Err(err) = res {
        let txn_args = tx
            .args
            .iter()
            .map(|arg| bcs::from_bytes(arg))
            .collect::<Result<Vec<_>, _>>()?;

        explain_execution_error(err, &state, &[], &[], &tx.type_args, &signers, &txn_args)
    } else {
        let (changeset, events) = session.finish().map_err(|e| e.into_vm_status())?;
        if verbose {
            explain_execution_effects(&changeset, &events, &state)?;
            println!("Spent {} gas.", budget.sub(status.remaining_gas()).get());
        }
        maybe_commit_effects(!dry_run, changeset, events, &state)
    }
}

fn prepare_state(
    ctx: &Context,
    dependencies: Vec<CompiledModule>,
) -> Result<OnDiskStateView, Error> {
    let state = OnDiskStateView::create(
        &ctx.path_for(&ctx.manifest.layout.storage_dir),
        &ctx.path_for(&ctx.manifest.layout.storage_dir),
    )?;
    let new_modules = dependencies
        .into_iter()
        .filter(|m| !state.has_module(&m.self_id()))
        .collect::<Vec<_>>();

    let mut serialized_modules = vec![];
    for module in new_modules {
        let mut module_bytes = vec![];
        module.serialize(&mut module_bytes)?;
        serialized_modules.push((module.self_id(), module_bytes));
    }
    state.save_modules(&serialized_modules)?;

    Ok(state)
}

fn get_gas_status(gas_schedule: &CostTable) -> (GasUnits<GasCarrier>, GasStatus) {
    let max_gas_budget = u64::MAX
        .checked_div(gas_schedule.gas_constants.gas_unit_scaling_factor)
        .unwrap();
    (
        GasUnits::new(max_gas_budget),
        GasStatus::new(gas_schedule, GasUnits::new(max_gas_budget)),
    )
}
