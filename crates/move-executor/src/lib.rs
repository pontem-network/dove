use anyhow::{Context, Result};

use dialects::DialectName;

use shared::results::ResourceChange;
use std::str::FromStr;
use utils::File;

type ChainStateChanges = serde_json::Value;

pub fn compile_and_execute_script(
    script: File,
    deps: &[File],
    dialect: String,
    sender: String,
    genesis_json_contents: ChainStateChanges,
) -> Result<ChainStateChanges> {
    let dialect = DialectName::from_str(&dialect)?.get_dialect();
    let initial_chain_state =
        serde_json::from_value::<Vec<ResourceChange>>(genesis_json_contents)
            .with_context(|| "Genesis contains invalid data")?;

    let execution_changes = dialect.compile_and_run(script, deps, sender, initial_chain_state)?;
    Ok(serde_json::to_value(execution_changes).unwrap())
}
