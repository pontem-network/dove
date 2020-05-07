use anyhow::Result;

pub use libra_types::account_address::AccountAddress;

pub fn parse_account_address(s: &str) -> Result<AccountAddress> {
    AccountAddress::from_hex_literal(s)
}
