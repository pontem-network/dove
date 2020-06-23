use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::request::{Initialize, Shutdown};
use lsp_types::{
    ClientCapabilities, DidChangeConfigurationParams, InitializeParams, InitializedParams,
};

use analysis::config::Config;
use crossbeam_channel::{unbounded, Sender};
use dialects::DialectName;

use integration_tests::config;
use integration_tests::get_test_resources_dir;
use lsp_types::notification::{DidChangeConfiguration, Initialized};
use move_language_server::fs::ws_root_vfs;
use move_language_server::global_state::GlobalState;
use move_language_server::main_loop::{main_loop, notification_new, request_new};
use move_language_server::server::run_server;
use ra_vfs::VfsTask;

const SHUTDOWN_REQ_ID: u64 = 10;

#[allow(deprecated)]
fn client_initialize_params() -> InitializeParams {
    let client_caps = ClientCapabilities::default();
    InitializeParams {
        capabilities: client_caps,
        initialization_options: None,
        client_info: None,
        process_id: None,
        root_path: None,
        root_uri: None,
        trace: None,
        workspace_folders: None,
    }
}

fn initialize_req(req_id: usize) -> Message {
    let req =
        request_new::<Initialize>(RequestId::from(req_id as u64), client_initialize_params());
    Message::Request(req)
}

fn shutdown_req() -> Message {
    let req = request_new::<Shutdown>(RequestId::from(SHUTDOWN_REQ_ID), ());
    Message::Request(req)
}

fn notification<N>(params: N::Params) -> Message
where
    N: lsp_types::notification::Notification,
{
    Message::Notification(notification_new::<N>(params))
}

fn response(req_id: usize, contents: serde_json::Value) -> Message {
    Message::Response(Response::new_ok(RequestId::from(req_id as u64), contents))
}

trait MessageType {
    fn into_request(self) -> Request;
    fn into_response(self) -> Response;
}

impl MessageType for Message {
    fn into_request(self) -> Request {
        match self {
            Message::Request(req) => req,
            _ => panic!(),
        }
    }

    fn into_response(self) -> Response {
        match self {
            Message::Response(resp) => resp,
            _ => panic!(),
        }
    }
}

fn send_messages(client_conn: &Connection, messages: Vec<Message>) {
    for message in messages {
        client_conn.sender.try_send(message).unwrap();
    }
    send_shutdown(client_conn);
}

fn send_shutdown(client_conn: &Connection) {
    client_conn.sender.try_send(shutdown_req()).unwrap();
    client_conn
        .sender
        .try_send(Message::Notification(Notification::new(
            "exit".to_string(),
            (),
        )))
        .unwrap();
}

fn global_state(config: Config) -> (GlobalState, Sender<VfsTask>) {
    let ws_root = get_test_resources_dir();
    let (fs_events_sender, fs_events_receiver) = unbounded::<VfsTask>();
    let vfs = ws_root_vfs(ws_root.clone(), fs_events_sender.clone());
    let global_state = GlobalState::new(ws_root, config, vfs, fs_events_receiver);
    (global_state, fs_events_sender)
}

#[test]
fn test_server_initialization() {
    let (client_conn, server_conn) = Connection::memory();
    send_messages(
        &client_conn,
        vec![
            initialize_req(1),
            notification::<Initialized>(InitializedParams {}),
        ],
    );

    run_server(&server_conn).unwrap();

    let init_finished_resp = client_conn.receiver.try_recv().unwrap().into_response();
    assert_eq!(init_finished_resp.id, RequestId::from(1));
    assert_eq!(
        init_finished_resp.result.unwrap()["capabilities"]["textDocumentSync"],
        1
    );
    let shutdown_req = client_conn.receiver.try_recv().unwrap();
    assert_eq!(
        shutdown_req.into_response().id,
        RequestId::from(SHUTDOWN_REQ_ID)
    );
    client_conn.receiver.try_recv().unwrap_err();
}

#[test]
fn test_server_config_change() {
    let (client_conn, server_conn) = Connection::memory();

    // sends workspace/DidChangeNotification
    let didchange_notification =
        notification::<DidChangeConfiguration>(DidChangeConfigurationParams {
            settings: serde_json::json!(""),
        });
    // receives workspace/configuration request from server, responds with json settings
    let updated_settings_response = response(1, serde_json::json!([{"dialect": "dfinance"}]));
    send_messages(
        &client_conn,
        vec![didchange_notification, updated_settings_response],
    );

    let (mut global_state, _) = global_state(config!());
    assert_eq!(global_state.config.dialect_name, DialectName::Libra);

    main_loop(&mut global_state, &server_conn).unwrap();

    assert_eq!(
        client_conn
            .receiver
            .try_recv()
            .unwrap()
            .into_request()
            .method,
        "workspace/configuration"
    );
    assert_eq!(global_state.config.dialect_name, DialectName::DFinance);
}
