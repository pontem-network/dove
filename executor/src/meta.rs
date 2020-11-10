use enum_iterator::IntoEnumIterator;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;

use crate::oracles::oracle_metadata;
use std::collections::HashMap;
use move_core_types::vm_status::StatusCode;

fn status_codes() -> HashMap<String, u64> {
    StatusCode::into_enum_iter()
        .map(|code| (format!("{:?}", code), code as u64))
        .collect()
}

fn split_around<'s>(s: &'s str, p: &str) -> (&'s str, &'s str) {
    let parts: Vec<_> = s.splitn(2, p).collect();
    let key = parts[0].trim();
    let val = parts[1].trim();
    (key, val)
}

fn split_signers(s: &str) -> Vec<AccountAddress> {
    s.split(',')
        .map(|s| s.trim())
        .map(|addr| AccountAddress::from_hex_literal(addr).unwrap())
        .collect()
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionMeta {
    pub signers: Vec<AccountAddress>,
    pub accounts_balance: Vec<(AccountAddress, String, u128)>,
    pub oracle_prices: Vec<(StructTag, u128)>,
    pub current_time: Option<u64>,
    pub aborts_with: Option<u64>,
    pub status: Option<u64>,
    pub dry_run: bool,
}

impl ExecutionMeta {
    pub fn apply_meta_comment(&mut self, comment: String) {
        if !comment.contains(':') {
            return;
        }
        let (key, val) = split_around(&comment, ":");
        match key {
            "signers" => self.signers = split_signers(val),
            "balance" => {
                if !val.contains(' ') {
                    eprintln!("Invalid balance doc comment: {}", comment);
                    return;
                }
                let (address, balance) = split_around(val, " ");
                if !balance.contains(' ') {
                    eprintln!("Invalid balance doc comment: {}", comment);
                    return;
                }
                let (coin, num) = split_around(balance, " ");
                self.accounts_balance.push((
                    AccountAddress::from_hex_literal(address).unwrap(),
                    coin.to_string(),
                    num.parse().unwrap(),
                ));
            }
            "price" => {
                if !val.contains(' ') {
                    eprintln!("Invalid ticker price doc comment: {}", comment);
                    return;
                }
                let (tickers, value) = split_around(val, " ");
                if !tickers.contains('_') {
                    eprintln!("Invalid ticker price doc comment: {}", comment);
                    return;
                }
                let (ticker_left, ticker_right) = split_around(&tickers, "_");
                let price_struct_tag = oracle_metadata(ticker_left, ticker_right);
                self.oracle_prices
                    .push((price_struct_tag, value.parse().unwrap()))
            }
            "current_time" => self.current_time = Some(val.parse().unwrap()),
            "aborts_with" => self.aborts_with = Some(val.parse().unwrap()),
            "status" => {
                self.status = status_codes().get(val).cloned();
                if self.status.is_none() {
                    eprintln!("Unknown status code name: {:?}", val);
                }
            }
            "dry_run" => self.dry_run = val.parse().unwrap(),
            _ => eprintln!("Unimplemented meta key, {:?}", key),
        }
    }
}
