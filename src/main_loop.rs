use anyhow::Result;
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;

pub fn main_loop(connection: &Connection, params: serde_json::Value) -> Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    log::info!("starting example main loop");

    for msg in &connection.receiver {
        log::info!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                log::info!("got request: {:?}", req);
            }
            Message::Response(resp) => {
                log::info!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                log::info!("got notification: {:?}", not);
            }
        }
    }
    Ok(())
}
