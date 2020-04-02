use anyhow::Result;
use lsp_server::{Connection, ProtocolError};
use lsp_types::{ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};
use serde::de::DeserializeOwned;

use crate::config::ServerConfig;
use crate::main_loop;
use crate::main_loop::show_message;

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

pub fn parse_initialize_params(
    init_params: serde_json::Value,
    connection: &Connection,
) -> Result<ServerConfig> {
    let init_params = from_json::<lsp_types::InitializeParams>("InitializeParams", init_params)?;
    if let Some(client_info) = init_params.client_info {
        log::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.unwrap_or_default()
        );
    }
    let server_config = init_params
        .initialization_options
        .and_then(|v| {
            from_json::<ServerConfig>("config", v)
                .map_err(|err| {
                    log::error!("{}", err);
                    show_message(
                        lsp_types::MessageType::Error,
                        err.to_string(),
                        &connection.sender,
                    );
                })
                .ok()
        })
        .unwrap_or_default();
    Ok(server_config)
}

pub fn run_server() -> Result<()> {
    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    let init_params = initialize_server(&connection)?;
    let server_config = parse_initialize_params(init_params, &connection)?;
    log::info!("Initialization is finished");
    log::info!("Server configuration is {:?}", &server_config);

    main_loop::main_loop(server_config, &connection)?;
    io_threads.join()?;
    Ok(())
}

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = T::deserialize(&json)
        .map_err(|err| anyhow::anyhow!("Failed to deserialize {}: {}; {}", what, err, json))?;
    Ok(res)
}
