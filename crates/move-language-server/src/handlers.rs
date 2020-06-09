use anyhow::Result;

use crate::global_state::GlobalStateSnapshot;
use crate::req;
use analysis::db::FilePosition;
use utils::leaked_fpath;

pub fn handle_completion(
    state_snapshot: GlobalStateSnapshot,
    params: req::CompletionParams,
) -> Result<Option<req::CompletionResponse>> {
    let req::TextDocumentPositionParams {
        text_document,
        position,
    } = params.text_document_position;
    let fpath = leaked_fpath(text_document.uri.to_file_path().unwrap());
    let pos = (position.line, position.character);
    let file_position = FilePosition {
        fpath,
        pos: (pos.0 as usize, pos.1 as usize),
    };

    let completions = state_snapshot.analysis.completions(file_position);
    Ok(Some(completions.into()))
}
