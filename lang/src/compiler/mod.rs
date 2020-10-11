pub mod address;
pub mod dialects;
pub mod errors;
pub mod parser;
pub mod source_map;

use move_lang::shared::Address;
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
use parser::parse_program;
use crate::compiler::errors::{into_exec_compiler_error, from_compiler_error, CompilerError};
use crate::compiler::dialects::Dialect;
use crate::compiler::address::ProvidedAccountAddress;

pub type FilesSourceText = HashMap<&'static str, String>;

pub fn compile_program(
    dialect: &dyn Dialect,
    targets: Vec<MvFile>,
    deps: Vec<MvFile>,
    sender: Option<Address>,
) -> anyhow::Result<(FilesSourceText, Result<Vec<CompiledUnit>, Errors>)> {
    let provided_sender = sender.map(ProvidedAccountAddress::from);
    let (source_text, offsets_map, pprog_and_comments_res) =
        parse_program(dialect, targets, deps, &provided_sender)?;
    let pprog_res = pprog_and_comments_res
        .map(|(pprog, comments)| pprog)
        .map_err(|errors| {
            errors
                .0
                .into_iter()
                .map(from_compiler_error)
                .collect::<Vec<_>>()
        });

    match move_lang::compile_program(pprog_res, sender) {
        Err(errors) => {
            let errors = into_exec_compiler_error(errors, offsets_map)
                .transform_with_source_map()
                .into_iter()
                .map(from_compiler_error)
                .collect::<Vec<_>>();
            Ok((source_text, Err(errors)))
        }
        Ok(compiled_units) => Ok((source_text, Ok(compiled_units))),
    }
}

pub fn check(
    dialect: &dyn Dialect,
    current: MvFile,
    deps: Vec<MvFile>,
    sender: Option<Address>,
) -> Result<(), Vec<CompilerError>> {
    // let (script_defs, dep_defs, offsets_map, _) = parse_files_To_remove(dialect, current, &deps, sender)
    //     .map_err(|errors| errors.transform_with_source_map())?;
    //
    // match check_defs(script_defs, dep_defs, sender.as_address()) {
    //     Ok(_) => Ok(()),
    //     Err(errors) => {
    //         Err(into_exec_compiler_error(errors, offsets_map).transform_with_source_map())
    //     }
    // }
    todo!()
}

use std::collections::{BTreeMap, HashMap};
use move_lang::{FileCommentMap, cfgir, parser as l_parser};
use move_lang::parser::ast::Definition;
use move_lang::errors::Error;
use crate::file::MvFile;
use utils::{MoveFilePath};

pub type ProgramCommentsMap = BTreeMap<MoveFilePath, (String, FileCommentMap)>;
pub type PreBytecodeProgram = cfgir::ast::Program;

pub fn check_defs(
    source_definitions: Vec<Definition>,
    lib_definitions: Vec<Definition>,
    sender: Address,
) -> Result<PreBytecodeProgram, Vec<Error>> {
    let ast_program = l_parser::ast::Program {
        source_definitions,
        lib_definitions,
    };
    move_lang::check_program(Ok(ast_program), Some(sender))
}
