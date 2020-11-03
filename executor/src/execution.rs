use std::collections::HashMap;

use anyhow::{Context, Result};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::{RemoteCache, TransactionEffects};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;
use vm::access::ModuleAccess;
use vm::CompiledModule;
use vm::errors::{Location, PartialVMError, PartialVMResult, VMResult};
use vm::file_format::{CompiledScript, FunctionDefinitionIndex};

use crate::explain::{explain_effects, explain_error, StepExecutionResult};
use crate::meta::ExecutionMeta;
use crate::oracles::{oracle_coins_module, time_metadata};
use move_vm_runtime::logging::NoContextLog;
use crate::session::ConstsMap;

#[derive(Debug, Default, Clone)]
pub struct FakeRemoteCache {
    modules: HashMap<ModuleId, Vec<u8>>,
    resources: HashMap<(AccountAddress, StructTag), Vec<u8>>,
}

impl FakeRemoteCache {
    pub fn new(compiled_modules: Vec<CompiledModule>) -> Result<Self> {
        let mut modules = HashMap::with_capacity(compiled_modules.len());
        for module in compiled_modules {
            let mut module_bytes = vec![];
            module
                .serialize(&mut module_bytes)
                .context("Module serialization error")?;
            modules.insert(module.self_id(), module_bytes);
        }
        let resources = HashMap::new();
        Ok(FakeRemoteCache { modules, resources })
    }

    /// Read the resource bytes stored on-disk at `addr`/`tag`
    pub fn get_resource_bytes(&self, addr: AccountAddress, tag: StructTag) -> Option<Vec<u8>> {
        self.resources.get(&(addr, tag)).map(|r| r.to_owned())
    }

    /// Read the resource bytes stored on-disk at `addr`/`tag`
    fn get_module_bytes(&self, module_id: &ModuleId) -> Option<Vec<u8>> {
        self.modules.get(module_id).map(|r| r.to_owned())
    }

    /// Deserialize and return the module stored on-disk at `addr`/`module_id`
    pub fn get_compiled_module(&self, module_id: &ModuleId) -> Result<CompiledModule> {
        CompiledModule::deserialize(&self.get_module_bytes(module_id).unwrap())
            .map_err(|e| anyhow::anyhow!("Failure deserializing module {:?}: {:?}", module_id, e))
    }

    pub fn resolve_function(&self, module_id: &ModuleId, idx: u16) -> Result<Identifier> {
        let m = self.get_compiled_module(module_id).unwrap();
        Ok(m.identifier_at(
            m.function_handle_at(m.function_def_at(FunctionDefinitionIndex(idx)).function)
                .name,
        )
        .to_owned())
    }

    pub fn merge_transaction_effects(&mut self, effects: TransactionEffects) -> usize {
        let mut resources_write_size = 0;
        for (addr, changes) in effects.resources {
            for (struct_tag, val) in changes {
                match val {
                    Some((layout, val)) => {
                        let serialized = val.simple_serialize(&layout).expect("Valid value.");
                        resources_write_size += serialized.len();
                        self.resources.insert((addr, struct_tag), serialized);
                    }
                    None => {
                        self.resources.remove(&(addr, struct_tag));
                    }
                }
            }
        }
        resources_write_size
    }
}

impl RemoteCache for FakeRemoteCache {
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        match self.modules.get(module_id) {
            None => {
                match self.get_module_bytes(module_id) {
                    Some(bytes) => Ok(Some(bytes)),
                    None => Err(PartialVMError::new(StatusCode::STORAGE_ERROR)
                        .finish(Location::Undefined)),
                }
            }
            m => Ok(m.cloned()),
        }
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        let res = match self.resources.get(&(*address, struct_tag.clone())) {
            None => self.get_resource_bytes(*address, struct_tag.clone()),
            res => res.cloned(),
        };
        Ok(res)
    }
}

pub fn serialize_script(script: &CompiledScript) -> Result<Vec<u8>> {
    let mut serialized = vec![];
    script
        .serialize(&mut serialized)
        .context("Script serialization error")?;
    Ok(serialized)
}

fn execute_script_with_runtime_session<R: RemoteCache>(
    data_store: &R,
    script: Vec<u8>,
    args: Vec<Value>,
    ty_args: Vec<TypeTag>,
    senders: Vec<AccountAddress>,
    cost_strategy: &mut CostStrategy,
) -> VMResult<TransactionEffects> {
    let vm = MoveVM::new();
    let mut runtime_session = vm.new_session(data_store);

    runtime_session.execute_script(
        script,
        ty_args,
        args,
        senders,
        cost_strategy,
        &NoContextLog::new(),
    )?;
    runtime_session.finish()
}

pub fn execute_script(
    meta: ExecutionMeta,
    data_store: &mut FakeRemoteCache,
    script: CompiledScript,
    args: Vec<Value>,
    cost_strategy: &mut CostStrategy,
    consts_map: &ConstsMap,
) -> Result<StepExecutionResult> {
    let mut ds = data_store.clone();
    let ExecutionMeta {
        signers,
        oracle_prices,
        current_time,
        aborts_with,
        ..
    } = meta;
    if !oracle_prices.is_empty() {
        // check if module exists, and fail with MISSING_DEPENDENCY if not
        if ds.get_module(&oracle_coins_module()).is_err() {
            return Ok(StepExecutionResult::Error(
                "Cannot use `price:` comments: missing `0x1::Coins` module".to_string(),
            ));
        }
    }
    let std_addr = AccountAddress::from_hex_literal("0x1").expect("Standart address");

    if let Some(current_time) = current_time {
        ds.resources.insert(
            (std_addr, time_metadata()),
            lcs::to_bytes(&current_time).unwrap(),
        );
    }
    for (price_tag, val) in oracle_prices {
        ds.resources
            .insert((std_addr, price_tag), lcs::to_bytes(&val).unwrap());
    }

    let res = execute_script_with_runtime_session(
        &ds,
        serialize_script(&script)?,
        args,
        vec![],
        signers.clone(),
        cost_strategy,
    );
    Ok(match res {
        Ok(effects) => {
            let mut explained = explain_effects(&effects, &ds)?;
            let write_set_size = data_store.merge_transaction_effects(effects);
            explained.set_write_set_size(write_set_size);
            StepExecutionResult::Success(explained)
        }
        Err(vm_error) => {
            let (abort_code, error_as_string) =
                explain_error(vm_error, data_store, &script, &signers, consts_map);
            match aborts_with {
                Some(expected_abort_code) => {
                    if abort_code.is_some() && abort_code.unwrap() == expected_abort_code {
                        StepExecutionResult::ExpectedError(format!(
                            "Expected error: {}",
                            error_as_string
                        ))
                    } else {
                        StepExecutionResult::Error(error_as_string)
                    }
                }
                None => StepExecutionResult::Error(error_as_string),
            }
        }
    })
}
