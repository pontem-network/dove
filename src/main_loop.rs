use anyhow::Result;
use crossbeam_channel::Sender;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
    ShowMessage,
};
use lsp_types::request::WorkspaceConfiguration;
use lsp_types::{ConfigurationItem, ConfigurationParams, MessageType, ShowMessageParams};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::compiler::utils::STDLIB_DIR;
use crate::config::ServerConfig;
use crate::handlers;
use crate::world::{Config, WorldState};
use std::path::Path;
use std::time::Instant;

pub fn get_config(server_config: &ServerConfig) -> Config {
    Config {
        dialect: server_config.dialect.clone(),
        stdlib_path: server_config.stdlib_path.clone(),
    }
}

pub fn main_loop(server_config: ServerConfig, connection: &Connection) -> Result<()> {
    log::info!("starting example main loop");

    let mut loop_state = LoopState::default();
    let mut world_state = WorldState::new(get_config(&server_config));
    world_state.load_stdlib(Path::new(STDLIB_DIR));

    for message in &connection.receiver {
        log::debug!("got message: {:?}", message);
        if let Message::Request(req) = &message {
            if connection.handle_shutdown(req)? {
                return Ok(());
            }
        }
        loop_turn(&connection, &mut world_state, &mut loop_state, message)?;
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct LoopState {
    next_request_id: u64,
    configuration_request_id: Option<RequestId>,
}

impl LoopState {
    fn next_request_id(&mut self) -> RequestId {
        self.next_request_id += 1;
        self.next_request_id.into()
    }

    pub fn with_config_request_id(req_id: &RequestId) -> Self {
        LoopState {
            configuration_request_id: Some(req_id.to_owned()),
            ..LoopState::default()
        }
    }
}

pub fn loop_turn(
    connection: &Connection,
    world_state: &mut WorldState,
    loop_state: &mut LoopState,
    message: Message,
) -> Result<()> {
    match message {
        Message::Request(req) => {
            log::info!("Got request: {:?}", req);
        }
        Message::Notification(not) => {
            log::info!("Got notification: {:?}", not);
            on_notification(&connection.sender, world_state, loop_state, not)?;
        }
        Message::Response(resp) => {
            log::info!("Got response: {:?}", resp);
            if Some(&resp.id) == loop_state.configuration_request_id.as_ref() {
                loop_state.configuration_request_id = None;
                log::info!("config update response: '{:?}", resp);

                let Response { error, result, .. } = resp;
                let parsed_config_val = result.map(serde_json::from_value::<Vec<ServerConfig>>);

                match (error, parsed_config_val) {
                    (Some(err), _) => log::error!("failed to fetch the server settings: {:?}", err),
                    (None, Some(Ok(new_config))) => {
                        let new_server_config = new_config
                            .first()
                            .expect("the client is expected to always send a non-empty config data")
                            .to_owned();
                        world_state.update_configuration(get_config(&new_server_config));
                    }
                    (None, Some(Err(e))) => {
                        log::error!("failed to parse client config response: {}", e)
                    }
                    (None, None) => {
                        log::error!("received empty server settings response from the client")
                    }
                }
            }
        }
    };
    Ok(())
}

fn on_notification(
    msg_sender: &Sender<Message>,
    world_state: &WorldState,
    loop_state: &mut LoopState,
    not: Notification,
) -> Result<()> {
    let notif = match notification_cast::<DidOpenTextDocument>(not) {
        Ok(params) => {
            let source_text = params.text_document.text;
            let not =
                handlers::on_document_change(world_state, params.text_document.uri, &source_text);
            log::info!("Sending {:?}", &not);
            msg_sender.send(not.into()).unwrap();
            return Ok(());
        }
        Err(not) => not,
    };
    let notif = match notification_cast::<DidChangeTextDocument>(notif) {
        Ok(params) => {
            let source_text = params.content_changes.get(0).unwrap().clone().text;
            let not =
                handlers::on_document_change(world_state, params.text_document.uri, &source_text);
            log::info!("Sending {:?}", &not);
            msg_sender.send(not.into())?;

            return Ok(());
        }
        Err(not) => not,
    };
    let not = match notification_cast::<DidCloseTextDocument>(notif) {
        Ok(_) => {
            return Ok(());
        }
        Err(not) => not,
    };
    let not = match notification_cast::<DidChangeConfiguration>(not) {
        Ok(_) => {
            // As stated in https://github.com/microsoft/language-server-protocol/issues/676,
            // this notification's parameters should be ignored and the actual config queried separately.
            let request_id = loop_state.next_request_id();
            let config_item = ConfigurationItem {
                section: Some("move".to_string()),
                scope_uri: None,
            };
            let request = request_new::<WorkspaceConfiguration>(
                request_id.clone(),
                ConfigurationParams {
                    items: vec![config_item],
                },
            );
            log::info!("Sending config request: {:?}", &request);
            msg_sender.send(request.into())?;
            loop_state.configuration_request_id = Some(request_id);

            return Ok(());
        }
        Err(not) => not,
    };
    if not.method.starts_with("$/") {
        return Ok(());
    }
    // log::error!("unhandled notification: {:?}", notif);
    Ok(())
}

pub fn notification_cast<N>(notification: Notification) -> Result<N::Params, Notification>
where
    N: lsp_types::notification::Notification,
    N::Params: DeserializeOwned,
{
    notification.extract(N::METHOD)
}

pub fn notification_new<N>(params: N::Params) -> Notification
where
    N: lsp_types::notification::Notification,
    N::Params: Serialize,
{
    Notification::new(N::METHOD.to_string(), params)
}

pub fn request_new<R>(id: RequestId, params: R::Params) -> Request
where
    R: lsp_types::request::Request,
    R::Params: Serialize,
{
    Request::new(id, R::METHOD.to_string(), params)
}

pub fn show_message(typ: MessageType, message: impl Into<String>, sender: &Sender<Message>) {
    let message = message.into();
    let params = ShowMessageParams { typ, message };
    let not = notification_new::<ShowMessage>(params);
    sender.send(not.into()).unwrap();
}
