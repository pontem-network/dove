use move_executor::compile_and_run_scripts_in_file;

use utils::leaked_fpath;
use utils::tests::{
    get_script_path, existing_module_file_abspath, modules_mod, get_modules_path,
    anonymous_script_file, record_mod,
};
use move_executor::explain::AddressResourceChanges;
use lang::compiler::errors::ExecCompilerError;
use lang::file::MvFile;

#[test]
fn test_show_compilation_errors() {
    let text = r"
script {
    fun main() {
        let _ = 0x0::Transaction::sender();
    }
}";
    let errors = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[],
        "libra",
        "0x1111111111111111",
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
    compile_and_run_scripts_in_file(
            MvFile::with_content(existing_module_file_abspath(), text.to_string())
        &deps,
        "libra",
        "0x1111111111111111",
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

    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);
    assert_eq!(
        effects.resources()[0],
        AddressResourceChanges::new(
            "0x0000000000000000000000001111111111111111",
            vec!["Added type 00000000::Record::T: [U8(10)]".to_string()],
        )
    );
}

// #[test]
// fn test_execute_script_with_genesis_state_provided() {
//     let script_text = r"
// script {
//     use 0x2::Record;
//
//     fun main(s: &signer) {
//         let record = Record::with_doubled_age(s);
//         Record::save(s, record);
//     }
// }";
//     let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];
//
//     let initial_chain_state = serde_json::json!([{
//         "account": "0x1111111111111111",
//         "ty": {
//             "address": "0x2",
//             "module": "Record",
//             "name": "T",
//             "ty_args": [],
//         },
//         "op": {"type": "SetValue", "values": [10]}
//     }]);
//     let state_changes = compile_and_execute_script_script(
//         (get_script_path(), script_text.to_string()),
//         &deps,
//         "libra",
//         "0x1111111111111111",
//         // initial_chain_state,
//         vec![],
//     )
//     .unwrap();
//     assert_eq!(
//         state_changes["changes"],
//         serde_json::json!([{
//             "account": "0x1111111111111111",
//             "ty": {
//                 "address": "0x0000000000000000000000000000000000000002",
//                 "module": "Record",
//                 "name": "T",
//                 "ty_args": [],
//             },
//             "op": {"type": "SetValue", "values": [20]}
//         }])
//     );
// }

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

    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), script_text.to_string()),
        &deps,
        "libra",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);
    assert_eq!(
        effects.resources()[0],
        AddressResourceChanges::new(
            "0x0000000000000000000000000000000000000001",
            vec!["Added type 00000000::M::T: [U8(10)]".to_string()],
        )
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

    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), script_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("m.move")),
            module_source_text.to_string(),
        )],
        "dfinance",
        "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();

    assert_eq!(
        effects.resources()[0],
        AddressResourceChanges::new(
            "0xde5f86ce8ad7944f272d693cb4625a955b610150",
            vec!["Added type de5f86ce::M::T: [U8(10)]".to_string()],
        )
    );

    // assert_eq!(
    //     state_changes["changes"],
    //     serde_json::json!([
    //       {
    //         "account": "0xde5f86ce8ad7944f272d693cb4625a955b610150",
    //         "ty": {
    //           "address": "0xde5f86ce8ad7944f272d693cb4625a955b610150",
    //           "module": "M",
    //           "name": "T",
    //           "ty_args": [],
    //         },
    //         "op": {"type": "SetValue", "values": [10]}
    //       }
    //     ])
    // );
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

    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), script_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("m.move")),
            module_source_text.to_string(),
        )],
        "libra",
        "0x1",
        vec![String::from("true")],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();

    assert_eq!(effects.resources().len(), 1);
    assert_eq!(
        effects.resources()[0],
        AddressResourceChanges::new(
            "0x0000000000000000000000000000000000000001",
            vec!["Added type 00000000::Module::T: [true]".to_string()]
        )
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
    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), source_text.to_string()),
        &[(
            leaked_fpath(get_modules_path().join("debug.move")),
            module_text.to_string(),
        )],
        "libra",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 0);
}

// #[test]
// fn test_resource_move_from_sender() {
//     let script_text = r"
// /// resource: 0x1111111111111111 0x2::Record::T [U8(10)]
// script {
//     use 0x2::Record;
//
//     fun main(s: &signer) {
//         Record::destroy_record(s);
//     }
// }";
//     let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];
//
//     // let initial_chain_state = serde_json::json!([{
//     //     "account": "0x1111111111111111",
//     //     "ty": {
//     //         "address": "0x2",
//     //         "module": "Record",
//     //         "name": "T",
//     //         "ty_args": [],
//     //     },
//     //     "op": {"type": "SetValue", "values": [10]}
//     // }]);
//     let effects = compile_and_execute_script_script(
//         (get_script_path(), script_text.to_string()),
//         &deps,
//         "libra",
//         "0x1111111111111111",
//         vec![],
//     )
//     .unwrap()
//     .effects;
//     assert_eq!(effects.resources().len(), 1);
//     assert_eq!(effects.resources()[0].address, "0x1111111111111111");
//     assert_eq!(effects.resources()[0].changes[0], "Add");
// }

// #[test]
// fn move_resource_from_another_user_to_sender() {
//     let script_text = r"
// script {
//     use 0x2::Record;
//
//     fun main(s: &signer) {
//         let original_record_owner = 0x1;
//         let record = Record::get_record(original_record_owner);
//         Record::save(s, record);
//     }
// }";
//     let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];
//
//     let initial_chain_state = serde_json::json!([{
//         "account": "0x1",
//         "ty": {
//             "address": "0x2",
//             "module": "Record",
//             "name": "T",
//             "ty_args": [],
//         },
//         "op": {"type": "SetValue", "values": [10]}
//     }]);
//     let state_changes = compile_and_execute_script_script(
//         (get_script_path(), script_text.to_string()),
//         &deps,
//         "libra",
//         "0x3",
//         initial_chain_state,
//         vec![],
//     )
//     .unwrap();
//
//     assert_eq!(
//         state_changes["changes"],
//         serde_json::json!([
//             {
//                 "account": "0x1",
//                 "ty": {
//                     "address": "0x0000000000000000000000000000000000000002",
//                     "module": "Record",
//                     "name": "T",
//                     "ty_args": [],
//                 },
//                 "op": {"type": "Delete"},
//             },
//             {
//                 "account": "0x3",
//                 "ty": {
//                     "address": "0x0000000000000000000000000000000000000002",
//                     "module": "Record",
//                     "name": "T",
//                     "ty_args": [],
//                 },
//                 "op": {"type": "SetValue", "values": [10]},
//             }
//         ])
//     );
// }

#[test]
fn test_bech32_address_and_sender_in_compiler_error() {
    let text = r"
script {
    fun main() {
        let _ = {{sender}}::Unknown::unknown();
    }
}
        ";
    let exec_error = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[],
        "dfinance",
        "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
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

// #[test]
// fn test_bech32_in_genesis_json() {
//     let script_text = r"
// script {
//     use 0x2::Record;
//
//     fun main(s: &signer) {
//         let record = Record::with_doubled_age(s);
//         Record::save(s, record);
//     }
// }";
//     let deps = vec![stdlib_mod("signer.move"), modules_mod("record.move")];
//     let initial_chain_state = serde_json::json!([{
//         "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
//         "ty": {
//             "address": "0x0000000000000000000000000000000000000002",
//             "module": "Record",
//             "name": "T",
//             "ty_args": [],
//         },
//         "op": {"type": "SetValue", "values": [10]}
//     }]);
//
//     let state_changes = compile_and_execute_script_script(
//         (get_script_path(), script_text.to_string()),
//         &deps,
//         "dfinance",
//         "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
//         initial_chain_state,
//         vec![],
//     )
//     .unwrap();
//     assert_eq!(
//         state_changes["changes"],
//         serde_json::json!([{
//             "account": "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
//             "ty": {
//                 "address": "0x0000000000000000000000000000000000000002",
//                 "module": "Record",
//                 "name": "T",
//                 "ty_args": [],
//             },
//             "op": {"type": "SetValue", "values": [20]}
//         }])
//     );
// }

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
    let res = compile_and_run_scripts_in_file(
        (existing_module_file_abspath(), text.to_string()),
        &deps,
        "libra",
        "0x1111111111111111",
        vec![],
    )
    .unwrap();
    assert_eq!(res.gas_spent, 7);
}

#[test]
fn test_dfinance_executor_allows_0x0() {
    let text = r"
script {
    fun main() {}
}";
    compile_and_run_scripts_in_file(
        (existing_module_file_abspath(), text.to_string()),
        &[],
        "dfinance",
        "0x0",
        // serde_json::json!([]),
        vec![],
    )
    .unwrap();

    compile_and_run_scripts_in_file(
        (existing_module_file_abspath(), text.to_string()),
        &[],
        "dfinance",
        "0x1",
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
    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);
    assert_eq!(
        effects.resources()[0].address,
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(
        effects.resources()[0].changes[0],
        "Added type 00000000::Record::T: [U8(20)]"
    );
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

    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    let account1_change = &effects.resources()[0];
    assert_eq!(
        account1_change.address,
        "0x0000000000000000000000000000000000000001"
    );
    assert_eq!(
        account1_change.changes[0],
        "Added type 00000000::Record::T: [U8(10)]"
    );

    let account2_change = &effects.resources()[1];
    assert_eq!(
        account2_change.address,
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(
        account2_change.changes[0],
        "Added type 00000000::Record::T: [U8(20)]"
    );
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
    let effects = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);

    let account1_change = &effects.resources()[0];
    assert_eq!(
        account1_change.address,
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(
        account1_change.changes[0],
        "Added type 00000000::Record::T: [U8(20)]"
    );
}

#[test]
fn test_fail_with_assert() {
    let text = r"
script {
    fun main() {
        assert(1 == 0, 401);
    }
}
    ";
    let res = compile_and_run_scripts_in_file(
        (get_script_path(), text.to_string()),
        &[],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 401 in transaction script"
    );
}

// #[test]
// fn test_no_oracle_module_available_and_oracle_price_passed() {
//     let text = r"
// /// price: usd_btc 100
// script {
//     fun main() {}
// }
//     ";
//     let res = compile_and_run_first_script(
//         anonymous_script_file(text),
//         &[],
//         "dfinance",
//         "0x3",
//         vec![],
//     )
//     .unwrap();
//     assert_eq!(
//         res.error(),
//         "Execution aborted with code 401 in transaction script"
//     );
// }

#[test]
fn test_script_starts_from_line_0() {
    let text = r"script { fun main() { assert(false, 401); } }";
    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 401 in transaction script"
    );
}

#[test]
fn test_doc_comment_starts_at_line_0() {
    let text = r"/// signer: 0x1
script { fun main(_: &signer) { assert(false, 401); } }";
    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 401 in transaction script"
    );
}

#[test]
fn test_coin_price_fails_if_no_coins_module_available() {
    let text = r"
/// price: btc_usdt 100
script {
    fun main() {}
}
    ";
    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Cannot use `price:` comments: missing `0x1::Coins` module".to_string()
    );
}

#[test]
fn test_initialize_coin_price_before_run() {
    let text = r"
/// price: btc_usdt 100
script {
    use 0x1::Coins;
    use 0x1::Coins::{BTC, USDT};

    fun main() {
        let price = Coins::get_price<BTC, USDT>();
        assert(price == 100, 1);
    }
}
    ";
    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[stdlib_mod("coins.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert!(res.effects().resources().is_empty());
}

#[test]
fn test_run_scripts_in_sequential_order() {
    let text = r"
script {
    use 0x2::Record;

    fun step_2(s: &signer) {
        Record::create_record(s, 10);
    }
}

script {
    use 0x2::Record;

    fun step_1(s: &signer) {
        Record::increment_record(s);
    }
}
    ";

    let effects = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[stdlib_mod("signer.move"), record_mod()],
        "libra",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);
    assert_eq!(
        effects.resources()[0],
        AddressResourceChanges::new(
            "0x0000000000000000000000000000000000000003",
            vec!["Changed type 00000000::Record::T: [U8(11)]".to_string()]
        )
    );
}

#[test]
fn test_failure_in_first_script() {
    let text = r"
script {
    fun step_1() {
        assert(false, 1);
    }
}

script {
    fun step_2() {
        assert(true, 2);
    }
}
    ";

    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[stdlib_mod("signer.move"), record_mod()],
        "libra",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 1 in transaction script"
    );
}

#[test]
fn test_failure_in_second_script() {
    let text = r"
script {
    fun step_1() {
        assert(true, 1);
    }
}

script {
    fun step_2() {
        assert(false, 2);
    }
}
    ";

    let res = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[stdlib_mod("signer.move"), record_mod()],
        "libra",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 2 in transaction script"
    );
}

#[test]
fn test_run_scripts_and_set_oracles_before_each_step() {
    let text = r"
/// price: btc_usdt 100
script {
    use 0x1::Coins;
    use 0x1::Coins::{BTC, USDT};

    fun test_1() {
        let price = Coins::get_price<BTC, USDT>();
        assert(price == 100, 1);
    }
}

/// price: btc_usdt 200
script {
    use 0x1::Coins;
    use 0x1::Coins::{BTC, USDT};

    fun test_3() {
        let price = Coins::get_price<BTC, USDT>();
        assert(price == 200, 1);
    }
}
    ";

    let results = compile_and_run_scripts_in_file(
        anonymous_script_file(text),
        &[stdlib_mod("coins.move")],
        "libra",
        "0x1",
        vec![],
    )
    .unwrap();
    assert_eq!(results.gas_spent, 10);
}

pub fn stdlib_mod(name: &str) -> MvFile {
    //io::load_move_file(get_stdlib_path().join(name)).unwrap()
    todo!()
}
