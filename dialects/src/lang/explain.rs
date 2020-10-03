use anyhow::Result;
use std::fmt::Write;

use vm::errors::VMError;
use move_core_types::vm_status::{VMStatus, AbortLocation, StatusCode};
use errmapgen::ErrorMapping;

use crate::lang::executor::FakeRemoteCache;
use move_vm_runtime::data_cache::TransactionEffects;

#[derive(Debug, serde::Serialize, Eq, PartialEq)]
pub struct AddressResourceChanges {
    pub address: String,
    pub changes: Vec<String>,
}

impl AddressResourceChanges {
    pub fn new<S: ToString>(address: S, changes: Vec<String>) -> Self {
        AddressResourceChanges {
            address: address.to_string(),
            changes,
        }
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ExplainedTransactionEffects {
    events: Vec<String>,
    resources: Vec<AddressResourceChanges>,
}

impl ExplainedTransactionEffects {
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }
    pub fn resources(&self) -> &Vec<AddressResourceChanges> {
        &self.resources
    }
}

pub fn explain_effects(
    effects: &TransactionEffects,
    state: &FakeRemoteCache,
) -> Result<ExplainedTransactionEffects> {
    // all module publishing happens via save_modules(), so effects shouldn't contain modules
    assert!(effects.modules.is_empty());

    let mut explained_effects = ExplainedTransactionEffects::default();
    if !effects.events.is_empty() {
        for (event_handle, event_sequence_number, _event_type, _event_layout, event_data, _) in
            &effects.events
        {
            explained_effects.events.push(format!(
                "Emitted {:?} as the {}th event to stream {:?}",
                event_data, event_sequence_number, event_handle
            ));
        }
    }
    for (addr, writes) in &effects.resources {
        let address = format!("0x{}", addr);
        let mut changes = vec![];
        for (struct_tag, write_opt) in writes {
            let change = match write_opt {
                Some((_layout, value)) => {
                    if state
                        .get_resource_bytes(*addr, struct_tag.clone())
                        .is_some()
                    {
                        format!("Changed type {}: {}", struct_tag, value)
                    } else {
                        format!("Added type {}: {}", struct_tag, value)
                    }
                }
                None => format!("Deleted type {}", struct_tag),
            };
            changes.push(change);
        }
        let change = AddressResourceChanges::new(address, changes);
        explained_effects.resources.push(change);
    }
    Ok(explained_effects)
}

/// Explain an execution error
pub fn explain_error(error: VMError, remote_cache: &FakeRemoteCache) -> anyhow::Error {
    let mut text_representation = String::new();
    match error.into_vm_status() {
        VMStatus::MoveAbort(AbortLocation::Module(id), abort_code) => {
            // try to use move-explain to explain the abort
            // TODO: this will only work for errors in the stdlib or Libra Framework. We should
            // add code to build an ErrorMapping for modules in move_lib as well
            let error_descriptions: ErrorMapping =
                lcs::from_bytes(compiled_stdlib::ERROR_DESCRIPTIONS).unwrap();
            writeln!(
                &mut text_representation,
                "Execution aborted with code {} in module {}.",
                abort_code, id
            )
            .unwrap();

            if let Some(error_desc) = error_descriptions.get_explanation(&id, abort_code) {
                writeln!(
                    &mut text_representation,
                    " Abort code details:\nReason:\n  Name: {}\n  Description:{}\nCategory:\n  Name: {}\n  Description:{}",
                    error_desc.reason.code_name,
                    error_desc.reason.code_description,
                    error_desc.category.code_name,
                    error_desc.category.code_description,
                ).unwrap();
            } else {
                writeln!(&mut text_representation).unwrap();
            }
        }
        VMStatus::MoveAbort(AbortLocation::Script, abort_code) => {
            // TODO: map to source code location
            writeln!(
                &mut text_representation,
                "Execution aborted with code {} in transaction script",
                abort_code
            )
            .unwrap();
        }
        VMStatus::ExecutionFailure {
            status_code,
            location,
            function,
            code_offset,
        } => {
            let status_explanation = match status_code {
                    StatusCode::RESOURCE_ALREADY_EXISTS => "resource already exists (i.e., move_to<T>(account) when there is already a value of type T under account)".to_string(),
                    StatusCode::MISSING_DATA => "resource does not exist (i.e., move_from<T>(a), borrow_global<T>(a), or borrow_global_mut<T>(a) when there is no value of type T at address a)".to_string(),
                    StatusCode::ARITHMETIC_ERROR => "arithmetic error (i.e., integer overflow, underflow, or divide-by-zero)".to_string(),
                    StatusCode::EXECUTION_STACK_OVERFLOW => "execution stack overflow".to_string(),
                    StatusCode::CALL_STACK_OVERFLOW => "call stack overflow".to_string(),
                    StatusCode::OUT_OF_GAS => "out of gas".to_string(),
                    _ => format!("{} error", status_code.status_type()),
                };
            // TODO: map to source code location
            let location_explanation = match location {
                AbortLocation::Module(id) => format!(
                    "{}::{}",
                    id,
                    remote_cache.resolve_function(&id, function).unwrap()
                ),
                AbortLocation::Script => "script".to_string(),
            };
            // TODO: code offset is 1-indexed, but disassembler instruction numbering starts at zero
            // This is potentially confusing to someone trying to understnd where something failed
            // by looking at a code offset + disassembled bytecode; we should fix it
            writeln!(
                &mut text_representation,
                "Execution failed with {} in {} at code offset {}",
                status_explanation, location_explanation, code_offset
            )
            .unwrap();
        }
        // VMStatus::Error(StatusCode::NUMBER_OF_TYPE_ARGUMENTS_MISMATCH) => writeln!(
        //     "Execution failed with incorrect number of type arguments: script expected {:?}, but \
        //      found {:?}",
        //     &script.as_inner().type_parameters.len(),
        //     vm_type_args.len()
        // ),
        VMStatus::Error(status_code) => writeln!(
            &mut text_representation,
            "Execution failed with unexpected error {:?}",
            status_code
        )
        .unwrap(),
        VMStatus::Executed => unreachable!(),
    };
    anyhow::anyhow!(text_representation)
}
