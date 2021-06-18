use move_executor::explain::{ChangeType, AddressResourceChanges, ResourceChange, PipelineExecutionResult};
use lang::compiler::file::MoveFile;
use resources::{assets_dir, stdlib_path, modules_path};
use lang::compiler::error::CompilerError;
use move_executor::executor::Executor;
use anyhow::{Error, Context};
use lang::compiler::dialects::DialectName;
use std::str::FromStr;

fn script_path() -> String {
    assets_dir()
        .join("script.move")
        .to_str()
        .unwrap()
        .to_owned()
}

fn module_path(name: &str) -> String {
    assets_dir().join(name).to_str().unwrap().to_owned()
}

pub fn stdlib_mod(name: &str) -> MoveFile {
    MoveFile::load(stdlib_path().join(name)).unwrap()
}

pub fn modules_mod(name: &str) -> MoveFile {
    MoveFile::load(modules_path().join(name)).unwrap()
}

fn execute_script(
    script: MoveFile,
    deps: Vec<MoveFile>,
    dialect: &str,
    address: &str,
    args: Vec<String>,
) -> Result<PipelineExecutionResult, Error> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let sender = dialect
        .parse_address(address)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), address))?;

    let executor = Executor::new(dialect.as_ref(), sender, deps);
    executor.execute_script(script, None, args)
}

#[test]
fn test_show_compilation_errors() {
    let text = r"
script {
    fun main() {
        let _ = 0x0::Transaction::sender();
    }
}";
    let errors = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "diem",
        "0x1111111111111111",
        vec![],
    )
    .unwrap_err()
    .downcast::<CompilerError>()
    .unwrap()
    .errors;
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0][0].1, "Unbound module \'0x0::Transaction\'");
}

#[test]
fn test_execute_custom_script_with_stdlib_module() {
    let text = r"
    script {
        use 0x1::Signer;

        fun main(s: signer) {
            let _ = Signer::address_of(&s);
        }
    }";
    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move")],
        "diem",
        "0x1111111111111111",
        vec![],
    )
    .unwrap();
}

#[test]
fn test_execute_script_and_record_resource_changes() {
    let text = r"
script {
    use 0x2::Record;

    fun main(s: signer) {
        let record = Record::create(10);
        Record::save(&s, record);
    }
}";

    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
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
            "0x1111111111111111",
            vec![(
                ChangeType::Added,
                ResourceChange("0x2::Record::T".to_string(), Some("[U8(10)]".to_string()))
            )],
        )
    );
}

#[test]
fn missing_write_set_for_move_to_sender() {
    let module_text = r"
    address 0x1 {
        module M {
           struct T has store, key { value: u8 }

            public fun get_t(s: &signer, v: u8) {
                move_to<T>(s, T { value: v })
            }
        }
    }
        ";
    let script_text = r"
    script {
        fun main(s: signer) {
            0x1::M::get_t(&s, 10);
        }
    }
        ";
    let deps = vec![MoveFile::with_content(module_path("m.move"), module_text)];

    let effects = execute_script(
        MoveFile::with_content(script_path(), script_text),
        deps,
        "diem",
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
            "0x1",
            vec![(
                ChangeType::Added,
                ResourceChange("0x1::M::T".to_string(), Some("[U8(10)]".to_string()))
            )],
        )
    );
}

#[test]
fn test_run_with_non_default_dfinance_dialect() {
    let module_text = r"
    address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
        module M {
            struct T has store, key { value: u8 }
            public fun get_t(s: &signer, v: u8) {
                move_to<T>(s, T { value: v })
            }
        }
    }
    ";
    let script_text = r"
    script {
        fun main(s: signer) {
            wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::M::get_t(&s, 10);
        }
    }
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), script_text),
        vec![MoveFile::with_content(module_path("m.move"), module_text)],
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
            "0xDE5F86CE8AD7944F272D693CB4625A955B610150",
            vec![(
                ChangeType::Added,
                ResourceChange(
                    "0xDE5F86CE8AD7944F272D693CB4625A955B610150::M::T".to_string(),
                    Some("[U8(10)]".to_string())
                )
            )],
        )
    );
}

#[test]
fn test_pass_arguments_to_script() {
    let module_text = r"
    address 0x1 {
        module Module {
            struct T has store, key { value: bool }
            public fun create_t(s: &signer, v: bool) {
                move_to<T>(s, T { value: v })
            }
        }
    }
    ";
    let script_text = r"
    script {
        use 0x1::Module;

        fun main(s: signer, val: bool) {
            Module::create_t(&s, val);
        }
    }
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), script_text),
        vec![MoveFile::with_content(module_path("m.move"), module_text)],
        "diem",
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
            "0x1",
            vec![(
                ChangeType::Added,
                ResourceChange("0x1::Module::T".to_string(), Some("[true]".to_string()))
            )],
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
    let effects = execute_script(
        MoveFile::with_content(script_path(), source_text),
        vec![MoveFile::with_content(
            module_path("debug.move"),
            module_text,
        )],
        "diem",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 0);
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
    let errors = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "dfinance",
        "wallet1pxqfjvnu0utauj8fctw2s7j4mfyvrsjd59c2u8",
        vec![],
    )
    .unwrap_err()
    .downcast::<CompilerError>()
    .unwrap()
    .errors;

    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0][0].1,
        "Unbound module \'0x98099327C7F17DE48E9C2DCA87A55DA48C1C24D::Unknown\'"
    );
}

#[test]
fn test_show_executor_gas_spent() {
    let text = r"
    script {
        use 0x1::Signer;

        fun main(s: signer) {
            let _ = Signer::address_of(&s);
        }
    }";

    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move")],
        "diem",
        "0x1111111111111111",
        vec![],
    )
    .unwrap();
    assert_eq!(res.overall_gas_spent(), 7);
}

#[test]
fn test_dfinance_executor_allows_0x0() {
    let text = r"
    script {
        fun main() {}
    }";

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "dfinance",
        "0x0",
        vec![],
    )
    .unwrap();

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "dfinance",
        "0x1",
        vec![],
    )
    .unwrap();
}

#[test]
fn test_execute_script_with_custom_signer() {
    let text = r"
    /// signers: 0x2
    script {
        use 0x2::Record;

        fun test_create_record(s1: signer) {
            let r1 = Record::create(20);
            Record::save(&s1, r1);
        }
    }
    ";
    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.resources().len(), 1);
    assert_eq!(effects.resources()[0].address, "0x2");
    assert_eq!(
        effects.resources()[0].changes,
        vec![(
            ChangeType::Added,
            ResourceChange("0x2::Record::T".to_string(), Some("[U8(20)]".to_string()))
        )]
    );
}

#[test]
fn test_multiple_signers() {
    let text = r"
    /// signers: 0x1, 0x2
    script {
        use 0x2::Record;

        fun test_multiple_signers(s1: signer, s2: signer) {
            let r1 = Record::create(10);
            Record::save(&s1, r1);

            let r2 = Record::create(20);
            Record::save(&s2, r2);
        }
    }
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();

    let account1_change = &effects.resources()[0];
    assert_eq!(account1_change.address, "0x1");
    assert_eq!(
        account1_change.changes,
        vec![(
            ChangeType::Added,
            ResourceChange("0x2::Record::T".to_string(), Some("[U8(10)]".to_string()))
        )]
    );

    let account2_change = &effects.resources()[1];
    assert_eq!(account2_change.address, "0x2");
    assert_eq!(
        account2_change.changes,
        vec![(
            ChangeType::Added,
            ResourceChange("0x2::Record::T".to_string(), Some("[U8(20)]".to_string()))
        )]
    );
}

#[test]
fn test_execute_script_with_module_in_the_same_file() {
    let text = r"
address 0x2 {
    module Record {
        struct T has store, key {
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

/// signers: 0x2
script {
    use 0x2::Record;

    fun test_create_record(s1: signer) {
        let r1 = Record::create(20);
        Record::save(&s1, r1);
    }
}
    ";
    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
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
    assert_eq!(account1_change.address, "0x2");
    assert_eq!(
        account1_change.changes,
        vec![(
            ChangeType::Added,
            ResourceChange("0x2::Record::T".to_string(), Some("[U8(20)]".to_string()))
        )]
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
    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
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
fn test_script_starts_from_line_0() {
    let text = r"script { fun main() { assert(false, 401); } }";
    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
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
    let text = r"/// signers: 0x1
script { fun main(_: signer) { assert(false, 401); } }";
    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
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
    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
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
    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("coins.move")],
        "dfinance",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(res.effects().write_set_size(), 0);
}

#[test]
fn test_run_scripts_in_sequential_order() {
    let text = r"
script {
    use 0x2::Record;

    fun step_2(s: signer) {
        Record::create_record(&s, 10);
    }
}

script {
    use 0x2::Record;

    fun step_1(s: signer) {
        Record::increment_record(&s);
    }
}
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
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
            "0x3",
            vec![(
                ChangeType::Changed,
                ResourceChange("0x2::Record::T".to_string(), Some("[U8(11)]".to_string()))
            )],
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

    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
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

    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
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

    let results = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("coins.move")],
        "diem",
        "0x1",
        vec![],
    )
    .unwrap();
    assert_eq!(results.overall_gas_spent(), 10);
}

#[test]
fn test_set_current_time() {
    let text = r"
/// current_time: 100
script {
    use 0x1::Time;

    fun main() {
        assert(false, Time::now());
    }
}
    ";

    let res = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("time.move")],
        "diem",
        "0x1",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap();
    assert_eq!(
        res.error(),
        "Execution aborted with code 100 in transaction script"
    );
}

#[test]
fn test_shows_size_of_transaction_writeset() {
    let text = r"
script {
    use 0x2::Record;

    fun step_2(s: signer) {
        Record::create_record(&s, 10);
    }
}
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(effects.write_set_size(), 1);
}

#[test]
fn test_aborts_with() {
    let text = r"
/// aborts_with: 101
script {
    fun main() {
        assert(false, 101);
    }
}
    ";

    let error_string = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .expected_error();
    assert_eq!(
        error_string,
        "Expected error: Execution aborted with code 101 in transaction script"
    );
}

#[test]
fn test_extract_error_name_if_prefixed_with_err() {
    let text = r"
        script {
            use 0x1::Signer;
            use 0x2::Record;

            fun main(s: signer) {
                let record = Record::get_record(Signer::address_of(&s));
                Record::save(&s, record);
            }
        }
    ";

    let error_string = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .error();
    assert_eq!(
        error_string,
        "Execution aborted with code 101: ERR_RECORD_DOES_NOT_EXIST in module 0x2::Record."
    );
}

#[test]
fn test_dry_run_do_not_apply_writeset_changes() {
    let text = r"
script {
    use 0x2::Record;

    fun step_1(s: signer) {
        Record::create_record(&s, 10);
    }
}

/// dry_run: true
script {
    use 0x2::Record;

    fun step_2(s: signer) {
        Record::increment_record(&s);
    }
}

script {
    use 0x2::Record;

    fun step_3(s: signer) {
        Record::increment_record(&s);
    }
}
    ";

    let effects = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("signer.move"), modules_mod("record.move")],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
    assert_eq!(
        effects.resources()[0].changes[0].1,
        ResourceChange("0x2::Record::T".to_string(), Some("[U8(11)]".to_string()))
    );
}

#[test]
fn test_set_balance_via_meta() {
    let text = r#"
script {
    use 0x1::Pontem;
    use 0x1::Coins::ETH;

    fun register_coins() {
        Pontem::register_coin<ETH>(b"eth", 18);
    }
}

/// signers: 0x2
/// balance: 0x2 eth 100
script {
    use 0x1::Account;
    use 0x1::Coins::ETH;

    fun main(s: signer) {
        assert(Account::balance<ETH>(&s) == 100, 101);
    }
}
    "#;

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![
            stdlib_mod("coins.move"),
            stdlib_mod("event.move"),
            stdlib_mod("signer.move"),
            stdlib_mod("pontem.move"),
            stdlib_mod("account.move"),
        ],
        "pont",
        "0x2",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
}

#[test]
fn test_set_native_balance_via_meta() {
    let text = r#"
script {
    use 0x1::PONT;
    use 0x1::Pontem;

    fun register_coins() {
        Pontem::register_coin<PONT::T>(b"pont", 18);
    }
}

/// signers: 0x2
/// native_balance: 0x2 pont 100
script {
    use 0x1::PONT;
    use 0x1::Pontem;

    fun main(s: signer) {
        assert(Pontem::get_native_balance<PONT::T>(&s) == 100, 101)
    }
}
    "#;

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![
            stdlib_mod("pont.move"),
            stdlib_mod("event.move"),
            stdlib_mod("signer.move"),
            stdlib_mod("pontem.move"),
        ],
        "pont",
        "0x2",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
}

#[test]
fn test_fail_with_arithmetic_error() {
    let text = r"
/// status: ARITHMETIC_ERROR
script {
    fun main() {
        1 / 0;
    }
}
    ";

    let error_string = execute_script(
        MoveFile::with_content(script_path(), text),
        vec![],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .expected_error();
    assert_eq!(
        error_string,
        "Expected error: Execution failed with an arithmetic error (i.e., integer overflow/underflow, div/mod by zero, or invalid shift) in script at code offset 2"
    );
}

#[test]
fn test_set_block_height() {
    let text = r"
/// block: 1024
script {
    use 0x1::Block;

    fun success() {
        assert(Block::get_current_block_height() == 1024, 1);
    }
}    ";

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("block.move")],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
}

#[test]
fn test_block_height_default_100() {
    let text = r"
script {
    use 0x1::Block;

    fun success() {
        assert(Block::get_current_block_height() == 100, 1);
    }
}    ";

    execute_script(
        MoveFile::with_content(script_path(), text),
        vec![stdlib_mod("block.move")],
        "diem",
        "0x3",
        vec![],
    )
    .unwrap()
    .last()
    .unwrap()
    .effects();
}
