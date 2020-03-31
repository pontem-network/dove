use anyhow::Result;
use crossbeam_channel::Sender;
use lsp_server::{Connection, Message, Notification};
use lsp_types::notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::handlers;

pub fn main_loop(connection: &Connection) -> Result<()> {
    log::info!("starting example main loop");

    for message in &connection.receiver {
        log::debug!("got message: {:?}", message);
        if let Message::Request(req) = &message {
            if connection.handle_shutdown(req)? {
                return Ok(());
            }
        }
        loop_turn(&connection, message)?;
    }
    Ok(())
}

fn loop_turn(connection: &Connection, message: Message) -> Result<()> {
    match message {
        Message::Request(req) => {
            log::info!("Got request: {:?}", req);
        }
        Message::Notification(notif) => {
            log::info!("Got notification: {:?}", notif);
            on_notification(&connection.sender, notif)?;
        }
        Message::Response(resp) => {
            log::info!("Got response: {:?}", resp);
        }
    };
    Ok(())
}

fn on_notification(msg_sender: &Sender<Message>, notif: Notification) -> Result<()> {
    let notif = match notification_cast::<DidOpenTextDocument>(notif) {
        Ok(params) => {
            let source_text = params.text_document.text;
            let not = handlers::on_document_change(params.text_document.uri, &source_text);
            log::info!("Sending {:?}", &not);
            msg_sender.send(not.into()).unwrap();
            return Ok(());
        }
        Err(notif) => notif,
    };
    let notif = match notification_cast::<DidChangeTextDocument>(notif) {
        Ok(params) => {
            let source_text = params.content_changes.get(0).unwrap().clone().text;
            let not = handlers::on_document_change(params.text_document.uri, &source_text);
            log::info!("Sending {:?}", &not);
            msg_sender.send(not.into()).unwrap();
            return Ok(());
        }
        Err(notif) => notif,
    };
    let notif = match notification_cast::<DidCloseTextDocument>(notif) {
        Ok(_) => {
            return Ok(());
        }
        Err(notif) => notif,
    };
    if notif.method.starts_with("$/") {
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
