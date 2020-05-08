use crate::FilePath;

use anyhow::Result;
use language_e2e_tests::data_store::FakeDataStore;

use libra_crypto::hash::CryptoHash;
use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use libra_types::language_storage::{ResourceKey, StructTag};
use libra_types::write_set::WriteSet;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_core_types::identifier::Identifier;
use move_lang::compiled_unit::{verify_units, CompiledUnit};
use move_lang::errors::{check_errors, Errors};
use move_lang::parser::ast::Definition;
use move_lang::parser::syntax;
use move_lang::shared::Address;
use move_lang::{parser, strip_comments_and_verify};
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, SystemExecutionContext};
use move_vm_types::gas_schedule::zero_cost_schedule;
use move_vm_types::values::Value;
use std::collections::HashMap;
use vm::access::ScriptAccess;
use vm::errors::VMResult;
use vm::file_format::CompiledScript;
use vm::transaction_metadata::TransactionMetadata;
use vm::CompiledModule;

pub mod types;

pub fn parse_file(fname: FilePath, text: &str) -> Result<Vec<Definition>, Errors> {
    let (no_comments_source, comment_map) = strip_comments_and_verify(fname, text)?;
    let res = syntax::parse_file_string(fname, &no_comments_source, comment_map)?;
    Ok(res.0)
}

pub fn check_parsed_program(
    current_file_defs: Vec<Definition>,
    dependencies: Vec<Definition>,
    sender: [u8; AccountAddress::LENGTH],
) -> Result<(), Errors> {
    let ast_program = parser::ast::Program {
        source_definitions: current_file_defs,
        lib_definitions: dependencies,
    };
    let sender_address = Address::new(sender);
    move_lang::check_program(Ok(ast_program), Some(sender_address)).map(|_| ())
}

pub fn compile_script(
    fname: FilePath,
    text: &str,
    deps: &[(FilePath, String)],
    sender: [u8; AccountAddress::LENGTH],
) -> Result<(CompiledScript, Vec<CompiledModule>), Errors> {
    let mut parsed_defs = parse_file(fname, text)?;
    for (fpath, text) in deps {
        let defs = parse_file(fpath, &text)?;
        parsed_defs.extend(defs);
    }
    let program = move_lang::parser::ast::Program {
        source_definitions: parsed_defs,
        lib_definitions: vec![],
    };

    let sender_address = Address::new(sender);
    let compiled_units = move_lang::compile_program(Ok(program), Some(sender_address))?;
    let (mut units, errors) = verify_units(compiled_units);
    check_errors(errors)?;

    let script = match units.remove(units.len() - 1) {
        CompiledUnit::Script { script, .. } => script,
        CompiledUnit::Module { .. } => unreachable!(),
    };
    let modules = units
        .into_iter()
        .map(|unit| match unit {
            CompiledUnit::Module { module, .. } => module,
            CompiledUnit::Script { .. } => unreachable!(),
        })
        .collect();
    Ok((script, modules))
}

pub fn serialize_script(script: CompiledScript) -> Vec<u8> {
    let mut serialized = vec![];
    script.serialize(&mut serialized).unwrap();
    serialized
}

pub fn prepare_fake_network_state(
    modules: Vec<CompiledModule>,
    genesis_changes_writeset: WriteSet,
) -> FakeDataStore {
    let mut network_state = FakeDataStore::default();
    for module in modules {
        network_state.add_module(&module.self_id(), &module);
    }
    network_state.add_write_set(&genesis_changes_writeset);
    network_state
}

pub fn get_resource_structs(compiled_script: &CompiledScript) -> HashMap<Vec<u8>, StructTag> {
    let mut resource_structs = HashMap::new();
    for struct_handle in compiled_script.struct_handles() {
        if struct_handle.is_nominal_resource {
            let module = compiled_script.module_handle_at(struct_handle.module);
            let module_address = compiled_script.address_identifier_at(module.address);
            let module_name =
                Identifier::new(compiled_script.identifier_at(module.name).as_str()).unwrap();
            let struct_name =
                Identifier::new(compiled_script.identifier_at(struct_handle.name).as_str())
                    .unwrap();
            let struct_tag = StructTag {
                address: *module_address,
                module: module_name,
                name: struct_name,
                type_params: vec![],
            };
            resource_structs.insert(struct_tag.hash().to_vec(), struct_tag);
        }
    }
    resource_structs
}

pub fn struct_tag_into_access_path(struct_tag: StructTag) -> AccessPath {
    let resource_key = ResourceKey::new(struct_tag.address, struct_tag);
    AccessPath::resource_access_path(&resource_key)
}

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
) -> VMResult<WriteSet> {
    let mut exec_context = SystemExecutionContext::new(data_store, GasUnits::new(1_000_000));
    let zero_cost_table = zero_cost_schedule();
    let txn_metadata = get_transaction_metadata(sender_address);

    let vm = MoveVM::new();
    vm.execute_script(
        script,
        &zero_cost_table,
        &mut exec_context,
        &txn_metadata,
        vec![],
        args,
    )?;
    exec_context.make_write_set()
}
