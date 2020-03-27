use crossbeam_channel::Receiver;
use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::notification::{DidCloseTextDocument, DidOpenTextDocument, PublishDiagnostics};
use lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeResult, TextDocumentIdentifier, TextDocumentItem, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url, VersionedTextDocumentIdentifier,
};

use move_language_server::main_loop::{main_loop, notification_cast, notification_new};
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
    main_loop(&server_conn).unwrap();

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
        "mytext".to_string(),
    );
    let didopen_params = DidOpenTextDocumentParams { text_document };
    let didopen_notif = notification_new::<DidOpenTextDocument>(didopen_params);
    client_conn.sender.send(didopen_notif.into()).unwrap();

    send_shutdown_requests(&client_conn);
    main_loop(&server_conn).unwrap();

    let diagnostics_message = client_conn.receiver.try_recv().unwrap();
    assert_diagnostics(diagnostics_message, document_url, vec![]);

    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}

#[test]
fn test_send_diagnostics_after_didchange() {
    setup_test_logging();
    let (server_conn, client_conn) = Connection::memory();

    let document_url = Url::parse("file:///foo/bar").unwrap();
    let text_document = VersionedTextDocumentIdentifier::new(document_url.clone(), 1);
    let didchange_notif = Notification::new(
        "textDocument/didChange".to_string(),
        DidChangeTextDocumentParams {
            text_document,
            content_changes: vec![],
        },
    );
    client_conn
        .sender
        .send(Message::Notification(didchange_notif))
        .unwrap();

    send_shutdown_requests(&client_conn);
    main_loop(&server_conn).unwrap();

    let diagnostics_message = client_conn.receiver.try_recv().unwrap();
    assert_diagnostics(diagnostics_message, document_url, vec![]);

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
    main_loop(&server_conn).unwrap();

    assert_receiver_has_only_shutdown_response(client_conn.receiver);
}
