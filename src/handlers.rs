use lsp_server::Notification;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{PublishDiagnosticsParams, Url};

use crate::ide::db::AnalysisChange;
use crate::main_loop::notification_new;
use crate::utils::io::leaked_fpath;
use crate::world::WorldState;

pub fn on_did_change_document(
    world_state: &mut WorldState,
    document_uri: Url,
    new_source_text: &str,
) -> Notification {
    let fpath = leaked_fpath(document_uri.path());

    // load in memory, or error locations won't work
    let mut change = AnalysisChange::new();
    change.update_file(fpath, new_source_text.to_string());
    world_state.analysis_host.apply_change(change);

    let analysis = world_state.analysis_host.analysis();
    let diagnostics = analysis.check_with_libra_compiler(fpath, new_source_text);
    notification_new::<PublishDiagnostics>(PublishDiagnosticsParams::new(
        document_uri,
        diagnostics,
        None,
    ))
}
