use language_e2e_tests::data_store::FakeDataStore;

use libra_types::account_address::AccountAddress;

use libra_types::write_set::WriteSet;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};

use move_lang::compiled_unit::{verify_units, CompiledUnit};
use move_lang::errors::Errors;
use move_lang::shared::Address;
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, SystemExecutionContext};
use move_vm_types::values::Value;

use vm::errors::VMResult;
use vm::file_format::CompiledScript;
use vm::gas_schedule::zero_cost_schedule;
use vm::transaction_metadata::TransactionMetadata;
use vm::CompiledModule;

use analysis::compiler;
use analysis::compiler::parse_file;
use analysis::db::FilePath;

use crate::serialization::{get_resource_structs, serialize_write_set, ResourceChange};

pub(crate) fn serialize_script(script: CompiledScript) -> Vec<u8> {
    let mut serialized = vec![];
    script.serialize(&mut serialized).unwrap();
    serialized
}

pub(crate) fn compile_script(
    fname: FilePath,
    text: &str,
    deps: Vec<(FilePath, String)>,
    sender: Address,
) -> Result<(CompiledScript, Vec<CompiledModule>), Errors> {
    let parsed_file = compiler::parse_file(fname, text).map_err(|err| vec![err])?;

    let mut parsed_files = vec![parsed_file];
    for (fpath, text) in deps {
        let parsed = parse_file(fpath, &text).map_err(|e| vec![e])?;
        parsed_files.push(parsed);
    }
    let program = move_lang::parser::ast::Program {
        source_definitions: parsed_files,
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

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

pub(crate) fn execute_script(
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
    )
    .map(|_| exec_context.make_write_set().unwrap())
}

pub(crate) fn compile_and_run(
    script: (FilePath, String),
    deps: Vec<(FilePath, String)>,
    sender: AccountAddress,
) -> VMResult<Vec<ResourceChange>> {
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        compile_script(fname, &script_text, deps, Address::new(sender.into())).unwrap();

    let mut network_state = FakeDataStore::default();
    for module in compiled_modules {
        network_state.add_module(&module.self_id(), &module);
    }
    let available_resource_structs = get_resource_structs(&compiled_script);

    let serialized_script = serialize_script(compiled_script);
    let write_set = execute_script(sender, &network_state, serialized_script, vec![])?;

    let changes = serialize_write_set(&write_set, available_resource_structs);
    Ok(changes)
}

#[cfg(test)]
mod tests {

    use analysis::utils::tests::{existing_file_abspath, get_stdlib_path};

    use crate::load_module_files;

    use super::*;
    use libra_types::language_storage::StructTag;
    use move_core_types::identifier::Identifier;

    #[test]
    fn test_run_with_empty_script() {
        let text = "fun main() {}";
        let vm_res = compile_and_run(
            (existing_file_abspath(), text.to_string()),
            vec![],
            AccountAddress::default(),
        );
        assert!(vm_res.is_ok());
    }

    #[test]
    fn test_execute_custom_script_with_stdlib_module() {
        let sender = AccountAddress::new([1; 24]);
        let text = r"
    use 0x0::Transaction;

    fun main() {
        let _ = Transaction::sender();
    }";
        let deps = load_module_files(vec![get_stdlib_path()]);
        let vm_res = compile_and_run((existing_file_abspath(), text.to_string()), deps, sender);
        assert!(vm_res.is_ok());
    }

    #[test]
    fn test_execute_script_with_resource_request() {
        let sender = AccountAddress::from_hex_literal("0x111111111111111111111111").unwrap();
        let module_name = existing_file_abspath();
        let module_text = r"
    address 0x111111111111111111111111:

    module Record {
        resource struct T {
            age: u8,
            age2: u8
        }

        public fun create(age: u8): T {
            T { age: age + 1, age2: age + 2 }
        }

        public fun save(record: T) {
            move_to_sender<T>(record);
        }
    }
        ";
        let script_text = r"
    use 0x111111111111111111111111::Record;

    fun main() {
        let record = Record::create(10);
        Record::save(record);
    }";
        let mut deps = load_module_files(vec![get_stdlib_path()]);
        deps.push((module_name, module_text.to_string()));

        let vm_res = compile_and_run(
            (existing_file_abspath(), script_text.to_string()),
            deps,
            sender,
        );
        let changes = vm_res.unwrap();

        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].struct_tag,
            StructTag {
                address: sender,
                module: Identifier::new("Record").unwrap(),
                name: Identifier::new("T").unwrap(),
                type_params: vec![]
            }
        );
        assert_eq!(changes[0].values, vec![11, 12]);
    }
}
