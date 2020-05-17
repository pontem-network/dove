use anyhow::Result;

pub trait Dialect {
    fn validate_sender_address(s: String) -> Result<String>;
    // fn parse_address(s: &str) -> Result<Address>;
    // fn parse_account_address(s: &str) -> Result<AccountAddress>;
}

pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn validate_sender_address(s: String) -> Result<String> {
        lang::types::AccountAddress::from_hex_literal(&s)?;
        Ok(s)
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
