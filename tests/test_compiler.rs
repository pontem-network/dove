use lsp_types::{Position, Range};
use move_lang::parser::ast::FileDefinition;
use move_lang::shared::Address;

use move_language_server::compiler::check_with_compiler;
use move_language_server::compiler::parser::parse_source_file;
use move_language_server::compiler::utils::leak_str;
use move_language_server::config::Config;
use move_language_server::test_utils::{get_modules_path, get_stdlib_path};
use move_language_server::world::WorldState;

const FNAME: &str = "main.move";

#[test]
fn test_fail_on_non_ascii_character() {
    let source_text = r"fun main() { return; }ффф";
    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(
        error.range,
        Range::new(Position::new(0, 22), Position::new(0, 22))
    );
}

#[test]
fn test_successful_parsing() {
    let source = r"
        fun main() { return; }
    ";
    let compiled = parse_source_file(FNAME, source).unwrap();
    assert!(matches!(compiled, FileDefinition::Main(_)));
}

#[test]
fn test_function_parse_error() {
    let source_text = "module M { struc S { f: u64 } }";
    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);
    let error = errors.get(0).unwrap();
    assert_eq!(
        error.range,
        Range::new(Position::new(0, 11), Position::new(0, 16))
    );
    assert_eq!(error.message, "Unexpected 'struc'");
}

#[test]
fn test_main_function_parse_error() {
    let source_text = "main() {}";
    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);
    let error = errors.get(0).unwrap();
    assert_eq!(
        error.range,
        Range::new(Position::new(0, 0), Position::new(0, 4))
    );
    assert_eq!(
        error.message,
        "Invalid address directive. Expected 'address' got 'main'"
    );
}

#[test]
fn test_multiline_function_parse_error() {
    let source_text = r"
module M {
    struc S {
        f: u64
    }
}
";
    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(
        error.range,
        Range::new(Position::new(2, 4), Position::new(2, 9))
    );
}

#[test]
fn test_expansion_checks_duplicates() {
    let source_text = r"module M {
    struct S {
        f: u64,
        f: u64,
    }
}
    ";

    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);
    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(3, 8), Position::new(3, 9))
    );
    assert_eq!(
        diagnostic.message,
        "Duplicate definition for field \'f\' in struct \'S\'"
    );
}

#[test]
fn test_expansion_checks_public_main_redundancy() {
    let source_text = r"public fun main() {}";

    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(0, 0), Position::new(0, 6))
    );
    assert_eq!(
        diagnostic.message,
        "Extraneous 'public' modifier. This top-level function is assumed to be public"
    );
}

#[test]
fn test_naming_checks_generics_with_type_parameters() {
    let source_text = r"module M {
    struct S<T> { f: T<u64> }
}
    ";

    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(1, 21), Position::new(1, 27))
    );
    assert_eq!(
        diagnostic.message,
        "Generic type parameters cannot take type arguments"
    );
}

#[test]
fn test_typechecking_invalid_local_borrowing() {
    let source_text = r"module M {
    fun t0(r: &u64) {
        &r;
    }
}
    ";

    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(2, 8), Position::new(2, 10))
    );
    assert_eq!(diagnostic.message, "Invalid borrow");
}

#[test]
fn test_check_unreachable_code_in_loop() {
    let source_text = r"module M {
    fun t() {
        let x = 0;
        let t = 1;

        if (x >= 0) {
            loop {
                let my_local = 0;
                if (my_local >= 0) { break; };
            };
            x = 1
        };
        t;
        x;
    }
}
    ";

    let world_state = WorldState::default();
    let errors = check_with_compiler(FNAME, source_text, &world_state).unwrap_err();
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(8, 42), Position::new(8, 43))
    );
    assert_eq!(diagnostic.message, "Unreachable code. This statement (and any following statements) will not be executed. In some cases, this will result in unused resource values.");
}

#[test]
fn test_stdlib_modules_are_available_if_loaded() {
    let source_text = r"
module MyModule {
    use 0x0::Transaction;

    public fun how_main(country: u8) {
        let _ = Transaction::sender();
    }
}
    ";
    let config = Config {
        module_folders: vec![get_stdlib_path()],
        ..Config::default()
    };
    let world_state = WorldState::with_modules_loaded(std::env::current_dir().unwrap(), config);
    check_with_compiler(FNAME, source_text, &world_state).unwrap();
}

#[test]
fn test_compile_check_script_with_additional_dependencies() {
    // hardcoded sender address
    let script_source_text = r"
use 0x8572f83cee01047effd6e7d0b5c19743::CovidTracker;
fun main() {
    CovidTracker::how_many(5);
}
    ";
    let config = Config {
        sender_address: Address::parse_str("0x8572f83cee01047effd6e7d0b5c19743").unwrap(),
        module_folders: vec![get_stdlib_path(), get_modules_path()],
        ..Config::default()
    };
    let world_state = WorldState::with_modules_loaded(std::env::current_dir().unwrap(), config);
    check_with_compiler(FNAME, script_source_text, &world_state).unwrap();
}

#[test]
fn test_compile_check_module_from_a_folder_with_folder_provided_as_dependencies() {
    let module_source_text = r"
module CovidTracker {
    use 0x0::Vector;
    use 0x0::Transaction;
	struct NewsReport {
		news_source_id: u64,
		infected_count: u64,
	}
	resource struct CovidSituation {
		country_id: u8,
		reports: vector<NewsReport>
	}
	public fun how_many(country: u8): u64 acquires CovidSituation {
        let case = borrow_global<CovidSituation>(Transaction::sender());
        let len  = Vector::length(&case.reports);
        let sum  = 0u64;
        let i    = 0;
        while (i < len) {
            sum = sum + Vector::borrow(&case.reports, i).infected_count;
        };
        sum
	}
}
    ";
    let config = Config {
        module_folders: vec![get_stdlib_path(), get_modules_path()],
        ..Config::default()
    };
    let world_state = WorldState::with_modules_loaded(std::env::current_dir().unwrap(), config);
    let covid_tracker_module = leak_str(
        get_modules_path()
            .join("covid_tracker.move")
            .to_str()
            .unwrap(),
    );
    check_with_compiler(covid_tracker_module, module_source_text, &world_state).unwrap();
}

#[test]
fn test_compile_with_sender_address_specified() {
    // hardcoded sender address
    let sender_address = "0x11111111111111111111111111111111";
    let script_source_text = r"
use 0x11111111111111111111111111111111::CovidTracker;
fun main() {
    CovidTracker::how_many(5);
}
    ";
    let config = Config {
        module_folders: vec![get_stdlib_path(), get_modules_path()],
        sender_address: Address::parse_str(sender_address).unwrap(),
        ..Config::default()
    };
    let world_state = WorldState::with_modules_loaded(std::env::current_dir().unwrap(), config);
    check_with_compiler(FNAME, script_source_text, &world_state).unwrap();
}
