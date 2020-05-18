use anyhow::Result;

pub trait Dialect {
    fn validate_sender_address(&self, s: &str) -> Result<String>;
    // fn parse_address(s: &str) -> Result<Address>;
    // fn parse_account_address(s: &str) -> Result<AccountAddress>;
}

#[derive(Default, Clone)]
pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn validate_sender_address(&self, s: &str) -> Result<String> {
        lang::types::AccountAddress::from_hex_literal(s)?;
        Ok(s.to_string())
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

#[derive(Default, Clone)]
pub struct MoveDialect;

impl Dialect for MoveDialect {
    fn validate_sender_address(&self, _s: &str) -> Result<String> {
        unimplemented!()
    }
}
