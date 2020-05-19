use anyhow::Result;

use shared::errors::CompilerError;
use shared::results::ResourceChange;

use shared::bech32::bech32_into_libra;
use utils::{FilePath, FilesSourceText};

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
        genesis_changes: Vec<ResourceChange>,
    ) -> Result<Vec<ResourceChange>>;

    fn print_compiler_errors_and_exit(
        &self,
        files: FilesSourceText,
        errors: Vec<CompilerError>,
    ) -> !;
}

#[derive(Default)]
pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn preprocess_and_validate_account_address(&self, s: &str) -> Result<String> {
        let s = bech32_into_libra(s)?;
        lang::parse_account_address(&s).map(|address| format!("0x{}", address))
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
        genesis_changes: Vec<ResourceChange>,
    ) -> Result<Vec<ResourceChange>> {
        let genesis_write_set = lang::resources::changes_into_writeset(genesis_changes)?;
        lang::executor::compile_and_run(script, deps, sender_address, genesis_write_set)
    }

    fn print_compiler_errors_and_exit(
        &self,
        files: FilesSourceText,
        errors: Vec<CompilerError>,
    ) -> ! {
        lang::report_errors(files, errors)
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
        _genesis_changes: Vec<ResourceChange>,
    ) -> Result<Vec<ResourceChange>> {
        unimplemented!()
    }

    fn print_compiler_errors_and_exit(
        &self,
        _files: FilesSourceText,
        _errors: Vec<CompilerError>,
    ) -> ! {
        unimplemented!()
    }
}
