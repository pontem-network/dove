use std::path::PathBuf;

use anyhow::Result;
use lsp_server::{Connection, ProtocolError};
use lsp_types::{
    CompletionOptions, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde::de::DeserializeOwned;

use analysis::config::Config;

use crate::main_loop;

pub fn get_default_server_capabilities() -> serde_json::Value {
    serde_json::to_value(&ServerCapabilities::default()).unwrap()
}

pub fn initialize_server(connection: &Connection) -> Result<serde_json::Value, ProtocolError> {
    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full)),
        completion_provider: Some(CompletionOptions::default()),
        ..ServerCapabilities::default()
    };
    connection.initialize(serde_json::to_value(server_capabilities).unwrap())
}

pub fn parse_initialize_params(init_params: serde_json::Value) -> Result<(PathBuf, Config)> {
    let initialize_params =
        from_json::<lsp_types::InitializeParams>("InitializeParams", init_params)?;
    if let Some(client_info) = initialize_params.client_info {
        log::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.unwrap_or_default()
        );
    }

    let cwd = std::env::current_dir()?;
    let root = initialize_params
        .root_uri
        .and_then(|it| it.to_file_path().ok())
        .unwrap_or(cwd);

    let mut config = Config::default();
    if let Some(value) = &initialize_params.initialization_options {
        config.update(value);
    }
    Ok((root, config))
}

pub fn run_server() -> Result<()> {
    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    let init_params = initialize_server(&connection)?;
    let (ws_root, config) = parse_initialize_params(init_params)?;
    log::info!("Initialization is finished");

    main_loop::main_loop(ws_root, config, &connection)?;
    io_threads.join()?;
    Ok(())
}

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = T::deserialize(&json)
        .map_err(|err| anyhow::anyhow!("Failed to deserialize {}: {}; {}", what, err, json))?;
    Ok(res)
}
