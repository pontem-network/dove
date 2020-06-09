use anyhow::{Context, Result};

use dialects::DialectName;

use dialects::shared::results::{AddressMap, ChainStateChanges, ResourceChange};

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

    let mut lowered_genesis_changes = Vec::with_capacity(initial_genesis_changes.len());
    let mut address_map = AddressMap::default();
    for (i, change) in initial_genesis_changes.into_iter().enumerate() {
        let provided_address = dialect
            .normalize_account_address(&change.account)
            .with_context(|| format!("Invalid genesis entry {}: Account address is invalid for the selected dialect", i))?;
        address_map.insert(provided_address);
        lowered_genesis_changes.push(change.with_replaced_addresses(&address_map.forward()));
    }

    let provided_address = dialect.normalize_account_address(sender).with_context(|| {
        format!(
            "Specified --sender is not a valid {:?} address: {:?}",
            dialect.name(),
            sender
        )
    })?;
    address_map.insert(provided_address.clone());

    let chain_state = dialect.compile_and_run(
        script,
        deps,
        provided_address,
        lowered_genesis_changes,
        args,
    )?;

    let ChainStateChanges {
        resource_changes,
        gas_spent,
        events,
    } = chain_state;
    let normalized_changes: Vec<_> = resource_changes
        .into_iter()
        .map(|change| change.with_replaced_addresses(&address_map.reversed()))
        .collect();
    Ok(serde_json::json!({
        "changes": normalized_changes,
        "gas_spent": gas_spent,
        "events": events
    }))
}
