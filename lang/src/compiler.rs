use anyhow::Result;
use move_core_types::gas_schedule::CostTable;
use move_core_types::parser::parse_transaction_argument;
use move_lang::parser::ast::Definition;
use move_lang::parser::syntax;
use move_lang::{strip_comments_and_verify, FileCommentMap};

use utils::{FilesSourceText, MoveFile};
use dialects::base::Dialect;
use dialects::shared::{ProvidedAccountAddress, line_endings};
use dialects::shared::errors::{FileSourceMap, ExecCompilerError, ProjectSourceMap, CompilerError};
use dialects::lang::{into_exec_compiler_error, PreBytecodeProgram, ProgramCommentsMap, check_defs, replace_sender_placeholder};
use dialects::lang;

pub fn parse_file(
    dialect: &dyn Dialect,
    file: MoveFile,
    sender: &ProvidedAccountAddress,
) -> Result<(Vec<Definition>, String, FileSourceMap, FileCommentMap), ExecCompilerError> {
    let ((fname, source_text), file_source_map) = normalize_source_text(dialect, file, sender);

    let (stripped_source_text, comment_map) = strip_comments_and_verify(fname, &source_text)
        .map_err(|errors| {
            into_exec_compiler_error(
                errors,
                ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
            )
        })?;
    let (defs, _) =
        syntax::parse_file_string(fname, &stripped_source_text, FileCommentMap::default())
            .map_err(|errors| {
                into_exec_compiler_error(
                    errors,
                    ProjectSourceMap::with_file_map(fname, file_source_map.clone()),
                )
            })?;
    Ok((defs, source_text, file_source_map, comment_map))
}

pub fn parse_files(
    dialect: &dyn Dialect,
    current_file: MoveFile,
    deps: &[MoveFile],
    sender: &ProvidedAccountAddress,
) -> Result<
    (
        Vec<Definition>,
        Vec<Definition>,
        ProjectSourceMap,
        ProgramCommentsMap,
    ),
    ExecCompilerError,
> {
    let mut exec_compiler_error = ExecCompilerError::default();

    let mut project_offsets_map = ProjectSourceMap::default();
    let mut comment_map = ProgramCommentsMap::new();

    let script_defs = match parse_file(dialect, current_file.clone(), &sender) {
        Ok((defs, normalized_source_text, offsets_map, comments)) => {
            project_offsets_map.0.insert(current_file.0, offsets_map);
            comment_map.insert(current_file.0, (normalized_source_text, comments));
            defs
        }
        Err(error) => {
            exec_compiler_error.extend(error);
            vec![]
        }
    };

    let mut dep_defs = vec![];
    for dep_file in deps.iter() {
        let defs = match parse_file(dialect, dep_file.clone(), &sender) {
            Ok((defs, normalized_source_text, offsets_map, file_comment_map)) => {
                project_offsets_map.0.insert(dep_file.0, offsets_map);
                comment_map.insert(dep_file.0, (normalized_source_text, file_comment_map));
                defs
            }
            Err(error) => {
                exec_compiler_error.extend(error);
                vec![]
            }
        };
        dep_defs.extend(defs);
    }
    if !exec_compiler_error.0.is_empty() {
        return Err(exec_compiler_error);
    }
    Ok((script_defs, dep_defs, project_offsets_map, comment_map))
}

pub fn check_with_compiler(
    dialect: &dyn Dialect,
    current: MoveFile,
    deps: Vec<MoveFile>,
    sender: &ProvidedAccountAddress,
) -> Result<(), Vec<CompilerError>> {
    let (script_defs, dep_defs, offsets_map, _) =
        parse_files(dialect, current, &deps, sender)
            .map_err(|errors| errors.transform_with_source_map())?;

    match check_defs(script_defs, dep_defs, sender.as_address()) {
        Ok(_) => Ok(()),
        Err(errors) => {
            Err(into_exec_compiler_error(errors, offsets_map).transform_with_source_map())
        }
    }
}

pub fn compile_to_prebytecode_program(
    dialect: &dyn Dialect,
    script: MoveFile,
    deps: &[MoveFile],
    sender: ProvidedAccountAddress,
) -> Result<(PreBytecodeProgram, ProgramCommentsMap, ProjectSourceMap), ExecCompilerError>
{
    let (mut file_defs, dep_defs, project_offsets_map, comments) =
        parse_files(dialect, script, deps, &sender)?;
    file_defs.extend(dep_defs);

    let program = check_defs(file_defs, vec![], sender.as_address())
        .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
    Ok((program, comments, project_offsets_map))
}

pub fn print_compiler_errors_and_exit(
    files: FilesSourceText,
    errors: Vec<CompilerError>,
) -> ! {
    lang::report_errors(files, errors)
}

fn normalize_source_text(
    dialect: &dyn Dialect,
    file: MoveFile,
    sender: &ProvidedAccountAddress,
) -> (MoveFile, FileSourceMap) {
    let (fname, source_text) = file;
    let (mut source_text, mut file_source_map) = line_endings::normalize(source_text);
    source_text = replace_sender_placeholder(
        source_text,
        &sender.normalized_original,
        &mut file_source_map,
    );
    source_text = dialect.replace_addresses(&source_text, &mut file_source_map);
    ((fname, source_text), file_source_map)
}