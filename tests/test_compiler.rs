use lsp_types::{Diagnostic, Position, Range};

use move_lang::shared::Address;

use move_language_server::compiler::utils::leak_str;
use move_language_server::config::Config;
use move_language_server::test_utils::{get_modules_path, get_stdlib_path};
use move_language_server::world::WorldState;

// just need some valid fname
fn existing_file_abspath() -> &'static str {
    let abspath = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("test_compiler.rs")
        .into_os_string()
        .into_string()
        .unwrap();
    leak_str(&abspath)
}

fn range(start: (u64, u64), end: (u64, u64)) -> Range {
    Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
}

fn diagnostics(text: &str) -> Vec<Diagnostic> {
    diagnostics_with_config(text, Config::default())
}

fn diagnostics_with_config(text: &str, config: Config) -> Vec<Diagnostic> {
    diagnostics_with_config_and_filename(text, config, existing_file_abspath())
}

fn diagnostics_with_config_and_filename(
    text: &str,
    config: Config,
    fname: &'static str,
) -> Vec<Diagnostic> {
    let world_state = WorldState::new(std::env::current_dir().unwrap(), config);
    let analysis = world_state.analysis();
    analysis.check_with_libra_compiler(fname, text)
}

#[test]
fn test_fail_on_non_ascii_character() {
    let source_text = r"fun main() { return; }ффф";
    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(error.range, range((0, 22), (0, 22)));
}

#[test]
fn test_successful_compilation() {
    let source = r"
        fun main() {}
    ";
    let errors = diagnostics(source);
    assert!(errors.is_empty());
}

#[test]
fn test_function_parse_error() {
    let source_text = "module M { struc S { f: u64 } }";
    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(error.message, "Unexpected 'struc'");
    assert_eq!(error.range, range((0, 11), (0, 16)));
}

#[test]
fn test_main_function_parse_error() {
    let source_text = "main() {}";
    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(
        error.message,
        "Invalid address directive. Expected 'address' got 'main'"
    );
    assert_eq!(error.range, range((0, 0), (0, 4)));
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
    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(error.range, range((2, 4), (2, 9)));
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
    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.message,
        "Duplicate definition for field \'f\' in struct \'S\'"
    );
    assert_eq!(diagnostic.range, range((3, 8), (3, 9)));
}

#[test]
fn test_expansion_checks_public_main_redundancy() {
    let source_text = r"public fun main() {}";

    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.message,
        "Extraneous 'public' modifier. This top-level function is assumed to be public"
    );
    assert_eq!(diagnostic.range, range((0, 0), (0, 6)));
}

#[test]
fn test_naming_checks_generics_with_type_parameters() {
    let source_text = r"module M {
    struct S<T> { f: T<u64> }
}
    ";

    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.message,
        "Generic type parameters cannot take type arguments"
    );
    assert_eq!(diagnostic.range, range((1, 21), (1, 27)));
}

#[test]
fn test_typechecking_invalid_local_borrowing() {
    let source_text = r"module M {
    fun t0(r: &u64) {
        &r;
    }
}
    ";

    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(diagnostic.message, "Invalid borrow");
    assert_eq!(diagnostic.range, range((2, 8), (2, 10)));
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

    let errors = diagnostics(source_text);
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.message,
        "Unreachable code. This statement (and any following statements) will not be executed. \
        In some cases, this will result in unused resource values."
    );
    assert_eq!(diagnostic.range, range((8, 42), (8, 43)));
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
    let errors = diagnostics_with_config(source_text, config);
    assert!(errors.is_empty());
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
    let errors = diagnostics_with_config(script_source_text, config);
    assert!(errors.is_empty());
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
    let covid_tracker_module = leak_str(
        get_modules_path()
            .join("covid_tracker.move")
            .to_str()
            .unwrap(),
    );
    let errors =
        diagnostics_with_config_and_filename(module_source_text, config, covid_tracker_module);
    assert!(errors.is_empty());
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
    let errors = diagnostics_with_config(script_source_text, config);
    assert!(errors.is_empty());
}

#[test]
fn test_compiler_out_of_bounds_multimessage_diagnostic() {
    let source_text = r"
use 0x0::CovidTracker;

fun main() {
    let how_many: u8;
    how_many = CovidTracker::how_many(10);
}
    ";
    let config = Config {
        module_folders: vec![get_stdlib_path(), get_modules_path()],
        ..Config::default()
    };
    let errors = diagnostics_with_config(source_text, config);
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap().to_owned();
    assert_eq!(error.related_information.unwrap().len(), 2);
}
