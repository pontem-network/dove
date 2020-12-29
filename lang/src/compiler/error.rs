use libra::move_lang::errors::{Errors, FilesSourceText};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub source_map: FilesSourceText,
    pub errors: Errors,
}

impl CompilerError {
    pub fn new(errors: Errors, source_map: FilesSourceText) -> CompilerError {
        CompilerError { source_map, errors }
    }
}

impl Error for CompilerError {}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}
