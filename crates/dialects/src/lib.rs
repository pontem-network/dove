use anyhow::Result;
use lang::types::WriteSet;
use shared::errors::CompilerError;
use shared::results::ResourceChange;

use utils::FilePath;

pub trait Dialect {
    fn preprocess_and_validate_account_address(&self, s: &str) -> Result<String>;

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
        sender_address: String,
        genesis_write_set: WriteSet,
    ) -> Result<Vec<ResourceChange>>;
}

#[derive(Default)]
pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn preprocess_and_validate_account_address(&self, s: &str) -> Result<String> {
        lang::parse_account_address(s).map(|address| format!("0x{}", address))
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
        sender_address: String,
        genesis_write_set: WriteSet,
    ) -> Result<Vec<ResourceChange>> {
        lang::executor::compile_and_run(script, deps, sender_address, genesis_write_set)
    }
}

#[derive(Default)]
pub struct MoveDialect;

impl Dialect for MoveDialect {
    fn preprocess_and_validate_account_address(&self, _s: &str) -> Result<String> {
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
        _sender_address: String,
        _genesis_write_set: WriteSet,
    ) -> Result<Vec<ResourceChange>> {
        unimplemented!()
    }
}
