use std::collections::HashMap;

use language_e2e_tests::data_store::FakeDataStore;
use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_lang::compiled_unit::CompiledUnit;
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

#[allow(dead_code)]
pub(crate) fn execute_script(
    sender_address: AccountAddress,
    network_state: HashMap<AccessPath, Vec<u8>>,
    script: Vec<u8>,
    args: Vec<Value>,
) -> VMResult<()> {
    let data_store = FakeDataStore::new(network_state);
    let cache = BlockDataCache::new(&data_store);

    let mut exec_context = SystemExecutionContext::new(&cache, GasUnits::new(1000));
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

#[cfg(test)]
mod tests {
    use move_lang::shared::Address;

    use analysis::utils::tests::existing_file_abspath;

    use super::*;

    fn _get_records_collection_module() -> String {
        r"
address 0x111111111111111111111111:

module RecordsCollection {
    use 0x0::Transaction as Tx;
    use 0x0::Vector;
    struct Record {
        name:   vector<u8>,
        author: vector<u8>,
        year:   u64
    }
    resource struct T {
        records: vector<Record>
    }
    fun initialize(sender: address) {
        if (!::exists<T>(sender)) {
            move_to_sender<T>(T { records: Vector::empty() })
        }
    }
    public fun add_to_my_collection(
        name: vector<u8>,
        author: vector<u8>,
        year: u64
    ) acquires T {
        let sender = Tx::sender();
        initialize(sender);
        let record = Record { name, author, year };
        let collection = borrow_global_mut<T>(sender);
        Vector::push_back(&mut collection.records, record)
    }
    public fun get_my_collection(): vector<Record> acquires T {
        let sender = Tx::sender();
        let collection = borrow_global<T>(sender);
        *&collection.records
    }
    public fun remove_from_me(): T acquires T {
        move_from<T>(Tx::sender())
    }
}"
        .to_string()
    }

    #[test]
    fn test_execute_empty_script() {
        let text = "fun main() {}";
        let script = compile_script(existing_file_abspath(), text, vec![], Address::default())
            .unwrap()
            .0;
        let res = execute_script(
            AccountAddress::default(),
            HashMap::new(),
            serialize_script(script),
            vec![],
        );
        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
    }

    // #[test]
    // fn test_execute_custom_script_with_stdlib_modules() {
    //     let sender = Address::new([1; 24]);
    //     let text = r"
    // use 0x0::Transaction;
    // use 0x0::LibraAccount;
    // use 0x0::LBR;
    //
    // fun main() {
    //     LibraAccount::balance<LBR::T>(Transaction::sender());
    // }";
    //     let stdlib_deps = io::get_module_files(get_stdlib_path().as_path());
    //     let script = compile_script(existing_file_abspath(), text, stdlib_deps, sender).unwrap();
    //     let mut network_state = HashMap::new();
    //
    //     let res = execute_script(AccountAddress::new([1; 24]), network_state, script, vec![]);
    //     assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
    // }
}
