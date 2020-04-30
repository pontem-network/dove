use language_e2e_tests::data_store::FakeDataStore;
use libra_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_lang::compiled_unit::{verify_units, CompiledUnit};
use move_lang::errors::Errors;
use move_lang::shared::Address;
use move_vm_runtime::MoveVM;
use move_vm_state::data_cache::BlockDataCache;
use move_vm_state::execution_context::SystemExecutionContext;
use move_vm_types::values::Value;
use vm::errors::VMResult;
use vm::file_format::CompiledScript;
use vm::gas_schedule::zero_cost_schedule;
use vm::transaction_metadata::TransactionMetadata;
use vm::CompiledModule;

use analysis::compiler;
use analysis::compiler::parse_file;
use analysis::db::FilePath;

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
    data_store: FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
) -> VMResult<()> {
    let cache = BlockDataCache::new(&data_store);

    let mut exec_context = SystemExecutionContext::new(&cache, GasUnits::new(1_000_000));
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
}

pub(crate) fn compile_and_run(
    script: (FilePath, String),
    deps: Vec<(FilePath, String)>,
    sender: AccountAddress,
) -> VMResult<()> {
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        compile_script(fname, &script_text, deps, Address::new(sender.into())).unwrap();

    let mut network_state = FakeDataStore::default();
    for module in compiled_modules {
        network_state.add_module(&module.self_id(), &module);
    }
    let serialized_script = serialize_script(compiled_script);
    execute_script(sender, network_state, serialized_script, vec![])
}

#[cfg(test)]
mod tests {
    use analysis::utils::tests::{existing_file_abspath, get_stdlib_path};

    use crate::load_module_files;

    use super::*;

    #[test]
    fn test_run_with_empty_script() {
        let text = "fun main() {}";
        let vm_res = compile_and_run(
            (existing_file_abspath(), text.to_string()),
            vec![],
            AccountAddress::default(),
        );
        assert!(vm_res.is_ok(), "{:?}", vm_res.unwrap_err());
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
        assert!(vm_res.is_ok(), "{:?}", vm_res.unwrap_err());
    }

    #[test]
    fn test_execute_script_with_resource_request() {
        let sender = AccountAddress::new([1; 24]);
        let module_name = existing_file_abspath();
        let module_text = r"
    address 0x111111111111111111111111:

    module Record {
        resource struct T {
            age: u8
        }

        public fun create(age: u8): T {
            T { age }
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
        assert!(vm_res.is_ok(), "{:?}", vm_res.unwrap_err());
    }
}
