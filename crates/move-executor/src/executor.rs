use dialects::dfinance::{
    execute_script, get_resource_structs, serialize_write_set, AccountAddress, Address, Errors,
    FakeDataStore, VMResult,
};
use dialects::resources::{changes_into_writeset, ResourceChange};
use dialects::{dfinance, FilePath};

pub fn compile_and_run(
    script: (FilePath, String),
    deps: &[(FilePath, String)],
    sender: AccountAddress,
    genesis: Option<Vec<ResourceChange>>,
) -> Result<VMResult<Vec<ResourceChange>>, Errors> {
    let (fname, script_text) = script;

    let (compiled_script, compiled_modules) = dialects::dfinance::compile_script(
        fname,
        &script_text,
        deps,
        Address::new(sender.into()),
    )?;
    let available_resource_structs = get_resource_structs(&compiled_script);

    let mut network_state = FakeDataStore::default();
    for module in compiled_modules {
        network_state.add_module(&module.self_id(), &module);
    }
    if let Some(changes) = genesis {
        let write_set = changes_into_writeset(changes);
        network_state.add_write_set(&write_set);
    }

    let serialized_script = dfinance::serialize_script(compiled_script);
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
    use dialects::dfinance::AccountAddress;

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
