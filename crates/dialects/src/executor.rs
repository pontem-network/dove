use crate::changes::{changes_into_writeset, ResourceChange};
use crate::dfinance;

use crate::dfinance::types::{AccountAddress, VMResult};
use crate::errors::CompilerError;
use libra_types::vm_error::StatusCode;
use libra_types::write_set::WriteOp;
use utils::FilePath;
use vm::errors::{vm_error, Location};

pub fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: String,
    genesis: Vec<ResourceChange>,
) -> Result<VMResult<Vec<ResourceChange>>, Vec<CompilerError>> {
    let sender = AccountAddress::from_hex_literal(&sender).unwrap();

    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) =
        dfinance::check_and_generate_bytecode(fname, &script_text, deps, sender.into())?;

    let write_set = changes_into_writeset(genesis);
    let network_state = dfinance::prepare_fake_network_state(compiled_modules, write_set);

    let serialized_script = dfinance::serialize_script(compiled_script);
    let changed_resources =
        match dfinance::execute_script(sender, &network_state, serialized_script, vec![]) {
            Ok(ws) => ws,
            Err(vm_status) => return Ok(Err(vm_status)),
        };
    let mut changes = vec![];
    for (_, global_val) in changed_resources {
        match global_val {
            None => {
                // deletion is not yet supported
                continue;
            }
            Some((struct_type, global_val)) => {
                if !global_val.is_clean().unwrap() {
                    // into_owned_struct will check if all references are properly released
                    // at the end of a transaction
                    let data = global_val.into_owned_struct().unwrap();
                    let val = match data.simple_serialize(&struct_type) {
                        Some(blob) => blob,
                        None => {
                            return Ok(Err(vm_error(
                                Location::new(),
                                StatusCode::VALUE_SERIALIZATION_ERROR,
                            )));
                        }
                    };
                    let change = ResourceChange::new(struct_type, WriteOp::Value(val));
                    changes.push(change);
                }
            }
        }
    }

    Ok(Ok(changes))
}

#[cfg(test)]
mod tests {
    use utils::tests::{existing_file_abspath, get_modules_path, get_stdlib_path};

    use super::*;
    use crate::dfinance::types::AccountAddress;
    use utils::{io, leaked_fpath};

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
        let text = r"
script {
    use 0x0::Transaction;

    fun main() {
        let _ = Transaction::sender();
    }
}";
        let errors = compile_and_run(
            (get_script_path(), text.to_string()),
            &[],
            "0x111111111111111111111111".to_string(),
            vec![],
        )
        .unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].parts[0].message,
            "Unbound module \'0x0::Transaction\'"
        );
    }

    #[test]
    fn test_execute_custom_script_with_stdlib_module() {
        let text = r"
script {
    use 0x0::Transaction;

    fun main() {
        let _ = Transaction::sender();
    }
}";
        let deps = io::load_move_module_files(vec![get_stdlib_path()]).unwrap();
        let vm_res = compile_and_run(
            (existing_file_abspath(), text.to_string()),
            &deps,
            "0x111111111111111111111111".to_string(),
            vec![],
        )
        .unwrap();
        assert!(vm_res.is_ok());
    }

    #[test]
    fn test_execute_script_and_record_resource_changes() {
        let sender = get_sender();
        let mut deps = io::load_move_module_files(vec![get_stdlib_path()]).unwrap();
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
            "0x111111111111111111111111".to_string(),
            vec![],
        )
        .unwrap();
        let changes = vm_res.unwrap();
        assert_eq!(changes.len(), 1);

        assert_eq!(
            serde_json::to_value(&changes[0]).unwrap(),
            serde_json::json!({
                "struct_type": {
                    "address": sender.to_string(),
                    "is_resource": true,
                    "module": "Record",
                    "name": "T",
                    "ty_args": [],
                    "layout": ["U8", "U8"]
                },
                "op": {"type": "SetValue", "values": [10, 20]}
            })
        );
    }

    #[test]
    fn test_execute_script_with_genesis_state_provided() {
        let sender = get_sender();
        let mut deps = io::load_move_module_files(vec![get_stdlib_path()]).unwrap();
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
            "struct_type": {
                "address": sender.to_string(),
                "module": "Record",
                "name": "T",
                "is_resource": true,
                "ty_args": [],
                "layout": ["U8", "U8"]
            },
            "op": {"type": "SetValue", "values": [10, 20]}
        }]);
        let genesis: Vec<ResourceChange> = serde_json::from_value(genesis).unwrap();
        let vm_res = compile_and_run(
            (get_script_path(), script_text.to_string()),
            &deps,
            "0x111111111111111111111111".to_string(),
            genesis,
        )
        .unwrap();
        let changes = vm_res.unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(
            serde_json::to_value(&changes[0]).unwrap(),
            serde_json::json!({
                "struct_type": {
                    "address": sender.to_string(),
                    "module": "Record",
                    "name": "T",
                    "is_resource": true,
                    "ty_args": [],
                    "layout": ["U8", "U8"]
                },
                "op": {"type": "SetValue", "values": [11, 20]}
            })
        );
    }

    #[test]
    fn missing_writesets_for_move_to_sender() {
        let module_text = r"
address 0x1 {
    module M {
        resource struct T { value: u8 }

        public fun get_t(v: u8) {
            move_to_sender<T>(T { value: v })
        }
    }
}
        ";
        let script_text = r"
script {
    fun main() {
        0x1::M::get_t(10);
    }
}
        ";
        let mut deps = io::load_move_module_files(vec![get_stdlib_path()]).unwrap();
        deps.push((
            leaked_fpath(get_modules_path().join("m.move")),
            module_text.to_string(),
        ));

        let vm_res = compile_and_run(
            (get_script_path(), script_text.to_string()),
            &deps,
            "0x1".to_string(),
            vec![],
        )
        .unwrap();
        let changes = vm_res.unwrap();
        println!("{}", serde_json::to_string_pretty(&changes).unwrap());
    }
}
