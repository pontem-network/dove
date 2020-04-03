use lsp_types::{Position, Range};
use move_lang::parser::ast::FileDefinition;

use move_language_server::compiler::check_with_compiler;
use move_language_server::compiler::parser::parse_source_file;

const FNAME: &str = "main.move";

#[test]
fn test_fail_on_non_ascii_character() {
    let source = r"fun main() { return; }ффф";
    let errors = check_with_compiler(FNAME, source).unwrap_err();
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
    let errors = check_with_compiler(FNAME, source).unwrap_err();
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
    let errors = check_with_compiler(FNAME, source).unwrap_err();
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
    let errors = check_with_compiler(FNAME, source).unwrap_err();
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

    let errors = check_with_compiler(FNAME, source_text).unwrap_err();
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

    let errors = check_with_compiler(FNAME, source_text).unwrap_err();
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

    let errors = check_with_compiler(FNAME, source_text).unwrap_err();
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

    let errors = check_with_compiler(FNAME, source_text).unwrap_err();
    assert_eq!(errors.len(), 1);

    let diagnostic = errors.get(0).unwrap();
    assert_eq!(
        diagnostic.range,
        Range::new(Position::new(2, 8), Position::new(2, 10))
    );
    assert_eq!(diagnostic.message, "Invalid borrow");
}
