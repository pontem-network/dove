use std::collections::HashMap;

use anyhow::Result;

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

use crate::explain::{explain_effects, explain_error, ExplainedTransactionEffects};
use crate::session::{ExecutionMeta, serialize_script};
use crate::oracles::oracle_coins_module;

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
            module.serialize(&mut module_bytes)?;
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

    pub fn merge_transaction_effects(&mut self, effects: TransactionEffects) {
        for (addr, changes) in effects.resources {
            for (struct_tag, val) in changes {
                match val {
                    Some((layout, val)) => {
                        let serialized = val.simple_serialize(&layout).expect("Valid value.");
                        self.resources.insert((addr, struct_tag), serialized);
                    }
                    None => {
                        self.resources.remove(&(addr, struct_tag));
                    }
                }
            }
        }
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

    // first signer param -> first passed sender (otherwise reversed)
    let senders = senders.into_iter().rev().collect();

    runtime_session.execute_script(script, ty_args, args, senders, cost_strategy)?;
    runtime_session.finish()
}

pub enum ExecutionResult {
    Success(ExplainedTransactionEffects),
    Error(String),
}

pub fn execute_script(
    meta: ExecutionMeta,
    data_store: &mut FakeRemoteCache,
    script: CompiledScript,
    args: Vec<Value>,
    cost_strategy: &mut CostStrategy,
) -> Result<ExecutionResult> {
    let mut ds = data_store.clone();
    let ExecutionMeta {
        signers,
        oracle_prices,
        ..
    } = meta;
    if !oracle_prices.is_empty() {
        // check if module exists, and fail with MISSING_DEPENDENCY if not
        if ds.get_module(&oracle_coins_module()).is_err() {
            return Ok(ExecutionResult::Error(
                "Cannot use `price:` comments: missing `0x1::Coins` module".to_string(),
            ));
        }
    }
    for (price_tag, val) in oracle_prices {
        let addr = AccountAddress::from_hex_literal("0x1").expect("Standart address");
        ds.resources
            .insert((addr, price_tag), lcs::to_bytes(&val).unwrap());
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
            let explained = explain_effects(&effects, &ds)?;
            let res = ExecutionResult::Success(explained);
            data_store.merge_transaction_effects(effects);
            res
        }
        Err(vm_error) => {
            let error_as_string = explain_error(vm_error, data_store, &script, &signers);
            ExecutionResult::Error(error_as_string)
        }
    })
}
