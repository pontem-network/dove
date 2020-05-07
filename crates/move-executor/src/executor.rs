use language_e2e_tests::data_store::FakeDataStore;
use libra_types::account_address::AccountAddress;
use libra_types::write_set::WriteSet;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_lang::compiled_unit::{verify_units, CompiledUnit};
use move_lang::errors::Errors;
use move_lang::shared::Address;
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, SystemExecutionContext};
use move_vm_types::gas_schedule::zero_cost_schedule;
use move_vm_types::values::Value;
use vm::errors::VMResult;
use vm::file_format::CompiledScript;
use vm::transaction_metadata::TransactionMetadata;
use vm::CompiledModule;

use analysis::compiler;
use analysis::db::FilePath;

use crate::serialization::{
    changes_into_writeset, get_resource_structs, serialize_write_set, ResourceChange,
};

pub(crate) fn serialize_script(script: CompiledScript) -> Vec<u8> {
    let mut serialized = vec![];
    script.serialize(&mut serialized).unwrap();
    serialized
}

pub(crate) fn compile_script(
    fname: FilePath,
    text: &str,
    deps: &[(FilePath, String)],
    sender: Address,
) -> Result<(CompiledScript, Vec<CompiledModule>), Errors> {
    let mut parsed_defs = compiler::parse_file(fname, text)?;
    for (fpath, text) in deps {
        let defs = compiler::parse_file(fpath, &text)?;
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
    )?;
    exec_context.make_write_set()
}

pub(crate) fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: AccountAddress,
    genesis: Option<Vec<ResourceChange>>,
) -> Result<VMResult<Vec<ResourceChange>>, Errors> {
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        compile_script(fname, &script_text, deps, Address::new(sender.into()))?;
    let available_resource_structs = get_resource_structs(&compiled_script);

    let mut network_state = FakeDataStore::default();
    for module in compiled_modules {
        network_state.add_module(&module.self_id(), &module);
    }
    if let Some(changes) = genesis {
        let write_set = changes_into_writeset(changes);
        network_state.add_write_set(&write_set);
    }

    let serialized_script = serialize_script(compiled_script);
    let write_set = match execute_script(sender, &network_state, serialized_script, vec![]) {
        Ok(ws) => ws,
        Err(vm_status) => return Ok(Err(vm_status)),
    };

    let changes = serialize_write_set(write_set, &available_resource_structs);
    Ok(Ok(changes))
}

#[cfg(test)]
mod tests {
    use analysis::utils::io::leaked_fpath;
    use analysis::utils::tests::{existing_file_abspath, get_modules_path, get_stdlib_path};

    use crate::io;

    use super::*;

    fn get_record_module_dep() -> (FilePath, String) {
        let text = r"
address 0x111111111111111111111111 {
    module Record {
        use 0x0::Transaction;

        resource struct T {
            age: u8,
            doubled_age: u8
        }

        public fun create(age: u8): T {
            T { age, doubled_age: age * 2 }
        }

        public fun save(record: T) {
            move_to_sender<T>(record);
        }

        public fun with_incremented_age(): T acquires T {
            let record: T;
            record = move_from<T>(Transaction::sender());
            record.age = record.age + 1;
            record
        }
    }
}
        "
        .to_string();
        let fpath = leaked_fpath(get_modules_path().join("record.move"));
        (fpath, text)
    }

    fn get_sender() -> AccountAddress {
        AccountAddress::from_hex_literal("0x111111111111111111111111").unwrap()
    }

    fn get_script_path() -> FilePath {
        leaked_fpath(get_modules_path().join("script.move"))
    }

    #[test]
    fn test_show_compilation_errors() {
        let sender = AccountAddress::new([1; AccountAddress::LENGTH]);
        let text = r"
script {
    use 0x0::Transaction;

    fun main() {
        let _ = Transaction::sender();
    }
}";
        let errors = compile_and_run((get_script_path(), text.to_string()), &[], sender, None)
            .unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0][0].1, "Unbound module \'0x0::Transaction\'");
    }

    #[test]
    fn test_execute_custom_script_with_stdlib_module() {
        let sender = AccountAddress::new([1; AccountAddress::LENGTH]);
        let text = r"
script {
    use 0x0::Transaction;

    fun main() {
        let _ = Transaction::sender();
    }
}";
        let deps = io::load_module_files(vec![get_stdlib_path()]).unwrap();
        let vm_res = compile_and_run(
            (existing_file_abspath(), text.to_string()),
            &deps,
            sender,
            None,
        )
        .unwrap();
        assert!(vm_res.is_ok());
    }

    #[test]
    fn test_execute_script_and_record_resource_changes() {
        let sender = get_sender();
        let mut deps = io::load_module_files(vec![get_stdlib_path()]).unwrap();
        deps.push(get_record_module_dep());

        let script_text = r"
script {
    use 0x111111111111111111111111::Record;

    fun main() {
        let record = Record::create(10);
        Record::save(record);
    }
}";

        let vm_res = compile_and_run(
            (get_script_path(), script_text.to_string()),
            &deps,
            sender,
            None,
        )
        .unwrap();
        let changes = vm_res.unwrap();
        assert_eq!(changes.len(), 1);

        assert_eq!(
            serde_json::to_value(&changes[0]).unwrap(),
            serde_json::json!({
                "struct_tag": {
                    "address": sender.to_string(),
                    "module": "Record",
                    "name": "T",
                    "type_params": []
                },
                "op": {"type": "SetValue", "values": [10, 20]}
            })
        );
    }

    #[test]
    fn test_execute_script_with_genesis_state_provided() {
        let sender = get_sender();
        let mut deps = io::load_module_files(vec![get_stdlib_path()]).unwrap();
        deps.push(get_record_module_dep());

        let script_text = r"
script {
    use 0x111111111111111111111111::Record;

    fun main() {
        let record = Record::with_incremented_age();
        Record::save(record);
    }
}";

        let genesis = serde_json::json!([{
            "struct_tag": {
                "address": sender.to_string(),
                "module": "Record",
                "name": "T",
                "type_params": []
            },
            "op": {"type": "SetValue", "values": [10, 20]}
        }]);
        let genesis: Vec<ResourceChange> = serde_json::from_value(genesis).unwrap();
        let vm_res = compile_and_run(
            (get_script_path(), script_text.to_string()),
            &deps,
            sender,
            Some(genesis),
        )
        .unwrap();
        let changes = vm_res.unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(
            serde_json::to_value(&changes[0]).unwrap(),
            serde_json::json!({
                "struct_tag": {
                    "address": sender.to_string(),
                    "module": "Record",
                    "name": "T",
                    "type_params": []
                },
                "op": {"type": "SetValue", "values": [11, 20]}
            })
        );
    }
}
