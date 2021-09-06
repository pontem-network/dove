use crossbeam_channel::unbounded;

use lsp_types::{Diagnostic, Position, Range};
use move_language_server::main_loop::{compute_file_diagnostics, FileSystemEvent, ResponseEvent};
use move_language_server::inner::config::Config;
use move_language_server::inner::db::FileDiagnostic;
use resources::{modules_path, stdlib_path};
use move_language_server::global_state::{
    GlobalState, GlobalStateSnapshot, initialize_new_global_state,
};
use lang::compiler::file::*;
use move_language_server::inner::change::AnalysisChange;
use tempfile::NamedTempFile;
use std::io::Write;
use std::path::PathBuf;

macro_rules! config {
    () => {{
        move_language_server::inner::config::Config::default()
    }};
    ($json: tt) => {{
        let config_json = serde_json::json!($json);
        let mut config = move_language_server::inner::config::Config::default();
        config.update(&config_json);
        config
    }};
}

fn range(start: (u32, u32), end: (u32, u32)) -> Range {
    Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
}

fn diagnostics(source: &str) -> Vec<Diagnostic> {
    diagnostics_with_config(source, Config::default())
}

fn diagnostics_with_config(source: &str, config: Config) -> Vec<Diagnostic> {
    let loc_ds = diagnostics_with_config_and_filename(source, config);
    loc_ds.into_iter().filter_map(|d| d.diagnostic).collect()
}

fn diagnostics_with_config_and_filename(source: &str, config: Config) -> Vec<FileDiagnostic> {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(source.as_bytes()).unwrap();
    let file_path_string = temp_file.path().to_string_lossy().to_string();

    let state_snapshot = global_state_snapshot(file_path_string.clone(), config, vec![]);
    let (task_sender, task_receiver) = unbounded::<ResponseEvent>();
    compute_file_diagnostics(state_snapshot.analysis, task_sender, vec![file_path_string]);

    let task = task_receiver.try_recv().unwrap();
    let mut ds = match task {
        ResponseEvent::Diagnostic(ds) => ds,
        _ => panic!(),
    };
    let empty = ds.remove(0);
    assert!(empty.diagnostic.is_none());
    ds
}

fn diagnostics_with_deps(
    script_file: PathBuf,
    deps: Vec<PathBuf>,
    config: Config,
) -> Option<FileDiagnostic> {
    let mut config = config;
    config.update(&serde_json::json!({ "modules_folders": [modules_path()] }));

    let mut fs_events: Vec<_> = deps.into_iter().map(FileSystemEvent::AddFile).collect();
    fs_events.push(FileSystemEvent::AddFile(script_file.clone()));

    let global_state = GlobalState::new(config, fs_events);
    global_state
        .analysis()
        .check_file(script_file.to_string_lossy().to_string())
}

pub fn global_state_snapshot(
    file: String,
    config: Config,
    additional_files: Vec<String>,
) -> GlobalStateSnapshot {
    let mut global_state = initialize_new_global_state(config);
    let mut change = AnalysisChange::new();

    for folder in &global_state.config().modules_folders {
        for fpath in find_move_files(&[folder]) {
            change.add_file(fpath.to_string_lossy().to_string());
        }
    }

    for file in additional_files {
        change.add_file(file);
    }

    change.update_file(file);

    global_state.apply_change(change);
    global_state.snapshot()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fail_on_non_ascii_character() {
        let source = r"fun main() { return; }ффф";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].range, range((0, 22), (0, 22)));
    }

    #[test]
    fn test_successful_compilation() {
        let source = r"
script {
    fun main() {}
}
";
        let errors = diagnostics(source);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_function_parse_error() {
        let source = "module M { struc S { f: u64 } }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].message, "Unexpected 'struc'");
        assert_eq!(errors[0].range, range((0, 11), (0, 16)));
    }

    #[test]
    fn test_main_function_parse_error() {
        let source = "script { main() {} }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unexpected 'main'");
    }

    #[test]
    fn test_multiline_function_parse_error() {
        let source = r"
module M {
    struc S {
        f: u64
    }
}
";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unexpected \'struc\'");
    }

    #[test]
    fn test_expansion_checks_duplicates() {
        let source = r"
module 0x1::M {
    struct S {
        f: u64,
        f: u64,
    }
}
";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message,
            "Duplicate definition for field \'f\' in struct \'S\'"
        );
    }

    #[test]
    fn test_expansion_checks_public_main_redundancy() {
        let source = r"script { public fun main() {} }";

        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message,
            "Extraneous 'public' modifier. Script functions are always 'public(script)'"
        );
    }

    #[test]
    fn test_naming_checks_generics_with_type_parameters() {
        let source = r"
module 0x1::M {
    struct S<T> { f: T<u64> }
}
";

        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message,
            "Generic type parameters cannot take type arguments"
        );
    }

    #[test]
    fn test_typechecking_invalid_local_borrowing() {
        let source = r"
module 0x1::M {
    fun t0(r: &u64) {
        &r;
    }
}
";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Invalid borrow");
    }

    #[test]
    fn test_stdlib_modules_are_available_if_loaded() {
        let source = r"
module 0x1::MyModule {
    use 0x1::Signer;

    public fun how_main(s: &signer) {
        let _ = Signer::address_of(s);
    }
}
";
        let errors = diagnostics_with_config(source, config!({ "stdlib_folder": stdlib_path() }));
        assert!(errors.is_empty());
    }

    #[test]
    fn test_compile_check_script_with_additional_dependencies() {
        // hardcoded sender address
        let source = r"
script {
    use 0x1::Signer;
    use 0x2::Record;

    fun main(s: signer) {
        let signer_address = Signer::address_of(&s);
        let record = Record::get_record(signer_address);
        Record::save(&s, record);
    }
}
";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x8572f83cee01047effd6e7d0b5c19743",
            "stdlib_folder": stdlib_path(),
            "modules_folders": [modules_path()],
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:#?}", errors);
    }

    #[test]
    fn test_compile_check_module_from_a_folder_with_folder_provided_as_dependencies() {
        let record = include_str!("../../resources/assets/modules/record.move");
        let config = config!({
            "stdlib_folder": stdlib_path(),
            "modules_folders": [modules_path()],
        });

        let errors = diagnostics_with_config_and_filename(record, config);
        assert!(errors.is_empty(), "{:#?}", errors);
    }

    #[test]
    fn test_compile_with_sender_address_specified() {
        // hardcoded sender address
        let source = r"
script {
    use 0x1::Signer;
    use 0x2::Record;

    fun main(s: signer) {
        let signer_address = Signer::address_of(&s);
        let record = Record::get_record(signer_address);
        Record::save(&s, record);
    }
}
";
        let config = config!({
            "dialect": "diem",
            "stdlib_folder": stdlib_path(),
            "modules_folders": [modules_path()],
            "sender_address": "0x1",
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:#?}", errors);
    }

    #[test]
    fn test_compiler_out_of_bounds_multimessage_diagnostic() {
        let source = r"
script {
    use 0x1::Signer;
    use 0x2::Record;

    fun main(s: signer) {
        let signer_address = Signer::address_of(s);
        let record: u8;
        record = Record::get_record(signer_address);
    }
}
";
        let config = config!({
            "stdlib_folder": stdlib_path(),
            "modules_folders": [modules_path()]
        });
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].related_information.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_check_one_of_the_stdlib_modules_no_duplicate_definition() {
        let source = r"
address 0x0 {
    module Debug {
        native public fun print<T>(x: &T);

        native public fun print_stack_trace();
    }
}
";
        let config = config!({
            "stdlib_folder": stdlib_path(),
        });
        let errors = diagnostics_with_config_and_filename(source, config);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn invalid_valid_in_precense_of_bech32_address() {
        let source = r"
address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
    module Debug {
        pubic fun main() {}
    }
}
 ";
        let errors = diagnostics_with_config(source, config!({"dialect": "dfinance"}));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unexpected \'pubic\'");
        assert_eq!(errors[0].range, range((3, 8), (3, 13)))
    }

    #[test]
    fn two_bech32_addresses_one_in_the_middle_of_script() {
        let source = r"
address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
    module Debug {
        public fun main() {
            let _ = @wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh;
        }
    }
}
";
        let errors = diagnostics_with_config(source, config!({"dialect": "dfinance"}));
        assert!(errors.is_empty(), "{:?}", errors);

        let source = r"
address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
    module Debug {
        public fun main() {
            let addr = @wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh;
        }
    }
}
        ";
        let errors = diagnostics_with_config(source, config!({"dialect": "dfinance"}));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unused assignment or binding for local 'addr'. Consider removing, replacing with '_', or prefixing with '_' (e.g., '_addr')");
        assert_eq!(errors[0].range, range((4, 16), (4, 20)));

        let source = r"
address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
    module Debug {
        public fun main() {
            let _ = @wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh;
            let _ = @wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh;
            let _: u10;
        }
    }
}
 ";
        let errors = diagnostics_with_config(source, config!({"dialect": "dfinance"}));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unbound type 'u10' in current scope");
        assert_eq!(errors[0].range, range((6, 19), (6, 22)));
    }

    #[test]
    fn pass_bech32_address_as_sender() {
        let source = r"
        address wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh {
            module Debug {
                public fun main() {}
            }
        }
        ";
        let config = config!({
            "dialect": "dfinance",
            "sender_address": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_substitude_sender_as_template_syntax() {
        let source = r"
        address {{sender}} {
            module Debug {
                public fun main() {
                    let _ = {{sender}};
                }
            }
        }";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1111111111111111"
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_substitude_sender_as_template_syntax_with_spaces() {
        let source = r"
        address {{ sender }} {
            module Debug {
                public fun main() {
                    let _ = {{ sender }};
                }
            }
        }";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1111111111111111"
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_sender_substitution_with_errors() {
        let source = r"
address {{sender}} {
    module Debug {
        public fun debug() {
            let _ = 0x0::Unknown::unknown();
        }
    }
}";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1111111111111111"
        });
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors[0].message, "Unbound module \'0x0::Unknown\'");
        assert_eq!(errors[0].range, range((4, 20), (4, 32)));
    }

    #[test]
    fn test_multiple_substitutions_with_sender() {
        let source = r"
address {{sender}} {
    module Debug {
        public fun debug() {
            let _ = {{sender}};
            let _ = 0x0::Unknown::unknown();
        }
    }
}";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1111111111111111"
        });
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors[0].message, "Unbound module \'0x0::Unknown\'");
        assert_eq!(errors[0].range, range((5, 20), (5, 32)));
    }

    #[test]
    fn test_bech32_and_sender_substitution_with_errors() {
        let source = "
address {{ sender }} {
    module Debug {
        public fun main() {
            let _ = @wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh;
            let _ = @{{ sender }};
            // errors out
            0x0::Unknown::unknown();
        }
    }
}";
        let config = config!({
            "dialect": "dfinance",
            "sender_address": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"
        });
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Unbound module \'0x0::Unknown\'");
        assert_eq!(errors[0].range, range((7, 12), (7, 24)));
    }

    #[test]
    fn test_replace_with_longer_form_if_sender_shorter_than_template_string() {
        let source = r"
address {{sender}} {
    module Debug {
        public fun main() {}
    }
}";
        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1"
        });
        let errors = diagnostics_with_config(source, config);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_sender_replacement_in_script() {
        let source = r"
script {
    fun main() {
        let _ = {{sender}}::Debug1::debug();
    }
}
        ";
        let mut script = NamedTempFile::new().unwrap();
        script.write_all(source.as_bytes()).unwrap();

        let config = config!({
            "dialect": "diem",
            "sender_address": "0x1",
        });

        let error = diagnostics_with_deps(
            script.path().to_path_buf(),
            vec![modules_path().join("debug.move")],
            config,
        );
        assert!(error.is_none(), "{:#?}", error);
    }

    #[test]
    fn test_error_message_for_unbound_module_with_bech32_address() {
        let source = r"
        script {
            fun main() {
                let _ = wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::Unknown::unknown();
            }
        }
        ";
        let config = config!({"dialect": "dfinance"});
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message,
            "Unbound module \'wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::Unknown\'"
        )
    }

    #[test]
    fn test_error_message_unbound_module_with_bech32_address_and_sender() {
        let source = r"
script {
    fun main() {
        let _ = {{sender}}::Unknown::unknown();
    }
}
        ";
        let config = config!({
            "dialect": "dfinance",
            "sender_address": "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"
        });
        let errors = diagnostics_with_config(source, config);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].range, range((3, 16), (3, 35)));
        assert_eq!(
            errors[0].message,
            "Unbound module \'0xDE5F86CE8AD7944F272D693CB4625A955B610150::Unknown\'"
        )
    }

    #[test]
    fn test_dfinance_documentation_issue_should_not_crash_with_span_overflow() {
        let source = r"
        address 0x0 {
        /// docs
        module DFI {
            struct T {}
        }
        }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_when_module_resolution_fails_error_should_be_at_use_site() {
        let source = r"script {
            use 0x0::UnknownPayments;
            fun main(s: &signer) {
                UnknownPayments::send_payment_event();
            }
        }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message,
            "Invalid \'use\'. Unbound module: \'0x0::UnknownPayments\'"
        );
    }

    #[test]
    fn test_windows_line_endings_are_allowed() {
        let source = "script { fun main() {} \r\n } \r\n";
        let errors = diagnostics(source);
        assert!(errors.is_empty(), "{:#?}", errors);
    }

    #[test]
    fn test_windows_line_endings_do_not_offset_errors() {
        let source = "script {\r\n func main() {} \r\n }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].range, range((1, 1), (1, 5)));

        let source = "script {\r\n\r\n\r\n func main() {} \r\n }";
        let errors = diagnostics(source);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].range, range((3, 1), (3, 5)));
    }
}
