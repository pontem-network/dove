use move_executor::compile_and_execute_script;
use shared::errors::ExecCompilerError;

use integration_tests::{
    existing_file_abspath, get_modules_path, get_script_path, get_stdlib_path,
};
use utils::{io, leaked_fpath, MoveFile};

fn stdlib_transaction_mod() -> MoveFile {
    io::load_move_file(get_stdlib_path().join("transaction.move")).unwrap()
}

fn record_mod() -> MoveFile {
    let text = r"
address 0x1111111111111111 {
    module Record {
        use 0x0::Transaction;

        resource struct T {
            age: u8,
        }

        public fun create(age: u8): T {
            T { age }
        }

        public fun save(record: T) {
            move_to_sender<T>(record);
        }

        public fun remove_record(user: address): T acquires T {
            move_from<T>(user)
        }

        public fun destroy_record() acquires T {
            let T { age: _ } = move_from<T>(Transaction::sender());
        }

        public fun with_doubled_age(): T acquires T {
            let record: T;
            record = move_from<T>(Transaction::sender());
            record.age = record.age * 2;
            record
        }
    }
}
        "
    .to_string();
    let fpath = leaked_fpath(get_modules_path().join("record.move"));
    (fpath, text)
}

#[test]
fn test_show_compilation_errors() {
    let text = r"
script {
    fun main() {
        let _ = 0x0::Transaction::sender();
    }
}";
    let errors = compile_and_execute_script(
        (get_script_path(), text.to_string()),
        &[],
        "libra",
        "0x1111111111111111",
        serde_json::json!([]),
        vec![],
    )
    .unwrap_err()
    .downcast::<ExecCompilerError>()
    .unwrap()
    .0;
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
    let deps = vec![stdlib_transaction_mod()];
    compile_and_execute_script(
        (existing_file_abspath(), text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
}

#[test]
fn test_execute_script_and_record_resource_changes() {
    let script_text = r"
script {
    use 0x1111111111111111::Record;

    fun main() {
        let record = Record::create(10);
        Record::save(record);
    }
}";
    let deps = vec![stdlib_transaction_mod(), record_mod()];

    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(
        changes,
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x00000000000000001111111111111111",
                "module": "Record",
                "name": "T",
                "ty_args": [],
                "layout": ["U8"]
            },
            "op": {"type": "SetValue", "values": [10]}
        }])
    );
}

#[test]
fn test_execute_script_with_genesis_state_provided() {
    let script_text = r"
script {
    use 0x1111111111111111::Record;

    fun main() {
        let record = Record::with_doubled_age();
        Record::save(record);
    }
}";
    let deps = vec![stdlib_transaction_mod(), record_mod()];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1111111111111111",
        "ty": {
            "address": "0x00000000000000001111111111111111",
            "module": "Record",
            "name": "T",
            "ty_args": [],
            "layout": ["U8"]
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        changes,
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x00000000000000001111111111111111",
                "module": "Record",
                "name": "T",
                "ty_args": [],
                "layout": ["U8"]
            },
            "op": {"type": "SetValue", "values": [20]}
        }])
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
    let mut deps = vec![];
    deps.push((
        leaked_fpath(get_modules_path().join("m.move")),
        module_text.to_string(),
    ));

    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(changes).unwrap(),
        serde_json::json!([
          {
            "account": "0x1",
            "ty": {
              "address": "0x00000000000000000000000000000001",
              "module": "M",
              "name": "T",
              "ty_args": [],
              "layout": [
                "U8"
              ]
            },
            "op": {"type": "SetValue", "values": [10]}
          }
        ])
    );
}

#[test]
fn test_run_with_non_default_dfinance_dialect() {
    let module_source_text = r"
address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
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
        wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::M::get_t(10);
    }
}
    ";

    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("m.move")),
            module_source_text.to_string(),
        )],
        "dfinance",
        "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();

    assert_eq!(
        changes,
        serde_json::json!([
          {
            "account": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
            "ty": {
              "address": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
              "module": "M",
              "name": "T",
              "ty_args": [],
              "layout": [
                "U8"
              ]
            },
            "op": {"type": "SetValue", "values": [10]}
          }
        ])
    );
}

#[test]
fn test_pass_arguments_to_script() {
    let module_source_text = r"
address 0x1 {
    module Module {
        resource struct T { value: bool }
        public fun create_t(v: bool) {
            move_to_sender<T>(T { value: v })
        }
    }
}
    ";
    let script_text = r"
script {
    use 0x1::Module;

    fun main(val: bool) {
        Module::create_t(val);
    }
}
    ";

    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("m.move")),
            module_source_text.to_string(),
        )],
        "libra",
        "0x1",
        serde_json::json!([]),
        vec![String::from("true")],
    )
    .unwrap();

    assert_eq!(
        changes,
        serde_json::json!([
          {
            "account": "0x1",
            "ty": {
              "address": "0x00000000000000000000000000000001",
              "module": "Module",
              "name": "T",
              "ty_args": [],
              "layout": [
                "Bool"
              ]
            },
            "op": {"type": "SetValue", "values": [1]}
          }
        ])
    );
}

#[test]
fn test_sender_string_in_script() {
    let module_text = r"
address {{sender}} {
    module Debug {
        public fun debug(): u8 {
            1
        }
    }
}";
    let source_text = r"
script {
    use {{sender}}::Debug;
    fun main() {
        let _ = Debug::debug();
    }
}
        ";
    let changes = compile_and_execute_script(
        (get_script_path(), source_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("debug.move")),
            module_text.to_string(),
        )],
        "libra",
        "0x1",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(changes, serde_json::json!([]));
}

#[test]
fn test_resource_move_from_sender() {
    let script_text = r"
script {
    use 0x1111111111111111::Record;

    fun main() {
        Record::destroy_record();
    }
}";
    let deps = vec![stdlib_transaction_mod(), record_mod()];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1111111111111111",
        "ty": {
            "address": "0x00000000000000001111111111111111",
            "module": "Record",
            "name": "T",
            "ty_args": [],
            "layout": ["U8"]
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        changes,
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x00000000000000001111111111111111",
                "module": "Record",
                "name": "T",
                "ty_args": [],
                "layout": ["U8"]
            },
            "op": {"type": "Delete"}
        }])
    );
}

#[test]
fn move_resource_from_another_user_to_sender() {
    let script_text = r"
script {
    use 0x1111111111111111::Record;

    fun main() {
        let record = Record::remove_record(0x1111111111111111);
        Record::save(record);
    }
}";
    let deps = vec![stdlib_transaction_mod(), record_mod()];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1111111111111111",
        "ty": {
            "address": "0x00000000000000001111111111111111",
            "module": "Record",
            "name": "T",
            "ty_args": [],
            "layout": ["U8"]
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x2222222222222222",
        initial_chain_state,
        vec![],
    )
    .unwrap();

    assert_eq!(
        changes,
        serde_json::json!([
            {
                "account": "0x1111111111111111",
                "ty": {
                    "address": "0x00000000000000001111111111111111",
                    "module": "Record",
                    "name": "T",
                    "ty_args": [],
                    "layout": ["U8"]
                },
                "op": {"type": "Delete"},
            },
            {
                "account": "0x2222222222222222",
                "ty": {
                    "address": "0x00000000000000001111111111111111",
                    "module": "Record",
                    "name": "T",
                    "ty_args": [],
                    "layout": ["U8"]
                },
                "op": {"type": "SetValue", "values": [10]},
            }
        ])
    );
}

#[test]
fn test_bech32_address_and_sender_in_compiler_error() {
    let text = r"
script {
    fun main() {
        let _ = {{sender}}::Unknown::unknown();
    }
}
        ";
    let exec_error = compile_and_execute_script(
        (get_script_path(), text.to_string()),
        &[],
        "dfinance",
        "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        serde_json::json!([]),
        vec![],
    )
    .unwrap_err()
    .downcast::<ExecCompilerError>()
    .unwrap();

    let errors = exec_error.transform_with_source_map();
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].parts[0].message,
        "Unbound module \'wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8::Unknown\'"
    );
}

#[test]
fn test_bech32_in_genesis_json() {
    let script_text = r"
script {
    use 0x1111111111111111::Record;

    fun main() {
        let record = Record::with_doubled_age();
        Record::save(record);
    }
}";
    let deps = vec![stdlib_transaction_mod(), record_mod()];
    let initial_chain_state = serde_json::json!([{
        "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        "ty": {
            "address": "0x0000000000000000000000001111111111111111",
            "module": "Record",
            "name": "T",
            "ty_args": [],
            "layout": ["U8"]
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);

    let changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "dfinance",
        "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        changes,
        serde_json::json!([{
            "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
            "ty": {
                "address": "0x0000000000000000000000001111111111111111",
                "module": "Record",
                "name": "T",
                "ty_args": [],
                "layout": ["U8"]
            },
            "op": {"type": "SetValue", "values": [20]}
        }])
    );
}
