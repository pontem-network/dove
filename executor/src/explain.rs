use std::fmt::Write;

use anyhow::Result;
use diem_types::contract_event::ContractEvent;
use diem_types::event::EventKey;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::transaction_argument::TransactionArgument;
use move_core_types::vm_status::{AbortLocation, StatusCode, VMStatus};
use move_core_types::effects::Event as MoveCoreEvent;
use move_lang::shared::Address;
use vm::access::ScriptAccess;
use vm::file_format::CompiledScript;

use crate::execution::{FakeRemoteCache, TransactionEffects};
use crate::session::ConstsMap;

pub type StepResultInfo = (String, u64, usize, StepExecutionResult);

#[derive(Debug)]
pub struct PipelineExecutionResult {
    pub step_results: Vec<StepResultInfo>,
}

impl PipelineExecutionResult {
    pub fn new(step_results: Vec<StepResultInfo>) -> Self {
        PipelineExecutionResult { step_results }
    }

    pub fn last(&self) -> Option<StepExecutionResult> {
        self.step_results.last().map(|(_, _, _, r)| r.to_owned())
    }

    pub fn overall_gas_spent(&self) -> u64 {
        self.step_results.iter().map(|(_, gas, _, _)| gas).sum()
    }
}

#[derive(Debug, Clone)]
pub enum StepExecutionResult {
    Error(String),
    ExpectedError(String),
    Success(ExplainedTransactionEffects),
}

impl StepExecutionResult {
    pub fn with_expected_error(s: String) -> StepExecutionResult {
        StepExecutionResult::ExpectedError(format!("Expected error: {}", s))
    }

    pub fn with_error(s: String) -> StepExecutionResult {
        StepExecutionResult::Error(s)
    }

    pub fn error(self) -> String {
        match self {
            StepExecutionResult::Error(error) => error,
            _ => panic!(),
        }
    }

    pub fn expected_error(self) -> String {
        match self {
            StepExecutionResult::ExpectedError(error) => error,
            _ => panic!(),
        }
    }

    pub fn effects(self) -> ExplainedTransactionEffects {
        match self {
            StepExecutionResult::Success(effects) => effects,
            StepExecutionResult::Error(msg) | StepExecutionResult::ExpectedError(msg) => {
                panic!("{}", msg)
            }
        }
    }
}

type Resource = String;
type ResourceType = String;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub enum ResourceChange {
    /// Resource was added.
    Added(Resource),
    /// Resource was modified.
    Changed(Resource),
    /// Resource was removed.
    Deleted(ResourceType),
}

impl std::fmt::Display for ResourceChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Added(resource) => write!(f, "Added: {}", resource),
            Self::Changed(resource) => write!(f, "Changed: {}", resource),
            Self::Deleted(resource_type) => write!(f, "Deleted: {}", resource_type),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, Eq, PartialEq)]
pub struct AddressResourceChanges {
    pub address: String,
    pub changes: Vec<ResourceChange>,
}

impl AddressResourceChanges {
    pub fn new<S: ToString>(address: S, changes: Vec<ResourceChange>) -> Self {
        AddressResourceChanges {
            address: address.to_string(),
            changes,
        }
    }
}

pub type Event = String;

#[derive(Debug, Default, Clone, serde::Serialize, Eq, PartialEq)]
pub struct ExplainedTransactionEffects {
    events: Vec<Event>,
    resources: Vec<AddressResourceChanges>,
    write_set_size: usize,
}

impl ExplainedTransactionEffects {
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn resources(&self) -> &[AddressResourceChanges] {
        &self.resources
    }

    pub fn write_set_size(&self) -> usize {
        self.write_set_size
    }

    pub fn set_write_set_size(&mut self, size: usize) {
        self.write_set_size = size;
    }
}

fn short_address(addr: &AccountAddress) -> String {
    Address::new(addr.to_u8()).to_string()
}

fn format_struct_tag(s: &StructTag) -> Result<String> {
    let mut f = String::new();
    write!(f, "{}::{}::{}", short_address(&s.address), s.module, s.name)?;
    if let Some(first_ty) = s.type_params.first() {
        write!(f, "<")?;
        write!(f, "{}", format_type_tag(first_ty)?)?;
        for ty in s.type_params.iter().skip(1) {
            write!(f, ", {}", format_type_tag(ty)?)?;
        }
        write!(f, ">")?;
    }
    Ok(f)
}

fn format_type_tag(type_tag: &TypeTag) -> Result<String> {
    let mut f = String::new();
    match type_tag {
        TypeTag::Struct(s) => write!(f, "{}", format_struct_tag(s)?),
        TypeTag::Vector(ty) => write!(f, "Vector<{}>", format_type_tag(ty)?),
        TypeTag::U8 => write!(f, "U8"),
        TypeTag::U64 => write!(f, "U64"),
        TypeTag::U128 => write!(f, "U128"),
        TypeTag::Address => write!(f, "Address"),
        TypeTag::Signer => write!(f, "Signer"),
        TypeTag::Bool => write!(f, "Bool"),
    }?;
    Ok(f)
}

fn format_struct(
    state: &FakeRemoteCache,
    struct_tag: &StructTag,
    value: &[u8],
) -> Result<String> {
    let annotator = resource_viewer::MoveValueAnnotator::new_no_stdlib(state);
    let annotated_struct = annotator.view_resource(struct_tag, value)?;
    let mut result = String::new();
    writeln!(result, "{}", annotated_struct)?;
    Ok(result)
}

/// It's probably a mistake or architectural flow that Event != ContractEvent.
/// It might be fixed in the future. For now, we just need to make a conversion and prey if it is
/// valid (because of tuple-typing, we can't be sure if these vectors are actually valid).
fn contract_event(event: &MoveCoreEvent) -> ContractEvent {
    // to build EventKey, we need not only sender address, but also an id of event stream
    // it's easier to fake it, as MoveValueAnnotator doesn't use this field
    let event_key = EventKey::new([0; EventKey::LENGTH]);

    ContractEvent::new(event_key, event.1, event.2.clone(), event.3.clone())
}

fn format_event(state: &FakeRemoteCache, event: &MoveCoreEvent) -> Result<String> {
    let annotator = resource_viewer::MoveValueAnnotator::new_no_stdlib(state);
    let annotated_event = annotator.view_contract_event(&contract_event(event))?;
    let mut result = String::new();
    writeln!(result, "{}", annotated_event)?;
    Ok(result)
}

pub fn explain_effects(
    effects: &TransactionEffects,
    state: &FakeRemoteCache,
) -> Result<ExplainedTransactionEffects> {
    let mut explained_effects = ExplainedTransactionEffects::default();

    for event in &effects.1 {
        explained_effects.events.push(format_event(state, event)?);
    }

    for (addr, writes) in &effects.0.accounts {
        let mut changes = Vec::with_capacity(writes.resources.len());

        for (struct_tag, written_data) in &writes.resources {
            let resource_change = match written_data {
                Some(value) => {
                    let resource = format_struct(state, struct_tag, &value)?;
                    match state.get_resource_bytes(*addr, struct_tag.clone()) {
                        Some(_) => ResourceChange::Changed(resource),
                        None => ResourceChange::Added(resource),
                    }
                }
                None => ResourceChange::Deleted(format_struct_tag(&struct_tag)?),
            };
            changes.push(resource_change);
        }
        let trimmed_address = Address::new(addr.to_u8()).to_string();
        let change = AddressResourceChanges::new(trimmed_address, changes);
        explained_effects.resources.push(change);
    }

    Ok(explained_effects)
}

pub fn explain_type_error(
    script: &CompiledScript,
    signers: &[AccountAddress],
    txn_args: &[TransactionArgument],
) -> String {
    use vm::file_format::SignatureToken::*;

    let script_params = script.signature_at(script.as_inner().parameters);
    let expected_num_signers = script_params
        .0
        .iter()
        .filter(|t| match t {
            Reference(r) => matches!(**r, Signer),
            _ => false,
        })
        .count();
    if expected_num_signers != signers.len() {
        return format!(
            "Execution failed with incorrect number of signers: script expected {:?}, but found \
             {:?}",
            expected_num_signers,
            signers.len()
        );
    }

    // TODO: printing type(s) of missing arguments could be useful
    let expected_num_args = script_params.len() - signers.len();
    if expected_num_args != txn_args.len() {
        return format!(
            "Execution failed with incorrect number of arguments: script expected {:?}, but found \
             {:?}",
            expected_num_args,
            txn_args.len()
        );
    }

    // TODO: print more helpful error message pinpointing the (argument, type)
    // pair that didn't match
    "Execution failed with type error when binding type arguments to type parameters".to_string()
}

pub fn explain_abort(vm_status: VMStatus, consts_map: &ConstsMap) -> String {
    match vm_status {
        VMStatus::MoveAbort(AbortLocation::Module(id), error_code) => {
            let const_key = (
                format!("{}", Address::new(id.address().to_u8())),
                id.name().to_string(),
                error_code as u128,
            );
            let const_name = consts_map.get(&const_key);
            let error = match const_name {
                Some(name) => format!("{}: {}", error_code, name),
                None => format!("{}", error_code),
            };
            return format!(
                "Execution aborted with code {} in module {}::{}.",
                error,
                short_address(id.address()),
                id.name()
            );
        }
        VMStatus::MoveAbort(AbortLocation::Script, error_code) => {
            // TODO: map to source code location
            return format!(
                "Execution aborted with code {} in transaction script",
                error_code
            );
        }
        _ => unreachable!(),
    }
}

pub fn explain_execution_failure(vm_status: VMStatus, remote_cache: &FakeRemoteCache) -> String {
    match vm_status {
        VMStatus::ExecutionFailure {
            status_code,
            location,
            function,
            code_offset,
        } => {
            let status_explanation = match status_code {
                StatusCode::RESOURCE_ALREADY_EXISTS => "a RESOURCE_ALREADY_EXISTS error (i.e., `move_to<T>(account)` when there is already a resource of type `T` under `account`)".to_string(),
                StatusCode::MISSING_DATA => "a RESOURCE_DOES_NOT_EXIST error (i.e., `move_from<T>(a)`, `borrow_global<T>(a)`, or `borrow_global_mut<T>(a)` when there is no resource of type `T` at address `a`)".to_string(),
                StatusCode::ARITHMETIC_ERROR => "an arithmetic error (i.e., integer overflow/underflow, div/mod by zero, or invalid shift)".to_string(),
                StatusCode::EXECUTION_STACK_OVERFLOW => "an execution stack overflow".to_string(),
                StatusCode::CALL_STACK_OVERFLOW => "a call stack overflow".to_string(),
                StatusCode::OUT_OF_GAS => "an out of gas error".to_string(),
                _ => format!("a {} error", status_code.status_type()),
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
            // TODO: code offset is 1-indexed, but decompiler instruction numbering starts at zero
            // This is potentially confusing to someone trying to understand where something failed
            // by looking at a code offset + disassembled bytecode; we should fix it
            return format!(
                "Execution failed with {} in {} at code offset {}",
                status_explanation, location_explanation, code_offset
            );
        }
        _ => unreachable!(),
    }
}
