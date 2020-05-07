//
// pub fn parse_file(fname: FilePath, text: &str) -> Result<Vec<Definition>, Errors> {
//     let (no_comments_source, comment_map) = strip_comments_and_verify(fname, text)?;
//     let res = libra_parser::syntax::parse_file_string(fname, &no_comments_source, comment_map)?;
//     Ok(res.0)
// }

// pub fn check_parsed_program(
//     current_file_defs: Vec<Definition>,
//     dependencies: Vec<Definition>,
//     sender_opt: Address,
// ) -> Result<(), Errors> {
//     let ast_program = libra_parser::ast::Program {
//         source_definitions: current_file_defs,
//         lib_definitions: dependencies,
//     };
//     move_lang::check_program(Ok(ast_program), Some(sender_opt))?;
//     Ok(())
// }
