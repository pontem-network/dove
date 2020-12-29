use anyhow::Result;
use std::fmt::Write;

use libra::move_vm_runtime::data_cache::TransactionEffects;
use crate::execution::FakeRemoteCache;
use libra::libra_types::vm_status::{VMStatus, AbortLocation, StatusCode};
use libra::vm::file_format::CompiledScript;
use libra::move_core_types::account_address::AccountAddress;
use libra::move_core_types::transaction_argument::TransactionArgument;
use libra::vm::access::ScriptAccess;
use libra::move_core_types::language_storage::{StructTag, TypeTag};
use libra::move_vm_types::values::{ValueImpl, Container};
use num_format::ToFormattedString;
use crate::session::ConstsMap;
use libra::move_lang::shared::Address;

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
    write_set_size: usize,
}

impl ExplainedTransactionEffects {
    pub fn events(&self) -> &Vec<ResourceChange> {
        &self.events
    }

    pub fn resources(&self) -> &Vec<AddressResourceChanges> {
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
    let mut trimmed = addr.short_str().to_string();
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

fn display_list_of_values<T, I, F>(items: I, format_value: F) -> Result<String>
where
    F: Fn(&T) -> Result<String>,
    I: IntoIterator<Item = T>,
{
    let mut out = String::new();
    write!(out, "[")?;
    let mut items = items.into_iter();
    if let Some(x) = items.next() {
        write!(out, "{}", format_value(&x)?)?;
        for x in items {
            write!(out, ", {}", format_value(&x)?)?;
        }
    }
    write!(out, "]")?;
    Ok(out)
}

fn format_container(
    container: Container,
    num_custom_format: num_format::CustomFormat,
) -> Result<String> {
    match container {
        Container::Locals(r)
        | Container::VecC(r)
        | Container::VecR(r)
        | Container::StructC(r)
        | Container::StructR(r) => display_list_of_values(r.borrow().iter(), format_value),
        Container::VecU8(r) => display_list_of_values(r.borrow().iter(), |num| {
            Ok(num.to_formatted_string(&num_custom_format))
        }),
        Container::VecU64(r) => display_list_of_values(r.borrow().iter(), |num| {
            Ok(num.to_formatted_string(&num_custom_format))
        }),
        Container::VecU128(r) => display_list_of_values(r.borrow().iter(), |num| {
            Ok(num.to_formatted_string(&num_custom_format))
        }),
        Container::VecBool(r) => {
            display_list_of_values(r.borrow().iter(), |b| Ok(format!("{}", b)))
        }
        Container::VecAddress(r) => {
            display_list_of_values(r.borrow().iter(), |b| Ok(short_address(b)))
        }
    }
}

fn format_value(value: &&ValueImpl) -> Result<String> {
    let format = num_format::CustomFormat::builder().separator("").build()?;
    let mut out = String::new();
    match value {
        ValueImpl::Invalid => write!(out, "Invalid"),

        ValueImpl::U8(num) => write!(out, "U8({})", num.to_formatted_string(&format)),
        ValueImpl::U64(num) => write!(out, "U64({})", num.to_formatted_string(&format)),
        ValueImpl::U128(num) => write!(out, "U128({})", num.to_formatted_string(&format)),
        ValueImpl::Bool(b) => write!(out, "{}", b),
        ValueImpl::Address(addr) => write!(out, "Address({})", short_address(addr)),

        ValueImpl::Container(r) => write!(out, "{}", format_container(r.clone(), format)?),

        ValueImpl::ContainerRef(r) => write!(out, "{}", r),
        ValueImpl::IndexedRef(r) => write!(out, "{}", r),
    }?;
    Ok(out)
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
            explained_effects.events.push(ResourceChange(
                formatted_ty,
                Some(format_value(&&event_data.0)?),
            ));
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
                            ResourceChange(formatted_struct_tag, Some(format_value(&&value.0)?)),
                        )
                    } else {
                        (
                            "Added".to_string(),
                            ResourceChange(formatted_struct_tag, Some(format_value(&&value.0)?)),
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

pub fn explain_type_error(
    script: &CompiledScript,
    signers: &[AccountAddress],
    txn_args: &[TransactionArgument],
) -> String {
    use libra::vm::file_format::SignatureToken::*;

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
            // TODO: code offset is 1-indexed, but disassembler instruction numbering starts at zero
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
