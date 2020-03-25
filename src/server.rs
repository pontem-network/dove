use anyhow::Result;
use lsp_server::{Connection, ProtocolError};
use lsp_types::{ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};

use crate::main_loop;

pub fn get_default_server_capabilities() -> serde_json::Value {
    serde_json::to_value(&ServerCapabilities::default()).unwrap()
}

pub fn initialize_server(connection: &Connection) -> Result<serde_json::Value, ProtocolError> {
    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full)),
        ..ServerCapabilities::default()
    };
    connection.initialize(serde_json::to_value(server_capabilities).unwrap())
}

pub fn run_server() -> Result<()> {
    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    let _initialization_params = initialize_server(&connection)?;
    log::info!("Initialization is finished");

    main_loop::main_loop(&connection)?;
    io_threads.join()?;
    Ok(())
}
