use lang::executor::compile_and_run;
use lang::resources::changes_into_writeset;
use lang::types::WriteSet;

use shared::errors::ExecCompilerError;
use utils::tests::{existing_file_abspath, get_modules_path, get_stdlib_path};
use utils::{io, leaked_fpath, FilePath};

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

fn get_sender() -> String {
    "0x111111111111111111111111".to_string()
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
        WriteSet::default(),
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
    let deps = io::load_move_module_files(vec![get_stdlib_path()]).unwrap();
    compile_and_run(
        (existing_file_abspath(), text.to_string()),
        &deps,
        get_sender(),
        WriteSet::default(),
    )
    .unwrap();
}

#[test]
fn test_execute_script_and_record_resource_changes() {
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

    let changes = compile_and_run(
        (get_script_path(), script_text.to_string()),
        &deps,
        get_sender(),
        WriteSet::default(),
    )
    .unwrap();
    assert_eq!(changes.len(), 1);

    assert_eq!(
        serde_json::to_value(&changes[0]).unwrap(),
        serde_json::json!({
            "ty": {
                "address": "0x000000000000000000000000111111111111111111111111",
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
    let _sender = get_sender();
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
        "ty": {
            "address": "0x000000000000000000000000111111111111111111111111",
            "module": "Record",
            "name": "T",
            "ty_args": [],
            "layout": ["U8", "U8"]
        },
        "op": {"type": "SetValue", "values": [10, 20]}
    }]);
    let changes = serde_json::from_value(genesis).unwrap();
    let genesis_write_set = changes_into_writeset(changes).unwrap();
    let changes = compile_and_run(
        (get_script_path(), script_text.to_string()),
        &deps,
        get_sender(),
        genesis_write_set,
    )
    .unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(
        serde_json::to_value(&changes[0]).unwrap(),
        serde_json::json!({
            "ty": {
                "address": "0x000000000000000000000000111111111111111111111111",
                "module": "Record",
                "name": "T",
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

    let sender = "0x1".to_string();
    let changes = compile_and_run(
        (get_script_path(), script_text.to_string()),
        &deps,
        sender,
        WriteSet::default(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(changes).unwrap(),
        serde_json::json!([
          {
            "ty": {
              "address": "0x000000000000000000000000000000000000000000000001",
              "module": "M",
              "name": "T",
              "ty_args": [],
              "layout": [
                "U8"
              ]
            },
            "op": {
              "type": "SetValue",
              "values": [
                10
              ]
            }
          }
        ])
    );
}
