use std::path::PathBuf;

use anyhow::Result;
use lsp_server::{Connection, ProtocolError, RequestId};
use lsp_types::{
    DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, RegistrationParams,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, WatchKind,
};
use serde::de::DeserializeOwned;

use crate::global_state::initialize_new_global_state;
use crate::main_loop;
use crate::main_loop::request_new;
use crate::inner::config::Config;

fn move_language_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Full),
                ..TextDocumentSyncOptions::default()
            },
        )),
        ..ServerCapabilities::default()
    }
}

pub fn initialize_server(connection: &Connection) -> Result<serde_json::Value, ProtocolError> {
    connection.initialize(serde_json::to_value(move_language_server_capabilities()).unwrap())
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

fn register_for_file_changes(connection: &Connection) {
    let move_files_watcher = FileSystemWatcher {
        glob_pattern: "**/*.move".to_string(),
        kind: Some(WatchKind::Delete),
    };
    let registration_options = DidChangeWatchedFilesRegistrationOptions {
        watchers: vec![move_files_watcher],
    };
    let registration = lsp_types::Registration {
        id: "workspace/didChangeWatchedFiles".to_string(),
        method: "workspace/didChangeWatchedFiles".to_string(),
        register_options: Some(serde_json::to_value(registration_options).unwrap()),
    };
    let registration_req = request_new::<lsp_types::request::RegisterCapability>(
        RequestId::from(1),
        RegistrationParams {
            registrations: vec![registration],
        },
    );
    connection.sender.send(registration_req.into()).unwrap();
}

pub fn run_server(connection: &Connection) -> Result<()> {
    let init_params = initialize_server(connection)?;
    let (_, config) = parse_initialize_params(init_params)?;
    log::info!("Initialization is finished");

    register_for_file_changes(connection);

    let mut global_state = initialize_new_global_state(config);
    main_loop::main_loop(&mut global_state, connection)
}

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = T::deserialize(&json)
        .map_err(|err| anyhow::anyhow!("Failed to deserialize {}: {}; {}", what, err, json))?;
    Ok(res)
}
