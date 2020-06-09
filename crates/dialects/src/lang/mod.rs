use anyhow::Result;
use codespan::ByteIndex;

use move_ir_types::location::Loc;
use move_lang::{
    cfgir,
    errors::{Error, FilesSourceText},
    parser,
    parser::ast::Definition,
    shared::Address,
};

use crate::shared::errors::{
    len_difference, CompilerError, CompilerErrorPart, ExecCompilerError, FileSourceMap, Location,
    ProjectSourceMap,
};

pub mod data_cache;
pub mod executor;
pub mod gas;
pub mod resources;

fn from_compiler_error(comp_error: CompilerError) -> Error {
    comp_error
        .parts
        .into_iter()
        .map(|part| {
            let CompilerErrorPart {
                location: Location { fpath, span },
                message,
            } = part;
            (
                Loc::new(
                    fpath,
                    codespan::Span::new(ByteIndex(span.0 as u32), ByteIndex(span.1 as u32)),
                ),
                message,
            )
        })
        .collect()
}

pub fn report_errors(files: FilesSourceText, errors: Vec<CompilerError>) -> ! {
    let errors = errors.into_iter().map(from_compiler_error).collect();
    move_lang::errors::report_errors(files, errors)
}

fn into_compiler_error(error: Error) -> CompilerError {
    let mut parts = vec![];
    for (loc, message) in error {
        let part = CompilerErrorPart {
            location: Location {
                fpath: loc.file(),
                span: (loc.span().start().to_usize(), loc.span().end().to_usize()),
            },
            message,
        };
        parts.push(part);
    }
    CompilerError { parts }
}

pub fn into_exec_compiler_error(
    errors: Vec<Error>,
    offsets_map: ProjectSourceMap,
) -> ExecCompilerError {
    let mut compiler_errors = vec![];
    for error in errors {
        compiler_errors.push(into_compiler_error(error));
    }
    ExecCompilerError(compiler_errors, offsets_map)
}

// fn is_inside_libra_directory() -> bool {
//     let path = PathBuf::from(file!());
//     path.parent().unwrap().to_str().unwrap().ends_with("libra")
// }

/// replace {{sender}} and {{ sender }} inside source code
pub fn replace_sender_placeholder(
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

// fn parse_file<F>(
//     fname: MoveFilePath,
//     source_text: &str,
//     sender: &ProvidedAccountAddress,
//     replace_addresses: F,
// ) -> Result<(Vec<Definition>, FileSourceMap), ExecCompilerError>
// where
//     F: Fn(&str, FileSourceMap) -> String,
// {
//     let (mut source_text, comment_map) =
//         strip_comments_and_verify(fname, source_text).map_err(|errors| {
//             into_exec_compiler_error(
//                 errors,
//                 ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
//             )
//         })?;
//
//     let mut file_source_map = FileSourceMap::default();
//     source_text = replace_sender_placeholder(
//         source_text,
//         &sender.normalized_original,
//         &mut file_source_map,
//     );
//     if !is_inside_libra_directory() {
//         source_text = bech32::replace_bech32_addresses(&source_text, &mut file_source_map);
//     }
//
//     let (defs, _) =
//         syntax::parse_file_string(fname, &source_text, comment_map).map_err(|errors| {
//             into_exec_compiler_error(
//                 errors,
//                 ProjectSourceMap::with_file_map(fname, file_source_map.clone()),
//             )
//         })?;
//     Ok((defs, file_source_map))
// }

type PreBytecodeProgram = cfgir::ast::Program;

pub fn check_defs(
    source_definitions: Vec<Definition>,
    lib_definitions: Vec<Definition>,
    sender: Address,
) -> Result<PreBytecodeProgram, Vec<Error>> {
    let ast_program = parser::ast::Program {
        source_definitions,
        lib_definitions,
    };
    move_lang::check_program(Ok(ast_program), Some(sender))
}

// pub fn check_with_compiler(
//     current: (MoveFilePath, String),
//     deps: Vec<(MoveFilePath, String)>,
//     sender: &ProvidedAccountAddress,
// ) -> Result<(), Vec<CompilerError>> {
//     let (script_defs, dep_defs, offsets_map) = parse_files(current, &deps, sender)
//         .map_err(|errors| errors.transform_with_source_map())?;
//
//     match check_defs(script_defs, dep_defs, sender.as_address()) {
//         Ok(_) => Ok(()),
//         Err(errors) => {
//             Err(into_exec_compiler_error(errors, offsets_map).transform_with_source_map())
//         }
//     }
// }
