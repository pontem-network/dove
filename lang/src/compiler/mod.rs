pub mod address;
pub mod dialects;
pub mod errors;
pub mod parser;
pub mod source_map;
pub mod file;

pub use move_lang::name_pool::ConstPool;

use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
use parser::parse_program;
use crate::compiler::errors::{into_exec_compiler_error, from_compiler_error, CompilerError};
use crate::compiler::dialects::Dialect;
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::file::MvFile;
use std::collections::HashMap;
use move_lang::check_program;

pub type FilesSourceText = HashMap<&'static str, String>;

pub fn compile_program(
    dialect: &dyn Dialect,
    targets: Vec<MvFile>,
    deps: Vec<MvFile>,
    sender: Option<&ProvidedAccountAddress>,
) -> (FilesSourceText, Result<Vec<CompiledUnit>, Errors>) {
    let (source_text, offsets_map, pprog_and_comments_res) =
        parse_program(dialect, targets, deps, sender);
    let pprog_res = pprog_and_comments_res
        .map(|(pprog, _comments)| pprog)
        .map_err(|errors| {
            errors
                .0
                .into_iter()
                .map(from_compiler_error)
                .collect::<Vec<_>>()
        });

    match move_lang::compile_program(pprog_res, sender.map(|addr| addr.as_address())) {
        Err(errors) => {
            let errors = into_exec_compiler_error(errors, offsets_map)
                .transform_with_source_map()
                .into_iter()
                .map(from_compiler_error)
                .collect::<Vec<_>>();
            (source_text, Err(errors))
        }
        Ok(compiled_units) => (source_text, Ok(compiled_units)),
    }
}

pub fn check(
    dialect: &dyn Dialect,
    current: MvFile,
    deps: Vec<MvFile>,
    sender: Option<&ProvidedAccountAddress>,
) -> Result<(), Vec<CompilerError>> {
    let (_, offsets_map, pprog_and_comments_res) =
        parse_program(dialect, vec![current], deps, sender.clone());

    let pprog = match pprog_and_comments_res.map(|(pprog, _)| pprog) {
        Ok(pprog) => Ok(pprog),
        Err(mut err) => {
            err.1 = offsets_map;
            return Err(err.transform_with_source_map());
        }
    };

    check_program(pprog, sender.map(|addr| addr.as_address())).map_err(|errors| {
        into_exec_compiler_error(errors, offsets_map)
            .transform_with_source_map()
    })?;
    Ok(())
}

// use std::collections::{BTreeMap, HashMap};
// use move_lang::{FileCommentMap, cfgir, parser as l_parser, check_program};
// use move_lang::parser::ast::Definition;
// use move_lang::errors::Error;
// use file::MvFile;
// use utils::{MoveFilePath};
//
// pub type ProgramCommentsMap = BTreeMap<MoveFilePath, (String, FileCommentMap)>;
// pub type PreBytecodeProgram = cfgir::ast::Program;
//
// pub fn check_defs(
//     source_definitions: Vec<Definition>,
//     lib_definitions: Vec<Definition>,
//     sender: Address,
// ) -> Result<PreBytecodeProgram, Vec<Error>> {
//     let ast_program = l_parser::ast::Program {
//         source_definitions,
//         lib_definitions,
//     };
//     move_lang::check_program(Ok(ast_program), Some(sender))
// }
