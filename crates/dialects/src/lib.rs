use anyhow::Result;
use lang::types::WriteSet;
use shared::errors::CompilerError;
use shared::results::{ExecResult, ResourceChange};

use utils::FilePath;

pub trait Dialect {
    fn validate_sender_address(&self, s: &str) -> Result<String>;

    fn check_with_compiler(
        &self,
        current: (FilePath, String),
        deps: Vec<(FilePath, String)>,
        sender: &str,
    ) -> Result<(), Vec<CompilerError>>;

    fn compile_and_run(
        &self,
        script: (FilePath, String),
        deps: &[(FilePath, String)],
        sender: String,
        genesis_write_set: WriteSet,
    ) -> Result<ExecResult<Vec<ResourceChange>>, Vec<CompilerError>>;
}

#[derive(Default)]
pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn validate_sender_address(&self, s: &str) -> Result<String> {
        lang::types::AccountAddress::from_hex_literal(s)?;
        Ok(s.to_string())
    }

    fn check_with_compiler(
        &self,
        current: (FilePath, String),
        deps: Vec<(FilePath, String)>,
        sender: &str,
    ) -> Result<(), Vec<CompilerError>> {
        lang::check_with_compiler(current, deps, sender)
    }

    fn compile_and_run(
        &self,
        script: (FilePath, String),
        deps: &[(FilePath, String)],
        sender: String,
        genesis_write_set: WriteSet,
    ) -> Result<ExecResult<Vec<ResourceChange>>, Vec<CompilerError>> {
        lang::executor::compile_and_run(script, deps, sender, genesis_write_set)
    }
}

#[derive(Default)]
pub struct MoveDialect;

impl Dialect for MoveDialect {
    fn validate_sender_address(&self, _s: &str) -> Result<String> {
        unimplemented!()
    }

    fn check_with_compiler(
        &self,
        _current: (&'static str, String),
        _deps: Vec<(&'static str, String)>,
        _sender: &str,
    ) -> Result<(), Vec<CompilerError>> {
        unimplemented!()
    }

    fn compile_and_run(
        &self,
        _script: (&'static str, String),
        _deps: &[(&'static str, String)],
        _sender: String,
        _genesis_write_set: WriteSet,
    ) -> Result<ExecResult<Vec<ResourceChange>>, Vec<CompilerError>> {
        unimplemented!()
    }
}
