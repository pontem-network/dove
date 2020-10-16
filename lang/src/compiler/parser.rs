use anyhow::Result;
use move_lang::{
    strip_comments_and_verify, FileCommentMap, parser, CommentMap, MatchedFileCommentMap,
};

use move_lang::name_pool::ConstPool;
use std::collections::{HashMap, BTreeMap};
use move_lang::parser::syntax::parse_file_string;
use crate::compiler::source_map::{FileSourceMap, ProjectSourceMap, len_difference};
use crate::compiler::errors::{ExecCompilerError, into_exec_compiler_error};
use crate::compiler::dialects::{Dialect, line_endings};
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::file::MvFile;
use move_lang::errors::FilesSourceText;
use utils::MoveFilePath;

pub type ProgramCommentsMap = BTreeMap<MoveFilePath, (String, FileCommentMap)>;

pub fn parse_program(
    dialect: &dyn Dialect,
    targets: Vec<MvFile>,
    deps: Vec<MvFile>,
    sender: Option<&ProvidedAccountAddress>,
) -> (
    FilesSourceText,
    ProjectSourceMap,
    Result<(parser::ast::Program, CommentMap), ExecCompilerError>,
) {
    let mut files: FilesSourceText = HashMap::new();
    let mut source_definitions = Vec::new();
    let mut source_comments = CommentMap::new();
    let mut lib_definitions = Vec::new();
    let mut project_offsets_map = ProjectSourceMap::default();
    let mut exec_compiler_error = ExecCompilerError::default();

    for target in targets {
        let (name, content) = target.into();
        let name = ConstPool::push(&name);
        let (defs, comments, es, offsets_map) =
            parse_file(dialect, &mut files, name, content, sender);
        source_definitions.extend(defs);
        source_comments.insert(name, comments);
        project_offsets_map.0.insert(name, offsets_map);

        exec_compiler_error.extend(es);
    }

    for dep in deps {
        let (name, content) = dep.into();
        let name = ConstPool::push(&name);
        let (defs, _, es, offsets_map) = parse_file(dialect, &mut files, name, content, sender);
        project_offsets_map.0.insert(name, offsets_map);
        lib_definitions.extend(defs);

        exec_compiler_error.extend(es);
    }

    let res = if exec_compiler_error.0.is_empty() {
        Ok((
            parser::ast::Program {
                source_definitions,
                lib_definitions,
            },
            source_comments,
        ))
    } else {
        Err(exec_compiler_error)
    };

    (files, project_offsets_map, res)
}

fn parse_file(
    dialect: &dyn Dialect,
    files: &mut FilesSourceText,
    fname: &'static str,
    source_buffer: String,
    sender: Option<&ProvidedAccountAddress>,
) -> (
    Vec<parser::ast::Definition>,
    MatchedFileCommentMap,
    ExecCompilerError,
    FileSourceMap,
) {
    let (source_buffer, file_source_map) = normalize_source_text(dialect, source_buffer, sender);
    let (no_comments_buffer, comment_map) = match strip_comments_and_verify(fname, &source_buffer)
    {
        Err(errs) => {
            files.insert(fname, source_buffer);
            let errors = into_exec_compiler_error(
                errs,
                ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
            );
            return (
                vec![],
                MatchedFileCommentMap::new(),
                errors,
                file_source_map,
            );
        }
        Ok(result) => result,
    };

    files.insert(fname, source_buffer);
    match parse_file_string(fname, &no_comments_buffer, comment_map) {
        Ok((defs, comments)) => (defs, comments, ExecCompilerError::empty(), file_source_map),
        Err(errs) => {
            let errors = into_exec_compiler_error(
                errs,
                ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
            );

            (
                vec![],
                MatchedFileCommentMap::new(),
                errors,
                file_source_map,
            )
        }
    }
}

//to remove
// fn parse_file(
//     dialect: &dyn Dialect,
//     file: MoveFile,
//     sender: &ProvidedAccountAddress,
// ) -> Result<(Vec<Definition>, String, FileSourceMap, FileCommentMap), ExecCompilerError> {
//     let ((fname, source_text), file_source_map) = normalize_source_text(dialect, file, sender);
//
//     let (stripped_source_text, comment_map) = strip_comments_and_verify(fname, &source_text)
//         .map_err(|errors| {
//             into_exec_compiler_error(
//                 errors,
//                 ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
//             )
//         })?;
//     let (defs, _) =
//         syntax::parse_file_string(fname, &stripped_source_text, FileCommentMap::default())
//             .map_err(|errors| {
//                 into_exec_compiler_error(
//                     errors,
//                     ProjectSourceMap::with_file_map(fname, file_source_map.clone()),
//                 )
//             })?;
//     Ok((defs, source_text, file_source_map, comment_map))
// }

//to remove
// pub fn parse_files_To_remove(
//     dialect: &dyn Dialect,
//     current_file: MoveFile,
//     deps: &[MoveFile],
//     sender: &ProvidedAccountAddress,
// ) -> Result<
//     (
//         Vec<Definition>,
//         Vec<Definition>,
//         ProjectSourceMap,
//         ProgramCommentsMap,
//     ),
//     ExecCompilerError,
// > {
//     let mut exec_compiler_error = ExecCompilerError::default();
//
//     let mut project_offsets_map = ProjectSourceMap::default();
//     let mut comment_map = ProgramCommentsMap::new();
//
//     let script_defs = match parse_file(dialect, current_file.clone(), &sender) {
//         Ok((defs, normalized_source_text, offsets_map, comments)) => {
//             project_offsets_map.0.insert(current_file.0, offsets_map);
//             comment_map.insert(current_file.0, (normalized_source_text, comments));
//             defs
//         }
//         Err(error) => {
//             exec_compiler_error.extend(error);
//             vec![]
//         }
//     };
//
//     let mut dep_defs = vec![];
//     for dep_file in deps.iter() {
//         let defs = match parse_file(dialect, dep_file.clone(), &sender) {
//             Ok((defs, normalized_source_text, offsets_map, file_comment_map)) => {
//                 project_offsets_map.0.insert(dep_file.0, offsets_map);
//                 comment_map.insert(dep_file.0, (normalized_source_text, file_comment_map));
//                 defs
//             }
//             Err(error) => {
//                 exec_compiler_error.extend(error);
//                 vec![]
//             }
//         };
//         dep_defs.extend(defs);
//     }
//     if !exec_compiler_error.0.is_empty() {
//         return Err(exec_compiler_error);
//     }
//     Ok((script_defs, dep_defs, project_offsets_map, comment_map))
// }

//to remove
// pub fn compile_to_prebytecode_program(
//     dialect: &dyn Dialect,
//     script: MoveFile,
//     deps: &[MoveFile],
//     sender: ProvidedAccountAddress,
// ) -> Result<(PreBytecodeProgram, ProgramCommentsMap, ProjectSourceMap), ExecCompilerError> {
//     // let (mut file_defs, dep_defs, project_offsets_map, comments) =
//     //     parse_files_To_remove(dialect, script, deps, &sender)?;
//     // file_defs.extend(dep_defs);
//     //
//     // let program = check_defs(file_defs, vec![], sender.as_address())
//     //     .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
//     // Ok((program, comments, project_offsets_map))
//     todo!()
// }

fn normalize_source_text(
    dialect: &dyn Dialect,
    source_text: String,
    sender: Option<&ProvidedAccountAddress>,
) -> (String, FileSourceMap) {
    let (mut source_text, mut file_source_map) = line_endings::normalize(source_text);
    if let Some(sender) = sender {
        source_text = replace_sender_placeholder(
            source_text,
            &sender.normalized_original,
            &mut file_source_map,
        );
    }
    source_text = dialect.replace_addresses(&source_text, &mut file_source_map);
    (source_text, file_source_map)
}

/// replace {{sender}} and {{ sender }} inside source code
fn replace_sender_placeholder(
    s: String,
    sender: &str,
    file_source_map: &mut FileSourceMap,
) -> String {
    assert!(
        sender.len() > 12,
        "Sender address length is too short: {}",
        sender.len()
    );
    let mut new_s = s;
    for template in &["{{sender}}", "{{ sender }}"] {
        while let Some(pos) = new_s.find(template) {
            new_s.replace_range(pos..pos + template.len(), sender);
            file_source_map.insert_layer(pos + sender.len(), len_difference(template, sender));
        }
    }
    new_s
}
