use anyhow::{Context, Result};
use language_e2e_tests::data_store::FakeDataStore;
use libra_types::{transaction::TransactionArgument, vm_error::VMStatus, write_set::WriteSet};
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{CostTable, GasAlgebra, GasUnits};

use move_lang::{compiled_unit::CompiledUnit, errors::Error, to_bytecode};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;
use vm::file_format::CompiledScript;
use vm::CompiledModule;

use crate::lang::{data_cache, PreBytecodeProgram};

use crate::shared::results::{ChainStateChanges, ExecutionError};

pub fn vm_status_into_exec_status(vm_status: VMStatus) -> ExecutionError {
    ExecutionError {
        status: format!("{:?}", vm_status.major_status),
        sub_status: vm_status.sub_status,
        message: vm_status.message,
    }
}

pub fn generate_bytecode(
    program: PreBytecodeProgram,
) -> Result<(Option<CompiledScript>, Vec<CompiledModule>), Vec<Error>> {
    let units = to_bytecode::translate::program(program)?;

    let mut gen_script = None;
    let mut gen_modules = vec![];
    for unit in units {
        match unit {
            CompiledUnit::Module { module, .. } => gen_modules.push(module),
            CompiledUnit::Script { script, .. } => gen_script = Some(script),
        }
    }
    Ok((gen_script, gen_modules))
}

pub fn serialize_script(script: CompiledScript) -> Result<Vec<u8>> {
    let mut serialized = vec![];
    script.serialize(&mut serialized)?;
    Ok(serialized)
}

pub fn prepare_fake_network_state(
    modules: Vec<CompiledModule>,
    genesis_write_set: WriteSet,
) -> FakeDataStore {
    let mut network_state = FakeDataStore::default();
    for module in modules {
        network_state.add_module(&module.self_id(), &module);
    }
    network_state.add_write_set(&genesis_write_set);
    network_state
}

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
    cost_table: CostTable,
) -> Result<ChainStateChanges> {
    let mut data_cache = data_cache::DataCache::new(data_store);

    let total_gas = 1_000_000;
    let mut cost_strategy = CostStrategy::transaction(&cost_table, GasUnits::new(total_gas));

    let vm = MoveVM::new();
    vm.execute_script(
        script,
        vec![],
        args,
        sender_address,
        &mut data_cache,
        &mut cost_strategy,
    )
    .map_err(vm_status_into_exec_status)
    .with_context(|| "Script execution error")?;

    let events = data_cache.events();
    let resource_changes = data_cache
        .resource_changes()
        .map_err(vm_status_into_exec_status)
        .with_context(|| "Changeset serialization error")?;
    let gas_spent = total_gas - cost_strategy.remaining_gas().get();
    Ok(ChainStateChanges {
        resource_changes,
        gas_spent,
        events,
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
