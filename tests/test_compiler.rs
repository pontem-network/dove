use lsp_types::{Position, Range};
use move_lang::parser::ast::FileDefinition;

use move_language_server::compiler::parser::parse_source_file;

const FNAME: &str = "main.move";

#[test]
fn test_fail_on_non_ascii_character() {
    let source = r"fun main() { return; }ффф";
    let errors = parse_source_file(FNAME, source).unwrap_err();
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
    let source = "module M { struc S { f: u64 } }";
    let errors = parse_source_file(FNAME, source).unwrap_err();
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
    let source = "main() {}";
    let errors = parse_source_file(FNAME, source).unwrap_err();
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
    let source = r"
module M {
    struc S {
        f: u64
    }
}
";
    let errors = parse_source_file(FNAME, source).unwrap_err();
    assert_eq!(errors.len(), 1);

    let error = errors.get(0).unwrap();
    assert_eq!(
        error.range,
        Range::new(Position::new(2, 4), Position::new(2, 9))
    );
}
