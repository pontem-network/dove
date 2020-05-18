use anyhow::Result;
use shared::errors::CompilerError;
use utils::FilePath;

pub trait Dialect {
    fn validate_sender_address(&self, s: &str) -> Result<String>;
    // fn parse_address(s: &str) -> Result<Address>;
    // fn parse_account_address(s: &str) -> Result<AccountAddress>;

    fn check_with_compiler(
        &self,
        current: (FilePath, String),
        deps: Vec<(FilePath, String)>,
        sender: &str,
    ) -> Result<(), Vec<CompilerError>>;
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

    // fn parse_address(s: &str) -> Result<Address> {
    //     Ok(dfinance::types::Address::new(
    //         Self::parse_account_address(s)?.into(),
    //     ))
    // }
    //
    // fn parse_account_address(s: &str) -> Result<AccountAddress> {
    //     dfinance::types::AccountAddress::from_hex_literal(s)
    // }
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
}
