use anyhow::Result;
use std::fmt::Write;

use vm::errors::VMError;

use move_vm_runtime::data_cache::TransactionEffects;
use crate::execution::FakeRemoteCache;
use libra_types::vm_status::{VMStatus, AbortLocation, StatusCode};
use vm::file_format::CompiledScript;
use move_core_types::account_address::AccountAddress;
use move_core_types::transaction_argument::TransactionArgument;
use vm::access::ScriptAccess;
use move_core_types::language_storage::{StructTag, TypeTag};

#[derive(Debug)]
pub struct PipelineExecutionResult {
    pub step_results: Vec<(String, u64, StepExecutionResult)>,
}

impl PipelineExecutionResult {
    pub fn new(step_results: Vec<(String, u64, StepExecutionResult)>) -> Self {
        PipelineExecutionResult { step_results }
    }

    pub fn last(&self) -> Option<StepExecutionResult> {
        self.step_results.last().map(|(_, _, r)| r.to_owned())
    }

    pub fn overall_gas_spent(&self) -> u64 {
        self.step_results.iter().map(|(_, gas, _)| gas).sum()
    }
}

#[derive(Debug, Clone)]
pub enum StepExecutionResult {
    Error(String),
    Success(ExplainedTransactionEffects),
}

impl StepExecutionResult {
    pub fn error(self) -> String {
        match self {
            StepExecutionResult::Error(error) => error,
            _ => panic!(),
        }
    }

    pub fn effects(self) -> ExplainedTransactionEffects {
        match self {
            StepExecutionResult::Success(effects) => effects,
            StepExecutionResult::Error(msg) => panic!("{}", msg),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, Eq, PartialEq)]
pub struct ResourceChange(pub String, pub Option<String>);

#[derive(Debug, Clone, serde::Serialize, Eq, PartialEq)]
pub struct AddressResourceChanges {
    pub address: String,
    pub changes: Vec<(String, ResourceChange)>,
}

impl AddressResourceChanges {
    pub fn new<S: ToString>(address: S, changes: Vec<(String, ResourceChange)>) -> Self {
        AddressResourceChanges {
            address: address.to_string(),
            changes,
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, Eq, PartialEq)]
pub struct ExplainedTransactionEffects {
    events: Vec<ResourceChange>,
    resources: Vec<AddressResourceChanges>,
}

impl ExplainedTransactionEffects {
    pub fn events(&self) -> &Vec<ResourceChange> {
        &self.events
    }
    pub fn resources(&self) -> &Vec<AddressResourceChanges> {
        &self.resources
    }
}

fn short_address(addr: &AccountAddress) -> String {
    let mut trimmed = addr.short_str();
    if trimmed == "00000000" {
        trimmed = addr.to_string().trim_start_matches('0').to_string();
    }
    format!("0x{}", trimmed)
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

pub fn explain_effects(
    effects: &TransactionEffects,
    state: &FakeRemoteCache,
) -> Result<ExplainedTransactionEffects> {
    // effects shouldn't contain modules
    assert!(effects.modules.is_empty());

    let mut explained_effects = ExplainedTransactionEffects::default();
    if !effects.events.is_empty() {
        for (_, _, ty, _, event_data, _) in &effects.events {
            let formatted_ty = format_type_tag(ty)?;
            explained_effects
                .events
                .push(ResourceChange(formatted_ty, Some(event_data.to_string())));
        }
    }
    for (addr, writes) in &effects.resources {
        let mut changes = vec![];
        for (struct_tag, write_opt) in writes {
            let formatted_struct_tag = format_struct_tag(&struct_tag)?;
            changes.push(match write_opt {
                Some((_, value)) => {
                    if state
                        .get_resource_bytes(*addr, struct_tag.clone())
                        .is_some()
                    {
                        (
                            "Changed".to_string(),
                            ResourceChange(formatted_struct_tag, Some(value.to_string())),
                        )
                    } else {
                        (
                            "Added".to_string(),
                            ResourceChange(formatted_struct_tag, Some(value.to_string())),
                        )
                    }
                }
                None => (
                    "Added".to_string(),
                    ResourceChange(formatted_struct_tag, None),
                ),
            });
        }
        let trimmed_address = format!("0x{}", addr.to_string().trim_start_matches('0'));
        let change = AddressResourceChanges::new(trimmed_address, changes);
        explained_effects.resources.push(change);
    }
    Ok(explained_effects)
}

// pub const ERROR_DESCRIPTIONS: &[u8] = include_bytes!("./error_descriptions/error_descriptions.errmap");

fn explain_type_error(
    text_representation: &mut String,
    script: &CompiledScript,
    signers: &[AccountAddress],
    txn_args: &[TransactionArgument],
) {
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
        writeln!(
            text_representation,
            "Execution failed with incorrect number of signers: script expected {:?}, but found \
             {:?}",
            expected_num_signers,
            signers.len()
        )
        .unwrap();
        return;
    }

    // TODO: printing type(s) of missing arguments could be useful
    let expected_num_args = script_params.len() - signers.len();
    if expected_num_args != txn_args.len() {
        writeln!(
            text_representation,
            "Execution failed with incorrect number of arguments: script expected {:?}, but found \
             {:?}",
            expected_num_args,
            txn_args.len()
        ).unwrap();
        return;
    }

    // TODO: print more helpful error message pinpointing the (argument, type)
    // pair that didn't match
    writeln!(
        text_representation,
        "Execution failed with type error when binding type arguments to type parameters"
    )
    .unwrap();
}

/// Explain an execution error
pub fn explain_error(
    error: VMError,
    remote_cache: &FakeRemoteCache,
    script: &CompiledScript,
    signers: &[AccountAddress],
) -> String {
    let mut text_representation = String::new();
    match error.into_vm_status() {
        VMStatus::MoveAbort(AbortLocation::Module(id), abort_code) => {
            // try to use move-explain to explain the abort
            // TODO: this will only work for errors in the stdlib or Libra Framework. We should
            // add code to build an ErrorMapping for modules in move_lib as well
            // let error_descriptions: ErrorMapping =
            //     lcs::from_bytes(ERROR_DESCRIPTIONS).unwrap();
            write!(
                &mut text_representation,
                "Execution aborted with code {} in module {}.",
                abort_code, id
            )
            .unwrap();
        }
        VMStatus::MoveAbort(AbortLocation::Script, abort_code) => {
            // TODO: map to source code location
            write!(
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
            // TODO: code offset is 1-indexed, but disassembler instruction numbering starts at zero
            // This is potentially confusing to someone trying to understand where something failed
            // by looking at a code offset + disassembled bytecode; we should fix it
            write!(
                &mut text_representation,
                "Execution failed with {} in {} at code offset {}",
                status_explanation, location_explanation, code_offset
            )
            .unwrap();
        }
        VMStatus::Error(StatusCode::TYPE_MISMATCH) => {
            explain_type_error(&mut text_representation, script, signers, &[])
        }
        VMStatus::Error(status_code) => write!(
            &mut text_representation,
            "Execution failed with unexpected error {:?}",
            status_code
        )
        .unwrap(),
        VMStatus::Executed => unreachable!(),
    };
    text_representation
}
