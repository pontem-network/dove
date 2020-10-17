use serde::export::Formatter;
use std::fmt;

use crate::compiler::source_map::{ProjectSourceMap, Location};
use move_lang::errors::Error;
use codespan::ByteIndex;
use move_ir_types::location::Loc;

#[derive(Debug, Clone)]
pub struct CompilerErrorPart {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub parts: Vec<CompilerErrorPart>,
}

impl From<Error> for CompilerError {
    fn from(err: Error) -> Self {
        let mut parts = vec![];
        for (loc, message) in err {
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
}

impl Into<Error> for CompilerError {
    fn into(self) -> Error {
        self.parts
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
}

#[derive(Debug, Default)]
pub struct ExecCompilerError(pub Vec<CompilerError>, pub ProjectSourceMap);

impl ExecCompilerError {
    pub fn new(errors: Vec<Error>, offsets_map: ProjectSourceMap) -> ExecCompilerError {
        let compiler_errors = errors.into_iter().map(CompilerError::from).collect();
        ExecCompilerError(compiler_errors, offsets_map)
    }

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
