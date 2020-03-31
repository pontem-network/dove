use lsp_server::Notification;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{PublishDiagnosticsParams, Url};

use crate::compiler::parser::parse_and_extract_diagnostics;
use crate::main_loop::notification_new;

pub(crate) fn on_document_change(document_uri: Url, new_source_text: &str) -> Notification {
    let fname = Box::leak(Box::new(document_uri.to_string()));
    let mut diagnostics = vec![];
    if let Err(diags) = parse_and_extract_diagnostics(fname, new_source_text) {
        diagnostics = diags;
    }
    notification_new::<PublishDiagnostics>(PublishDiagnosticsParams::new(
        document_uri,
        diagnostics,
        None,
    ))
}
