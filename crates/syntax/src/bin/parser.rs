use syntax::ast::{Definition, SourceFile};
use tree_sitter::{Language, Parser};

#[link(name = "tree-sitter-move")]
extern "C" {
    fn tree_sitter_move() -> Language;
}

fn main() {
    let language = unsafe { tree_sitter_move() };
    let mut parser = Parser::new();
    if let Err(err_message) = parser.set_language(language) {
        dbg!(err_message);
        std::process::exit(1);
    }

    let source_code = "module Module { public fun main() { func(&mut 1); } }";
    let tree = parser.parse(source_code, None).unwrap();
    println!("{:?}", tree.root_node().to_sexp());

    let file = SourceFile::new(source_code, tree.root_node());
    if let Some(Definition::Module(module)) = file.definition() {
        let items = module.body().unwrap();
        dbg!(&items[0]);
    }
}
