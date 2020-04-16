use anyhow::Result;

use crate::req;
use crate::world::WorldSnapshot;

pub fn handle_completion(
    world: WorldSnapshot,
    _params: req::CompletionParams,
) -> Result<Option<req::CompletionResponse>> {
    let completions = world.analysis.completions();
    Ok(Some(completions.into()))
}
