use lsp_server::Notification;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{PublishDiagnosticsParams, Url};

use crate::compiler;

use crate::main_loop::notification_new;

pub(crate) fn on_document_change(document_uri: Url, new_source_text: &str) -> Notification {
    let fname = Box::leak(Box::new(document_uri.to_string()));
    let diagnostics = match compiler::check_with_compiler(fname, new_source_text) {
        Ok(_) => vec![],
        Err(diagnostics) => diagnostics,
    };
    notification_new::<PublishDiagnostics>(PublishDiagnosticsParams::new(
        document_uri,
        diagnostics,
        None,
    ))
}
