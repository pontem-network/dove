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
