use anyhow::{Context, Result};
use orig_language_e2e_tests::data_store::FakeDataStore;
use orig_libra_types::{
    transaction::{parse_transaction_argument, TransactionArgument},
    vm_error::VMStatus,
    write_set::WriteSet,
};
use orig_move_core_types::account_address::AccountAddress;
use orig_move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use orig_move_lang::{compiled_unit::CompiledUnit, errors::Error, shared::Address, to_bytecode};
use orig_move_vm_runtime::move_vm::MoveVM;

use orig_move_vm_types::values::Value;
use orig_move_vm_types::{
    gas_schedule::{zero_cost_schedule, CostStrategy},
    transaction_metadata::TransactionMetadata,
};

use orig_vm::file_format::CompiledScript;
use orig_vm::CompiledModule;

use shared::errors::ExecCompilerError;
use shared::results::{ExecutionError, ResourceChange};
use utils::MoveFilePath;

use crate::libra::{
    check_defs, data_cache, into_exec_compiler_error, parse_files, PreBytecodeProgram,
};

pub fn vm_status_into_exec_status(vm_status: VMStatus) -> ExecutionError {
    ExecutionError {
        status: format!("{:?}", vm_status.major_status),
        sub_status: vm_status.sub_status,
        message: vm_status.message,
    }
}

pub fn generate_bytecode(
    program: PreBytecodeProgram,
) -> Result<(CompiledScript, Vec<CompiledModule>), Vec<Error>> {
    let mut units = to_bytecode::translate::program(program)?;
    let script = match units.remove(units.len() - 1) {
        CompiledUnit::Script { script, .. } => script,
        CompiledUnit::Module { .. } => unreachable!(),
    };
    let modules = units
        .into_iter()
        .map(|unit| match unit {
            CompiledUnit::Module { module, .. } => module,
            CompiledUnit::Script { .. } => unreachable!(),
        })
        .collect();
    Ok((script, modules))
}

pub fn check_and_generate_bytecode(
    fname: MoveFilePath,
    text: &str,
    deps: &[(MoveFilePath, String)],
    sender: AccountAddress,
) -> Result<(CompiledScript, Vec<CompiledModule>), ExecCompilerError> {
    let (mut script_defs, modules_defs, project_offsets_map) =
        parse_files((fname, text.to_owned()), deps, format!("0x{}", sender))?;
    script_defs.extend(modules_defs);

    let sender = Address::new(sender.into());
    let program = check_defs(script_defs, vec![], sender)
        .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
    generate_bytecode(program)
        .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map))
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

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
) -> Result<Vec<ResourceChange>> {
    let mut data_cache = data_cache::DataCache::new(data_store);

    let zero_cost_table = zero_cost_schedule();
    let mut cost_strategy = CostStrategy::system(&zero_cost_table, GasUnits::new(1_000_000));

    let txn_metadata = get_transaction_metadata(sender_address);

    let vm = MoveVM::new();
    vm.execute_script(
        script,
        vec![],
        args,
        &mut cost_strategy,
        &mut data_cache,
        &txn_metadata,
    )
    .map_err(vm_status_into_exec_status)
    .with_context(|| "Script execution error")?;

    data_cache
        .resource_changes()
        .map_err(vm_status_into_exec_status)
        .with_context(|| "Changeset serialization error")
}

/// Convert the transaction arguments into move values.
fn convert_txn_arg(arg: TransactionArgument) -> Value {
    match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => unimplemented!(),
    }
}

pub fn compile_and_run(
    script: (MoveFilePath, String),
    deps: &[(MoveFilePath, String)],
    sender: String,
    genesis_write_set: WriteSet,
    args: Vec<String>,
) -> Result<Vec<ResourceChange>> {
    let sender =
        AccountAddress::from_hex_literal(&sender).expect("Should be validated in the caller");
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        check_and_generate_bytecode(fname, &script_text, deps, sender)?;

    let network_state = prepare_fake_network_state(compiled_modules, genesis_write_set);

    let serialized_script =
        serialize_script(compiled_script).context("Script serialization error")?;

    let mut script_args = Vec::with_capacity(args.len());
    for passed_arg in args {
        let transaction_argument = parse_transaction_argument(&passed_arg)?;
        let script_arg = convert_txn_arg(transaction_argument);
        script_args.push(script_arg);
    }

    execute_script(sender, &network_state, serialized_script, script_args)
}
