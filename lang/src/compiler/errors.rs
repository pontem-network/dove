use serde::export::Formatter;
use std::fmt;

use crate::compiler::source_map::{ProjectSourceMap, Location};
use move_lang::errors::{Error, FilesSourceText};
use codespan::ByteIndex;
use move_ir_types::location::Loc;

pub fn from_compiler_error(comp_error: CompilerError) -> Error {
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

#[derive(Debug, Clone)]
pub struct CompilerErrorPart {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub parts: Vec<CompilerErrorPart>,
}

#[derive(Debug, Default)]
pub struct ExecCompilerError(pub Vec<CompilerError>, pub ProjectSourceMap);

impl ExecCompilerError {
    pub fn empty() -> ExecCompilerError {
        ExecCompilerError {
            0: vec![],
            1: Default::default(),
        }
    }

    pub fn transform_with_source_map(self) -> Vec<CompilerError> {
        let ExecCompilerError(errors, project_source_map) = self;
        errors
            .into_iter()
            .map(|error| project_source_map.transform(error))
            .collect()
    }

    pub fn extend(&mut self, other: ExecCompilerError) {
        let ExecCompilerError(errors, proj_offsets_map) = other;
        self.0.extend(errors);
        for (fpath, offsets_map) in proj_offsets_map.0.into_iter() {
            self.1.insert(fpath, offsets_map);
        }
    }
}

impl std::error::Error for ExecCompilerError {}

impl fmt::Display for ExecCompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
