use anyhow::{Context, Result};

use libra_types::{transaction::TransactionArgument};
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{CostTable, GasAlgebra, GasUnits};

use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;

use vm::CompiledModule;

use move_core_types::language_storage::{ModuleId, StructTag, TypeTag};

use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::{TransactionEffects, RemoteCache};

use vm::errors::{Location, PartialVMError, VMResult, PartialVMResult};
use crate::lang::session::{ExecutionMeta, serialize_script};
use crate::lang::explain::{explain_error, explain_effects, ExplainedTransactionEffects};
use vm::file_format::{CompiledScript, FunctionDefinitionIndex};
use std::collections::HashMap;
use move_core_types::identifier::Identifier;
use vm::access::ModuleAccess;

#[derive(Debug, Default)]
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

#[derive(Debug, serde::Serialize)]
pub struct ExecutionResult {
    pub effects: ExplainedTransactionEffects,
    pub gas_spent: u64,
}

pub fn execute_script(
    meta: ExecutionMeta,
    data_store: &FakeRemoteCache,
    script: CompiledScript,
    args: Vec<Value>,
    cost_table: CostTable,
) -> Result<ExecutionResult> {
    let total_gas = 1_000_000;
    let mut cost_strategy = CostStrategy::transaction(&cost_table, GasUnits::new(total_gas));

    let signers = meta.signers;
    let ty_args = vec![];
    let effects = execute_script_with_runtime_session(
        data_store,
        serialize_script(&script)?,
        args,
        ty_args,
        signers,
        &mut cost_strategy,
    )
    .map_err(|error| explain_error(error, data_store))
    .with_context(|| "Script execution error")?;
    let gas_spent = total_gas - cost_strategy.remaining_gas().get();
    let effects = explain_effects(&effects, &data_store)?;

    Ok(ExecutionResult { effects, gas_spent })
}

/// Convert the transaction arguments into move values.
pub fn convert_txn_arg(arg: TransactionArgument) -> Value {
    match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => unimplemented!(),
    }
}
