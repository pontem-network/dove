use dialects::shared::errors::ExecCompilerError;
use move_executor::compile_and_execute_script;

use integration_tests::{
    existing_module_file_abspath, get_modules_path, get_script_path, modules_mod, stdlib_mod,
};
use utils::leaked_fpath;

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
    use 0x1::Signer;

    fun main(s: &signer) {
        let _ = Signer::address_of(s);
    }
}";
    let deps = vec![stdlib_mod("signer.move")];
    compile_and_execute_script(
        (existing_module_file_abspath(), text.to_string()),
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
    use 0x2::Record;

    fun main(s: &signer) {
        let record = Record::create(10);
        Record::save(s, record);
    }
}";
    let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];

    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(
        state_changes["changes"],
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x0000000000000000000000000000000000000002",
                "module": "Record",
                "name": "T",
                "ty_args": [],
            },
            "op": {"type": "SetValue", "values": [10]}
        }])
    );
}

#[test]
fn test_execute_script_with_genesis_state_provided() {
    let script_text = r"
script {
    use 0x2::Record;

    fun main(s: &signer) {
        let record = Record::with_doubled_age(s);
        Record::save(s, record);
    }
}";
    let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1111111111111111",
        "ty": {
            "address": "0x2",
            "module": "Record",
            "name": "T",
            "ty_args": [],
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        state_changes["changes"],
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x0000000000000000000000000000000000000002",
                "module": "Record",
                "name": "T",
                "ty_args": [],
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

        public fun get_t(s: &signer, v: u8) {
            move_to<T>(s, T { value: v })
        }
    }
}
        ";
    let script_text = r"
script {
    fun main(s: &signer) {
        0x1::M::get_t(s, 10);
    }
}
        ";
    let mut deps = vec![];
    deps.push((
        leaked_fpath(get_modules_path().join("m.move")),
        module_text.to_string(),
    ));

    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(
        state_changes["changes"],
        serde_json::json!([
          {
            "account": "0x1",
            "ty": {
              "address": "0x1",
              "module": "M",
              "name": "T",
              "ty_args": [],
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
        public fun get_t(s: &signer, v: u8) {
            move_to<T>(s, T { value: v })
        }
    }
}
    ";
    let script_text = r"
script {
    fun main(s: &signer) {
        wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::M::get_t(s, 10);
    }
}
    ";

    let state_changes = compile_and_execute_script(
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
        state_changes["changes"],
        serde_json::json!([
          {
            "account": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
            "ty": {
              "address": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
              "module": "M",
              "name": "T",
              "ty_args": [],
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
        public fun create_t(s: &signer, v: bool) {
            move_to<T>(s, T { value: v })
        }
    }
}
    ";
    let script_text = r"
script {
    use 0x1::Module;

    fun main(s: &signer, val: bool) {
        Module::create_t(s, val);
    }
}
    ";

    let state_changes = compile_and_execute_script(
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
        state_changes["changes"],
        serde_json::json!([
          {
            "account": "0x1",
            "ty": {
              "address": "0x1",
              "module": "Module",
              "name": "T",
              "ty_args": [],
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
    let state_changes = compile_and_execute_script(
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
    assert_eq!(state_changes["changes"], serde_json::json!([]));
}

#[test]
fn test_resource_move_from_sender() {
    let script_text = r"
script {
    use 0x2::Record;

    fun main(s: &signer) {
        Record::destroy_record(s);
    }
}";
    let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1111111111111111",
        "ty": {
            "address": "0x2",
            "module": "Record",
            "name": "T",
            "ty_args": [],
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        state_changes["changes"],
        serde_json::json!([{
            "account": "0x1111111111111111",
            "ty": {
                "address": "0x0000000000000000000000000000000000000002",
                "module": "Record",
                "name": "T",
                "ty_args": [],
            },
            "op": {"type": "Delete"}
        }])
    );
}

#[test]
fn move_resource_from_another_user_to_sender() {
    let script_text = r"
script {
    use 0x2::Record;

    fun main(s: &signer) {
        let original_record_owner = 0x1;
        let record = Record::get_record(original_record_owner);
        Record::save(s, record);
    }
}";
    let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];

    let initial_chain_state = serde_json::json!([{
        "account": "0x1",
        "ty": {
            "address": "0x2",
            "module": "Record",
            "name": "T",
            "ty_args": [],
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);
    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x3",
        initial_chain_state,
        vec![],
    )
    .unwrap();

    assert_eq!(
        state_changes["changes"],
        serde_json::json!([
            {
                "account": "0x1",
                "ty": {
                    "address": "0x0000000000000000000000000000000000000002",
                    "module": "Record",
                    "name": "T",
                    "ty_args": [],
                },
                "op": {"type": "Delete"},
            },
            {
                "account": "0x3",
                "ty": {
                    "address": "0x0000000000000000000000000000000000000002",
                    "module": "Record",
                    "name": "T",
                    "ty_args": [],
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
    use 0x2::Record;

    fun main(s: &signer) {
        let record = Record::with_doubled_age(s);
        Record::save(s, record);
    }
}";
    let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];
    let initial_chain_state = serde_json::json!([{
        "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        "ty": {
            "address": "0x0000000000000000000000000000000000000002",
            "module": "Record",
            "name": "T",
            "ty_args": [],
        },
        "op": {"type": "SetValue", "values": [10]}
    }]);

    let state_changes = compile_and_execute_script(
        (get_script_path(), script_text.to_string()),
        &deps,
        "dfinance",
        "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        initial_chain_state,
        vec![],
    )
    .unwrap();
    assert_eq!(
        state_changes["changes"],
        serde_json::json!([{
            "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
            "ty": {
                "address": "0x0000000000000000000000000000000000000002",
                "module": "Record",
                "name": "T",
                "ty_args": [],
            },
            "op": {"type": "SetValue", "values": [20]}
        }])
    );
}

#[test]
fn test_show_executor_gas_in_genesis_if_gas_flag_is_present() {
    let text = r"
script {
    use 0x1::Signer;

    fun main(s: &signer) {
        let _ = Signer::address_of(s);
    }
}";
    let deps = vec![stdlib_mod("signer.move")];
    let chain_state = compile_and_execute_script(
        (existing_module_file_abspath(), text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    assert_eq!(chain_state["gas_spent"], 7);
}

#[test]
fn test_dfinance_executor_allows_0x0() {
    let text = r"
script {
    fun main() {}
}";
    compile_and_execute_script(
        (existing_module_file_abspath(), text.to_string()),
        &[],
        "dfinance",
        "0x0",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();

    compile_and_execute_script(
        (existing_module_file_abspath(), text.to_string()),
        &[],
        "dfinance",
        "0x1",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
}

#[test]
fn test_execute_script_with_custom_signer() {
    let text = r"
/// signer: 0x2
script {
    use 0x2::Record;

    fun test_create_record(s1: &signer) {
        let r1 = Record::create(20);
        Record::save(s1, r1);
    }
}
    ";
    let changes = compile_and_execute_script(
        (get_script_path(), text.to_string()),
        &[stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    let account1_change = changes["changes"][0].clone();
    assert_eq!(
        account1_change["account"],
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(account1_change["op"]["values"][0], 20);
}

#[test]
fn test_multiple_signers() {
    let text = r"
    /// signer: 0x1
    /// signer: 0x2
    script {
        use 0x2::Record;

        fun test_multiple_signers(s1: &signer, s2: &signer) {
            let r1 = Record::create(10);
            Record::save(s1, r1);

            let r2 = Record::create(20);
            Record::save(s2, r2);
        }
    }
    ";

    let changes = compile_and_execute_script(
        (get_script_path(), text.to_string()),
        &[stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    let account1_change = changes["changes"][0].clone();
    assert_eq!(
        account1_change["account"],
        "0x0000000000000000000000000000000000000001"
    );
    assert_eq!(account1_change["op"]["values"][0], 10);

    let account2_change = changes["changes"][1].clone();
    assert_eq!(
        account2_change["account"],
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(account2_change["op"]["values"][0], 20);
}

#[test]
fn test_execute_script_with_module_in_the_same_file() {
    let text = r"
address 0x2 {
    module Record {
        resource struct T {
            age: u8
        }

        public fun create(age: u8): T {
            T { age }
        }

        public fun save(account: &signer, record: T) {
            move_to<T>(account, record);
        }
    }
}

/// signer: 0x2
script {
    use 0x2::Record;

    fun test_create_record(s1: &signer) {
        let r1 = Record::create(20);
        Record::save(s1, r1);
    }
}
    ";
    let changes = compile_and_execute_script(
        (get_script_path(), text.to_string()),
        &[],
        "dfinance",
        "0x3",
        serde_json::json!([]),
        vec![],
    )
    .unwrap();
    let account1_change = changes["changes"][0].clone();
    assert_eq!(
        account1_change["account"],
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(account1_change["op"]["values"][0], 20);
}
