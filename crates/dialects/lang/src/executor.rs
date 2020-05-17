use crate::types::{AccountAddress, VMResult};
use libra_types::vm_error::StatusCode;
use libra_types::write_set::WriteOp;
use utils::FilePath;
use vm::errors::{vm_error, Location};

use crate::changes::{changes_into_writeset, ResourceChange};
use shared::errors::CompilerError;

pub fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: String,
    genesis: Vec<ResourceChange>,
) -> Result<VMResult<Vec<ResourceChange>>, Vec<CompilerError>> {
    let sender = AccountAddress::from_hex_literal(&sender).unwrap();

    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        crate::check_and_generate_bytecode(fname, &script_text, deps, sender.into())?;

    let write_set = changes_into_writeset(genesis);
    let network_state = crate::prepare_fake_network_state(compiled_modules, write_set);

    let serialized_script = crate::serialize_script(compiled_script);
    let changed_resources =
        match crate::execute_script(sender, &network_state, serialized_script, vec![]) {
            Ok(ws) => ws,
            Err(vm_status) => return Ok(Err(vm_status)),
        };
    let mut changes = vec![];
    for (_, global_val) in changed_resources {
        match global_val {
            None => {
                // deletion is not yet supported
                continue;
            }
            Some((struct_type, global_val)) => {
                if !global_val.is_clean().unwrap() {
                    // into_owned_struct will check if all references are properly released
                    // at the end of a transaction
                    let data = global_val.into_owned_struct().unwrap();
                    let val = match data.simple_serialize(&struct_type) {
                        Some(blob) => blob,
                        None => {
                            return Ok(Err(vm_error(
                                Location::new(),
                                StatusCode::VALUE_SERIALIZATION_ERROR,
                            )));
                        }
                    };
                    let change = ResourceChange::new(struct_type, WriteOp::Value(val));
                    changes.push(change);
                }
            }
        }
    }

    Ok(Ok(changes))
}
