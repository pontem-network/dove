use analysis::config::Config;
use integration_tests::global_state_snapshot;
use lsp_types::{
    CompletionItem, Position, TextDocumentIdentifier, TextDocumentPositionParams, Url,
};
use move_language_server::handlers::handle_completion;
use move_language_server::req;

use utils::{MoveFile, MoveFilePath};

fn position_params(pos: (u64, u64), fpath: MoveFilePath) -> req::TextDocumentPositionParams {
    let text_document = TextDocumentIdentifier::new(Url::from_file_path(fpath).unwrap());
    let position = Position::new(pos.0, pos.1);
    TextDocumentPositionParams::new(text_document, position)
}

fn completions(file: MoveFile, config: Config, pos: (u64, u64)) -> Vec<CompletionItem> {
    let snapshot = global_state_snapshot(file.clone(), config, vec![file.clone()]);
    let comp_params = req::CompletionParams {
        text_document_position: position_params(pos, file.0),
        work_done_progress_params: req::WorkDoneProgressParams::default(),
        partial_result_params: req::PartialResultParams::default(),
        context: None,
    };
    let comp_response = handle_completion(snapshot, comp_params).unwrap().unwrap();
    match comp_response {
        req::CompletionResponse::Array(items) => items,
        req::CompletionResponse::List(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::completions;
    use integration_tests::{config, get_script_path};

    #[test]
    fn test_top_level_completion() {
        let source_text = "script { fun main() {} }";
        let completions = completions(
            (get_script_path(), source_text.to_string()),
            config!({}),
            (0, 0),
        );
        dbg!(completions);
    }
}
