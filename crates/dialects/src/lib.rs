use anyhow::Result;

use lang::{dfina, libra};

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
        dfina::parse_account_address(&s).map(|address| format!("0x{}", address))
    }

    fn check_with_compiler(
        &self,
        current: (FilePath, String),
        deps: Vec<(FilePath, String)>,
        sender: &str,
    ) -> Result<(), Vec<CompilerError>> {
        dfina::check_with_compiler(current, deps, sender)
    }

    fn compile_and_run(
        &self,
        script: (FilePath, String),
        deps: &[(FilePath, String)],
        sender_address: String,
        genesis_changes: Vec<ResourceChange>,
    ) -> Result<Vec<ResourceChange>> {
        let genesis_write_set = dfina::resources::changes_into_writeset(genesis_changes)?;
        dfina::executor::compile_and_run(script, deps, sender_address, genesis_write_set)
    }

    fn print_compiler_errors_and_exit(
        &self,
        files: FilesSourceText,
        errors: Vec<CompilerError>,
    ) -> ! {
        dfina::report_errors(files, errors)
    }
}

#[derive(Default)]
pub struct MoveDialect;

impl Dialect for MoveDialect {
    fn preprocess_and_validate_account_address(&self, s: &str) -> Result<String> {
        libra::parse_account_address(&s).map(|address| format!("0x{}", address))
    }

    fn check_with_compiler(
        &self,
        current: (FilePath, String),
        deps: Vec<(FilePath, String)>,
        sender: &str,
    ) -> Result<(), Vec<CompilerError>> {
        libra::check_with_compiler(current, deps, sender)
    }

    fn compile_and_run(
        &self,
        script: (FilePath, String),
        deps: &[(FilePath, String)],
        sender_address: String,
        genesis_changes: Vec<ResourceChange>,
    ) -> Result<Vec<ResourceChange>> {
        let genesis_write_set = libra::resources::changes_into_writeset(genesis_changes)?;
        libra::executor::compile_and_run(script, deps, sender_address, genesis_write_set)
    }

    fn print_compiler_errors_and_exit(
        &self,
        files: FilesSourceText,
        errors: Vec<CompilerError>,
    ) -> ! {
        libra::report_errors(files, errors)
    }
}
