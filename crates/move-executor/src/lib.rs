use anyhow::{Context, Result};

use dialects::DialectName;

use shared::bech32::bech32_into_libra;
use shared::results::ResourceChange;
use std::collections::HashMap;
use std::str::FromStr;
use utils::MoveFile;

pub fn compile_and_execute_script(
    script: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    genesis_json_contents: serde_json::Value,
    args: Vec<String>,
) -> Result<serde_json::Value> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let initial_genesis_changes = serde_json::from_value::<Vec<ResourceChange>>(
        genesis_json_contents,
    )
    .with_context(|| {
        "Genesis JSON data is in invalid format (list of genesis resource objects required)"
    })?;

    let mut normalized_changes = Vec::with_capacity(initial_genesis_changes.len());
    let mut account_address_replacements = HashMap::new();
    for (i, change) in initial_genesis_changes.into_iter().enumerate() {
        let mut raw_account_address = dialect
            .normalize_account_address(&change.account)
            .with_context(|| format!("Invalid genesis entry {}: Account address is invalid for the selected dialect", i))?;
        if dialect.name() == "dfinance" {
            let bech32_address = bech32_into_libra(&raw_account_address).unwrap();
            account_address_replacements.insert(bech32_address.clone(), raw_account_address);
            raw_account_address = bech32_address;
        }
        normalized_changes.push(ResourceChange {
            account: raw_account_address,
            ty: change.ty,
            op: change.op,
        });
    }

    let raw_sender_string = dialect.normalize_account_address(sender).with_context(|| {
        format!(
            "Specified --sender is not a valid {:?} address: {:?}",
            dialect.name(),
            sender
        )
    })?;
    if dialect.name() == "dfinance" {
        let libra_sender_address = bech32_into_libra(&raw_sender_string).unwrap();
        account_address_replacements.insert(libra_sender_address, raw_sender_string.clone());
    }

    let execution_changes =
        dialect.compile_and_run(script, deps, raw_sender_string, normalized_changes, args)?;

    let mut normalized_changes = Vec::with_capacity(execution_changes.len());
    for ResourceChange {
        mut account,
        mut ty,
        op,
    } in execution_changes
    {
        if let Some(bech32_address) = account_address_replacements.get(&account) {
            account = bech32_address.to_owned();
        }
        if let Some(bech32_address) = account_address_replacements.get(&ty.address) {
            ty.address = bech32_address.to_owned();
        }
        normalized_changes.push(ResourceChange { account, ty, op });
    }
    Ok(serde_json::to_value(normalized_changes).unwrap())
}
