use anyhow::Result;
use lsp_server::{Connection, Message, Notification};
use lsp_types::PublishDiagnosticsParams;

use lsp_types::notification::DidOpenTextDocument;

use serde::de::DeserializeOwned;

pub fn main_loop(connection: &Connection) -> Result<()> {
    log::info!("starting example main loop");

    for message in &connection.receiver {
        log::debug!("got message: {:?}", message);
        if let Message::Request(req) = &message {
            if connection.handle_shutdown(req)? {
                return Ok(());
            }
        }
        match message {
            Message::Request(req) => {
                log::info!("Got request: {:?}", req);
            }
            Message::Notification(notif) => {
                log::info!("Got notification: {:?}", notif);
                if let Ok(params) = notification_cast::<DidOpenTextDocument>(notif) {
                    log::info!(
                        "Received 'textDocument/didOpen' with text {:?}",
                        params.text_document.text
                    );

                    let diagnostics_notif = Message::Notification(Notification::new(
                        "textDocument/publishDiagnostics".to_string(),
                        PublishDiagnosticsParams::new(params.text_document.uri, vec![], None),
                    ));
                    log::info!("Sending {:?}", &diagnostics_notif);
                    connection.sender.send(diagnostics_notif).unwrap();
                }
            }
            Message::Response(resp) => {
                log::info!("Got response: {:?}", resp);
            }
        }
    }
    Ok(())
}

pub fn notification_cast<N>(
    notification: Notification,
) -> std::result::Result<N::Params, Notification>
where
    N: lsp_types::notification::Notification,
    N::Params: DeserializeOwned,
{
    notification.extract(N::METHOD)
}
