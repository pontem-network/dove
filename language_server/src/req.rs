//! Defines specific custom messages.

pub use lsp_types::{
    notification::*, request::*, ApplyWorkspaceEditParams, CodeActionParams, CodeLens,
    CodeLensParams, CompletionParams, CompletionResponse, ConfigurationItem, ConfigurationParams,
    DiagnosticTag, DidChangeConfigurationParams, DidChangeWatchedFilesParams,
    DidChangeWatchedFilesRegistrationOptions, DocumentOnTypeFormattingParams,
    DocumentSymbolParams, DocumentSymbolResponse, FileSystemWatcher, Hover, InitializeResult,
    MessageType, PartialResultParams, ProgressParams, ProgressParamsValue, ProgressToken,
    PublishDiagnosticsParams, ReferenceParams, Registration, RegistrationParams, SelectionRange,
    SelectionRangeParams, ServerCapabilities, ShowMessageParams,
    SignatureHelp, SymbolKind, TextDocumentEdit, TextDocumentPositionParams, TextEdit,
    WorkDoneProgressParams, WorkspaceEdit, WorkspaceSymbolParams,
};
