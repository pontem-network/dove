fn main() {
    let resources_dir = std::env::current_dir().unwrap().join("resources");
    let move_parser_file = resources_dir.join("parser.c");

    println!("cargo:rerun-if-changed={}", resources_dir.to_str().unwrap()); // <1>

    cc::Build::new()
        .file(move_parser_file)
        .include(&resources_dir)
        .compile("tree-sitter-move");
}
