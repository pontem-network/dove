use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{
    DidOpenTextDocumentParams, InitializeResult, TextDocumentItem, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};

use move_language_server::main_loop::{main_loop, notification_cast};

use move_language_server::server::initialize_server;

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

    let initialized_notif = Message::Notification(Notification::new(
        "initialized".to_string(),
        serde_json::json!({}),
    ));
    client_conn.sender.send(initialized_notif).unwrap();

    initialize_server(&server_conn).unwrap();

    let init_response = client_conn.receiver.try_recv().unwrap();
    assert!(matches!(init_response, Message::Response(_)));
    if let Message::Response(response) = init_response {
        assert!(response.error.is_none());
    }
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
}

#[test]
fn test_server_publishes_diagnostic_after_receiving_didopen() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let document_url = Url::parse("file:///foo/bar").unwrap();
    let opened_document = TextDocumentItem::new(
        document_url.clone(),
        "move".to_string(),
        1,
        "mytext".to_string(),
    );
    let didopen_notif = Notification::new(
        "textDocument/didOpen".to_string(),
        DidOpenTextDocumentParams {
            text_document: opened_document,
        },
    );
    client_conn
        .sender
        .send(Message::Notification(didopen_notif))
        .unwrap();

    send_shutdown_requests(&client_conn);
    main_loop(&server_conn).unwrap();

    let diagnostics_message = client_conn.receiver.try_recv().unwrap();
    assert!(matches!(diagnostics_message, Message::Notification(_)));
    if let Message::Notification(notif) = diagnostics_message {
        let params = notification_cast::<PublishDiagnostics>(notif).unwrap();
        assert_eq!(params.uri, document_url);
        assert_eq!(params.diagnostics, vec![]);
    }
}
