use anyhow::{Context, Result};
use language_e2e_tests::data_store::FakeDataStore;
use libra_types::{transaction::TransactionArgument, write_set::WriteSet};
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{CostTable, GasAlgebra, GasUnits};

use move_lang::{compiled_unit::CompiledUnit, errors::Error, to_bytecode};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;
use vm::file_format::CompiledScript;
use vm::CompiledModule;

use crate::lang::PreBytecodeProgram;

use crate::lang::resources::ResourceWriteOp;
use crate::shared::results::{ChainStateChanges, ResourceChange, ResourceType};

use libra_types::write_set::WriteOp;
use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::value::MoveTypeLayout;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::TransactionEffects;

use vm::errors::{Location, PartialVMError, VMResult};

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

fn serialize_val(val: Value, layout: MoveTypeLayout) -> VMResult<Vec<u8>> {
    match val.simple_serialize(&layout) {
        Some(blob) => Ok(blob),
        None => {
            let partial_vm_error = PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR);
            Err(partial_vm_error.finish(Location::Undefined))
        }
    }
}

type ResourceChangeData = (TypeTag, Option<(MoveTypeLayout, Value)>);

fn into_resource_changes(
    effect_resources: Vec<(AccountAddress, Vec<ResourceChangeData>)>,
) -> VMResult<Vec<ResourceChange>> {
    let mut resources = vec![];
    for (addr, resource_vals) in effect_resources {
        let account_address = format!("0x{}", addr.to_string());
        for (ty, val) in resource_vals {
            let resource_type = ResourceType::new(ty);
            match val {
                None => resources.push(ResourceChange::new(
                    account_address.clone(),
                    resource_type,
                    ResourceWriteOp(WriteOp::Deletion),
                )),
                Some((layout, val)) => {
                    let val = serialize_val(val, layout)?;
                    resources.push(ResourceChange::new(
                        account_address.clone(),
                        resource_type,
                        ResourceWriteOp(WriteOp::Value(val)),
                    ));
                }
            }
        }
    }
    Ok(resources)
}

#[derive(Debug, serde::Serialize)]
pub struct Event {
    guid: Vec<u8>,
    seq_num: u64,
    ty_tag: TypeTag,
    val: Vec<u8>,
    caller: Option<ModuleId>,
}

type EventData = (
    Vec<u8>,
    u64,
    TypeTag,
    MoveTypeLayout,
    Value,
    Option<ModuleId>,
);

fn into_events(effect_events: Vec<EventData>) -> VMResult<Vec<Event>> {
    let mut events = vec![];
    for (guid, seq_num, ty_tag, ty_layout, val, caller) in effect_events {
        let val = serialize_val(val, ty_layout)?;
        events.push(Event {
            guid,
            seq_num,
            ty_tag,
            val,
            caller,
        })
    }
    Ok(events)
}

fn execute_script_with_runtime_session(
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
    sender: AccountAddress,
    cost_strategy: &mut CostStrategy,
) -> VMResult<TransactionEffects> {
    let vm = MoveVM::new();
    let mut runtime_session = vm.new_session(data_store);

    runtime_session.execute_script(script, vec![], args, sender, cost_strategy)?;
    runtime_session.finish()
}

pub fn chain_state_changes(
    effects: TransactionEffects,
    gas_spent: u64,
) -> VMResult<ChainStateChanges> {
    let TransactionEffects {
        resources, events, ..
    } = effects;
    let resource_changes = into_resource_changes(resources)?;
    let events = into_events(events)?;
    Ok(ChainStateChanges {
        resource_changes,
        events,
        gas_spent,
    })
}

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
    cost_table: CostTable,
) -> Result<ChainStateChanges> {
    let total_gas = 1_000_000;
    let mut cost_strategy = CostStrategy::transaction(&cost_table, GasUnits::new(total_gas));

    let effects = execute_script_with_runtime_session(
        data_store,
        script,
        args,
        sender_address,
        &mut cost_strategy,
    )
    .map_err(|error| error.into_vm_status())
    .with_context(|| "Script execution error")?;
    let gas_spent = total_gas - cost_strategy.remaining_gas().get();

    let changes =
        chain_state_changes(effects, gas_spent).map_err(|error| error.into_vm_status())?;
    Ok(changes)
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
