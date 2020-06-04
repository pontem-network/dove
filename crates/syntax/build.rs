fn main() {
    let project_root_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
        .parent()
        .unwrap()
        .to_owned();
    let grammar_dir = project_root_dir.join("resources").join("grammar");
    let move_parser_file = grammar_dir.join("parser.c");

    println!("cargo:rerun-if-changed={}", grammar_dir.to_str().unwrap()); // <1>

    cc::Build::new()
        .file(move_parser_file)
        .include(&grammar_dir)
        .compile("tree-sitter-move");
}
