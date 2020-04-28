use move_lang::errors::Error;
use move_lang::parser::ast::FileDefinition;

use analysis::db::FilePath;
use analysis::libra_parser::syntax;
use analysis::utils::io::leaked_fpath;

// just need some valid fname
fn existing_file_abspath() -> FilePath {
    let abspath = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("test_compiler.rs")
        .into_os_string()
        .into_string()
        .unwrap();
    leaked_fpath(&abspath)
}

fn parse_text(text: &str) -> Result<FileDefinition, Error> {
    syntax::parse_file_string(existing_file_abspath(), text)
}

#[test]
fn test_if_use_stmt_is_incomplete_show_error_and_skip() {
    let text = "use; use 0x0::Transaction; fun main() {}";
    let file = parse_text(text).unwrap();
    let main = match file {
        FileDefinition::Main(main) => main,
        FileDefinition::Modules(_) => unreachable!(),
    };
    let uses = main.uses;
    assert_eq!(uses.len(), 1);
    assert_eq!(
        (uses[0].0).0.value.name.to_string(),
        "Transaction".to_string()
    );
}
