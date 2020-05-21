use anyhow::Result;
use dfin_libra_types::{
    transaction::{parse_as_transaction_argument, TransactionArgument},
    vm_error::StatusCode,
    write_set::{WriteOp, WriteSet},
};
use dfin_move_core_types::account_address::AccountAddress;
use dfin_move_vm_types::loaded_data::types::FatStructType;
use dfin_move_vm_types::{values::GlobalValue, values::Value};
use dfin_vm::errors::{vm_error, Location, VMResult};

use shared::results::ResourceChange;
use utils::FilePath;

use crate::dfina::resources::{ResourceStructType, ResourceWriteOp};
use crate::dfina::vm_status_into_exec_status;

fn convert_set_value(struct_type: FatStructType, val: GlobalValue) -> VMResult<ResourceChange> {
    // into_owned_struct will check if all references are properly released at the end of a transaction
    let data = val.into_owned_struct()?;
    let val = match data.simple_serialize(&struct_type) {
        Some(blob) => blob,
        None => {
            let vm_status = vm_error(Location::new(), StatusCode::VALUE_SERIALIZATION_ERROR);
            return Err(vm_status);
        }
    };
    let change = ResourceChange::new(
        ResourceStructType(struct_type),
        ResourceWriteOp(WriteOp::Value(val)),
    );
    Ok(change)
}

/// Convert the transaction arguments into move values.
fn convert_txn_arg(arg: TransactionArgument) -> Value {
    match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
    }
}

pub fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: String,
    genesis_write_set: WriteSet,
    args: Vec<String>,
) -> Result<Vec<ResourceChange>> {
    let sender =
        AccountAddress::from_hex_literal(&sender).expect("Should be validated in the caller");
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        crate::dfina::check_and_generate_bytecode(fname, &script_text, deps, sender)?;

    let network_state =
        crate::dfina::prepare_fake_network_state(compiled_modules, genesis_write_set);

    let serialized_script = crate::dfina::serialize_script(compiled_script)?;

    let mut script_args = Vec::with_capacity(args.len());
    for passed_arg in args {
        let transaction_argument = parse_as_transaction_argument(&passed_arg)?;
        let script_arg = convert_txn_arg(transaction_argument);
        script_args.push(script_arg);
    }
    let changed_resources =
        crate::dfina::execute_script(sender, &network_state, serialized_script, script_args)?;

    let mut changes = vec![];
    for (_, global_val) in changed_resources {
        match global_val {
            None => {
                // deletion is not yet supported
                continue;
            }
            Some((struct_type, global_val)) => {
                if !global_val.is_clean().map_err(vm_status_into_exec_status)? {
                    let change = convert_set_value(struct_type, global_val)
                        .map_err(vm_status_into_exec_status)?;
                    changes.push(change);
                }
            }
        }
    }
    Ok(changes)
}
