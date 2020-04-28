use language_e2e_tests::data_store::FakeDataStore;

use libra_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use move_vm_runtime::MoveVM;
use move_vm_state::data_cache::BlockDataCache;
use move_vm_state::execution_context::SystemExecutionContext;
use move_vm_types::values::Value;
use vm::errors::VMResult;
use vm::gas_schedule::zero_cost_schedule;
use vm::transaction_metadata::TransactionMetadata;

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

#[allow(dead_code)]
pub(crate) fn execute_script(
    sender_address: AccountAddress,
    script: Vec<u8>,
    args: Vec<Value>,
) -> VMResult<()> {
    let data_store = FakeDataStore::default();
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
    use move_lang::compile_program;
    use move_lang::errors::Errors;
    use move_lang::parser::ast::Program;

    use move_lang::shared::Address;

    use analysis::compiler::parse_file;
    use analysis::utils::tests::existing_file_abspath;

    use super::*;

    fn compile_script(text: &str, deps: &[String], sender: Address) -> Result<Vec<u8>, Errors> {
        let parsed_file = parse_file(existing_file_abspath(), text).map_err(|err| vec![err])?;

        let mut parsed_deps = vec![];
        for _dep in deps {
            let parsed = parse_file(existing_file_abspath(), text).map_err(|e| vec![e])?;
            parsed_deps.push(parsed);
        }
        let program = Program {
            source_definitions: vec![parsed_file],
            lib_definitions: parsed_deps,
        };
        let compiled = compile_program(Ok(program), Some(sender))?.remove(0);
        Ok(compiled.serialize())
    }

    #[test]
    fn test_execute_empty_script() {
        let text = "fun main() {}";
        let script = compile_script(text, &[], Address::default()).unwrap();
        let res = execute_script(AccountAddress::default(), script, vec![]);
        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
    }
}
