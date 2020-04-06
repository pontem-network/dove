use anyhow::Result;
use crossbeam_channel::Receiver;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, PublishDiagnostics,
};
use lsp_types::request::Initialize;
use lsp_types::{
    ClientCapabilities, Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, InitializeParams, InitializeResult, Position, Range,
    TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem,
    TextDocumentSyncCapability, TextDocumentSyncKind, Url, VersionedTextDocumentIdentifier,
};

use move_language_server::config::{Config, MoveDialect};
use move_language_server::main_loop::{
    loop_turn, main_loop, notification_cast, notification_new, request_new, LoopState,
};
use move_language_server::server::{from_json, initialize_server, parse_initialize_params};
use move_language_server::test_utils::STDLIB_DIR;
use move_language_server::world::WorldState;

fn setup_test_logging() {
    std::env::set_var("RUST_LOG", "info");
    // silently returns Err if called more than once
    env_logger::builder()
        .is_test(true)
        .try_init()
        .unwrap_or_default();
}

fn send_shutdown_requests(client_connection: &Connection) {
    let shutdown_req = Message::Request(Request::new(
        3.into(),
        "shutdown".to_string(),
        serde_json::json!("null"),
    ));
    client_connection.sender.send(shutdown_req).unwrap();

    let exit_notif = Message::Notification(Notification::new(
        "exit".to_string(),
        serde_json::json!("null"),
    ));
    client_connection.sender.send(exit_notif).unwrap();
}

fn assert_diagnostics(
    diagnostics_message: Message,
    expected_document_url: Url,
    expected_diagnostics: Vec<Diagnostic>,
) {
    assert!(matches!(diagnostics_message, Message::Notification(_)));
    if let Message::Notification(notif) = diagnostics_message {
        let params = notification_cast::<PublishDiagnostics>(notif).unwrap();
        assert_eq!(params.uri, expected_document_url);
        assert_eq!(params.diagnostics, expected_diagnostics);
    }
}

fn assert_receiver_has_only_shutdown_response(client_receiver: Receiver<Message>) {
    let shutdown_message = client_receiver.try_recv().unwrap();

    assert!(matches!(shutdown_message, Message::Response(_)));
    if let Message::Response(resp) = shutdown_message {
        assert_eq!(resp.result, Some(serde_json::to_value(()).unwrap()));
        assert!(resp.error.is_none());
    }
    client_receiver
        .try_recv()
        .expect_err("Unexpected message in the client channel");
}

fn run_main_loop(connection: &Connection) -> Result<()> {
    let config = Config::default();
    let ws_root = std::env::current_dir().unwrap();
    main_loop(ws_root, config, connection)
}

#[test]
fn test_server_returns_successful_response_on_initialization() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let initialize_req = Message::Request(Request::new(
        1.into(),
        "initialize".to_string(),
        serde_json::json!({"capabilities": {}}),
    ));
    client_conn.sender.send(initialize_req).unwrap();

    let initialized_not = Message::Notification(Notification::new(
        "initialized".to_string(),
        serde_json::json!({}),
    ));
    client_conn.sender.send(initialized_not).unwrap();

    initialize_server(&server_conn).unwrap();

    let init_response = client_conn.receiver.try_recv().unwrap();
    assert!(matches!(init_response, Message::Response(_)));
    if let Message::Response(response) = init_response {
        assert!(response.error.is_none());
    }
    assert!(
        client_conn.receiver.is_empty(),
        "Unexpected message after initialization"
    );
}

#[test]
fn test_shutdown_handler_returns_response_to_the_client() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    send_shutdown_requests(&client_conn);
    run_main_loop(&server_conn).unwrap();

    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}

#[test]
fn test_announce_text_document_sync_capabilities() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let initialize_req = Message::Request(Request::new(
        1.into(),
        "initialize".to_string(),
        serde_json::json!({"capabilities": {}}),
    ));
    client_conn.sender.send(initialize_req).unwrap();
    let initialized_notif = Message::Notification(Notification::new(
        "initialized".to_string(),
        serde_json::json!({}),
    ));
    client_conn.sender.send(initialized_notif).unwrap();

    initialize_server(&server_conn).unwrap();

    let server_caps = client_conn.receiver.try_recv().unwrap();
    assert!(matches!(server_caps, Message::Response(_)));
    if let Message::Response(resp) = server_caps {
        assert!(resp.error.is_none());
        assert_eq!(resp.id, RequestId::from(1));
        let caps: InitializeResult = serde_json::from_value(resp.result.unwrap()).unwrap();
        assert_eq!(
            caps.capabilities.text_document_sync,
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full))
        );
    }
    client_conn
        .receiver
        .try_recv()
        .expect_err("Unexpected message in client channel");
}

#[test]
fn test_server_publishes_diagnostic_after_receiving_didopen() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let document_url = Url::parse("file:///foo/bar").unwrap();
    let text_document = TextDocumentItem::new(
        document_url.clone(),
        "move".to_string(),
        1,
        "main() {}".to_string(),
    );
    let didopen_params = DidOpenTextDocumentParams { text_document };
    let didopen_notif = notification_new::<DidOpenTextDocument>(didopen_params);
    client_conn.sender.send(didopen_notif.into()).unwrap();

    send_shutdown_requests(&client_conn);
    run_main_loop(&server_conn).unwrap();

    let diagnostics_message = client_conn.receiver.try_recv().unwrap();
    assert_diagnostics(
        diagnostics_message,
        document_url,
        vec![Diagnostic::new_simple(
            Range::new(Position::new(0, 0), Position::new(0, 4)),
            "Invalid address directive. Expected 'address' got 'main'".to_string(),
        )],
    );
    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}

#[test]
fn test_send_diagnostics_after_didchange() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let document_url = Url::parse("file:///foo/bar").unwrap();
    let text_document = VersionedTextDocumentIdentifier::new(document_url.clone(), 1);
    let didchange_not = notification_new::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document,
        content_changes: vec![TextDocumentContentChangeEvent {
            text: "main() {}".to_string(),
            range: None,
            range_length: None,
        }],
    });
    client_conn
        .sender
        .send(Message::Notification(didchange_not))
        .unwrap();

    send_shutdown_requests(&client_conn);
    run_main_loop(&server_conn).unwrap();

    let diagnostics_message = client_conn.receiver.try_recv().unwrap();
    assert_diagnostics(
        diagnostics_message,
        document_url,
        vec![Diagnostic::new_simple(
            Range::new(Position::new(0, 0), Position::new(0, 4)),
            "Invalid address directive. Expected 'address' got 'main'".to_string(),
        )],
    );
    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}

#[test]
fn test_send_nothing_after_didclose() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let document_url = Url::parse("file:///foo/bar").unwrap();
    let didclose_params = DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier::new(document_url),
    };
    let didclose_notif = notification_new::<DidCloseTextDocument>(didclose_params);
    client_conn.sender.send(didclose_notif.into()).unwrap();

    send_shutdown_requests(&client_conn);
    run_main_loop(&server_conn).unwrap();

    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}

#[test]
fn test_initialize_server_configuration() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let mut initialize_params = from_json::<InitializeParams>(
        "InitializeParams",
        serde_json::json!({ "capabilities": serde_json::to_value(ClientCapabilities::default()).unwrap() }),
    )
    .unwrap();
    initialize_params.initialization_options =
        Some(serde_json::json!({"dialect": "dfinance", "stdlib_path": STDLIB_DIR}));

    let initialize_req = request_new::<Initialize>(RequestId::from(1), initialize_params);
    client_conn.sender.send(initialize_req.into()).unwrap();

    let initialized_not = Message::Notification(Notification::new(
        "initialized".to_string(),
        serde_json::json!({}),
    ));
    client_conn.sender.send(initialized_not).unwrap();

    let init_params = initialize_server(&server_conn).unwrap();
    let (_, config) = parse_initialize_params(init_params).unwrap();
    assert_eq!(config.dialect, MoveDialect::DFinance);
}

#[test]
fn test_update_server_configuration_from_the_client() {
    setup_test_logging();
    let (server_conn, _) = Connection::memory();

    let config_req_id = RequestId::from(1);
    let mut loop_state = LoopState::with_config_request_id(&config_req_id);
    let mut world_state = WorldState::new(std::env::current_dir().unwrap(), Config::default());

    let content = serde_json::json!({
        "dialect": "dfinance"
    });
    let client_config_response = Response::new_ok(config_req_id, vec![content]);

    loop_turn(
        &server_conn,
        &mut world_state,
        &mut loop_state,
        client_config_response.into(),
    )
    .unwrap();

    assert_eq!(world_state.config.dialect, MoveDialect::DFinance);
}
