use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::request::{Initialize, Shutdown};
use lsp_types::{
    ClientCapabilities, DidChangeConfigurationParams, DidChangeWatchedFilesParams,
    FileChangeType, FileEvent, InitializeParams, InitializedParams, Url,
};

use dialects::DialectName;

use lsp_types::notification::{DidChangeConfiguration, DidChangeWatchedFiles, Initialized};

use move_language_server::global_state::{initialize_new_global_state, GlobalState};
use move_language_server::main_loop::{main_loop, notification_new, request_new, FileSystemEvent};
use move_language_server::server::run_server;

use move_language_server::inner::config::Config;
use utils::tests::get_script_path;

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
    fn into_notification(self) -> Notification;
}

impl MessageType for Message {
    fn into_request(self) -> Request {
        match self {
            Message::Request(req) => req,
            _ => panic!("not a request"),
        }
    }

    fn into_response(self) -> Response {
        match self {
            Message::Response(resp) => resp,
            _ => panic!("not a response"),
        }
    }

    fn into_notification(self) -> Notification {
        match self {
            Message::Notification(notification) => notification,
            _ => panic!("not a notification"),
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

fn global_state(config: Config) -> GlobalState {
    initialize_new_global_state(config)
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
        serde_json::json!({"change": 1, "openClose": true})
    );
    let registration_req = client_conn.receiver.try_recv().unwrap().into_request();
    assert_eq!(registration_req.method, "client/registerCapability");
    assert_eq!(
        registration_req.params["registrations"][0]["method"],
        "workspace/didChangeWatchedFiles"
    );

    let shutdown_resp = client_conn.receiver.try_recv().unwrap().into_response();
    assert_eq!(shutdown_resp.id, RequestId::from(SHUTDOWN_REQ_ID));

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

    let mut global_state = global_state(Config::default());
    assert_eq!(global_state.config().dialect_name, DialectName::Libra);

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
    assert_eq!(global_state.config().dialect_name, DialectName::DFinance);
}

#[test]
fn test_removed_file_not_present_in_the_diagnostics() {
    let (client_conn, server_conn) = Connection::memory();

    let script_text = r"script {
        use 0x0::Unknown;
        fun main() {}
    }";
    let script_file = (get_script_path(), script_text.to_string());

    let mut global_state = global_state(Config::default());
    global_state.update_from_events(vec![FileSystemEvent::AddFile(script_file)]);

    let delete_event = FileEvent::new(
        Url::from_file_path(get_script_path()).unwrap(),
        FileChangeType::Deleted,
    );
    let files_changed_notification =
        notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
            changes: vec![delete_event],
        });
    send_messages(&client_conn, vec![files_changed_notification]);

    main_loop(&mut global_state, &server_conn).unwrap();

    assert!(global_state.analysis().db().available_files.is_empty());
}
