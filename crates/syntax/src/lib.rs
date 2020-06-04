use tree_sitter::{Language, Parser};

pub mod ast;

#[link(name = "tree-sitter-move")]
extern "C" {
    fn tree_sitter_move() -> Language;
}

pub fn parser() -> Parser {
    let language = unsafe { tree_sitter_move() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    parser
}
