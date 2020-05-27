use anyhow::{Context, Result};

use dialects::DialectName;

use shared::results::ResourceChange;
use std::str::FromStr;
use utils::MoveFile;

type ChainStateChanges = serde_json::Value;

pub fn compile_and_execute_script(
    script: MoveFile,
    deps: &[MoveFile],
    dialect: &str,
    sender: &str,
    genesis_json_contents: ChainStateChanges,
    args: Vec<String>,
) -> Result<ChainStateChanges> {
    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let initial_chain_state =
        serde_json::from_value::<Vec<ResourceChange>>(genesis_json_contents)
            .with_context(|| "Genesis contains invalid data")?;
    let raw_sender_string = dialect.normalize_account_address(sender).with_context(|| {
        format!(
            "Specified --sender is not a valid {:?} address: {:?}",
            dialect.name(),
            sender
        )
    })?;

    let execution_changes =
        dialect.compile_and_run(script, deps, raw_sender_string, initial_chain_state, args)?;
    Ok(serde_json::to_value(execution_changes).unwrap())
}
