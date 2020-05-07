use anyhow::Result;

use crate::resources::ResourceChange;
use crate::FilePath;
pub use language_e2e_tests::data_store::FakeDataStore;
pub use libra_crypto::hash::CryptoHash;
pub use libra_types::access_path::AccessPath;
pub use libra_types::account_address::AccountAddress;
pub use libra_types::language_storage::StructTag;
pub use libra_types::vm_error::{StatusCode, VMStatus};
pub use libra_types::write_set::{WriteOp, WriteSet};
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_core_types::identifier::Identifier;
pub use move_ir_types::location::Loc;
pub use move_lang::compiled_unit::{verify_units, CompiledUnit};
pub use move_lang::errors::{report_errors, FilesSourceText};
pub use move_lang::errors::{Error, Errors};
use move_lang::parser;
pub use move_lang::parser::ast::Definition;
pub use move_lang::parser::syntax;
pub use move_lang::shared::Address;
pub use move_lang::strip_comments_and_verify;
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, SystemExecutionContext};
pub use move_vm_types::gas_schedule::zero_cost_schedule;
pub use move_vm_types::values::Value;
use std::collections::HashMap;
pub use vm::access::ScriptAccess;
pub use vm::errors::VMResult;
pub use vm::file_format::CompiledScript;
pub use vm::transaction_metadata::TransactionMetadata;
pub use vm::CompiledModule;

pub fn parse_account_address(s: &str) -> Result<AccountAddress> {
    AccountAddress::from_hex_literal(s)
}

pub fn parse_file(fname: FilePath, text: &str) -> Result<Vec<Definition>, Errors> {
    let (no_comments_source, comment_map) = strip_comments_and_verify(fname, text)?;
    let res = syntax::parse_file_string(fname, &no_comments_source, comment_map)?;
    Ok(res.0)
}

pub fn check_parsed_program(
    current_file_defs: Vec<Definition>,
    dependencies: Vec<Definition>,
    sender_opt: Address,
) -> Result<(), Errors> {
    let ast_program = parser::ast::Program {
        source_definitions: current_file_defs,
        lib_definitions: dependencies,
    };
    move_lang::check_program(Ok(ast_program), Some(sender_opt))?;
    Ok(())
}

pub fn compile_script(
    fname: FilePath,
    text: &str,
    deps: &[(FilePath, String)],
    sender: Address,
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
    let mut compiled_modules = vec![];
    let mut compiled_script = None;
    let compiled_units = move_lang::compile_program(Ok(program), Some(sender))?;
    let (compiled_units, errors) = verify_units(compiled_units);
    if !errors.is_empty() {
        return Err(errors);
    }

    for unit in compiled_units {
        match unit {
            CompiledUnit::Module { module, .. } => compiled_modules.push(module),
            CompiledUnit::Script { script, .. } => compiled_script = Some(script),
        }
    }
    Ok((compiled_script.unwrap(), compiled_modules))
}

pub fn serialize_script(script: CompiledScript) -> Vec<u8> {
    let mut serialized = vec![];
    script.serialize(&mut serialized).unwrap();
    serialized
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

fn get_struct_tag_at(
    access_path: AccessPath,
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Option<StructTag> {
    // resource tag
    let path = access_path.path;
    let struct_sha3 = &path[1..];
    if path[0] == 1 {
        if let Some(struct_tag) = resource_structs.get(struct_sha3) {
            return Some(struct_tag.clone());
        }
    }
    None
}

#[allow(clippy::implicit_hasher)]
pub fn serialize_write_set(
    write_set: WriteSet,
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Vec<ResourceChange> {
    let mut changed = vec![];
    for (access_path, write_op) in write_set {
        let struct_tag = get_struct_tag_at(access_path, resource_structs);
        if let Some(struct_tag) = struct_tag {
            let change = ResourceChange::new(struct_tag, write_op);
            changed.push(change);
        }
    }
    changed
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
