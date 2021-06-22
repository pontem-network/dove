use enum_iterator::IntoEnumIterator;

use move_core_types::account_address::AccountAddress;
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
            "aborts_with" => self.aborts_with = Some(val.parse().unwrap()),
            "status" => {
                self.status = status_codes().get(val).copied();
                if self.status.is_none() {
                    eprintln!("Unknown status code name: {:?}", val);
                }
            }
            "dry_run" => self.dry_run = val.parse().unwrap(),
            _ => eprintln!("Unimplemented meta key, {:?}", key),
        }
    }
}
