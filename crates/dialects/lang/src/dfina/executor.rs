use anyhow::Result;
use dfinance_libra_types::{
    vm_error::StatusCode,
    write_set::{WriteOp, WriteSet},
};
use dfinance_move_core_types::account_address::AccountAddress;
use dfinance_move_vm_types::{loaded_data::types::FatStructType, values::GlobalValue};
use dfinance_vm::errors::{vm_error, Location, VMResult};

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

pub fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: String,
    genesis_write_set: WriteSet,
) -> Result<Vec<ResourceChange>> {
    let sender =
        AccountAddress::from_hex_literal(&sender).expect("Should be validated in the caller");
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        crate::dfina::check_and_generate_bytecode(fname, &script_text, deps, sender.into())?;

    let network_state =
        crate::dfina::prepare_fake_network_state(compiled_modules, genesis_write_set);

    let serialized_script = crate::dfina::serialize_script(compiled_script)?;
    let changed_resources =
        crate::dfina::execute_script(sender, &network_state, serialized_script, vec![])?;

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
