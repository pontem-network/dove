use anyhow::Result;
use lsp_types::CompletionItem;

use crate::req;
use crate::world::WorldSnapshot;

pub fn handle_completion(
    _world: WorldSnapshot,
    _params: req::CompletionParams,
) -> Result<Option<req::CompletionResponse>> {
    let keywords = vec!["struct", "resource", "module", "fun", "return"];
    let completions: Vec<CompletionItem> = keywords
        .into_iter()
        .map(|kw| format!("{} ", kw))
        .map(|kw| CompletionItem::new_simple(kw, "None".to_string()))
        .collect();
    Ok(Some(completions.into()))
}
