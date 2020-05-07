use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Result;
use crossbeam_channel::{unbounded, Sender};
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
    PublishDiagnostics, ShowMessage,
};
use lsp_types::request::WorkspaceConfiguration;
use lsp_types::{
    ConfigurationItem, ConfigurationParams, MessageType, PublishDiagnosticsParams,
    ShowMessageParams, Url,
};
use ra_vfs::VfsTask;
use serde::de::DeserializeOwned;
use serde::Serialize;
use threadpool::ThreadPool;

use analysis::analysis::Analysis;
use analysis::config::Config;
use analysis::db::{FileDiagnostic, FilePath};
use analysis::utils::io::leaked_fpath;

use crate::dispatcher::PoolDispatcher;
use crate::handlers;
use crate::req;
use crate::subscriptions::OpenedFiles;
use crate::world::WorldState;

#[derive(Debug)]
pub struct LspError {
    pub code: i32,
    pub message: String,
}

impl LspError {
    pub const UNKNOWN_FILE: i32 = -32900;

    pub fn new(code: i32, message: String) -> LspError {
        LspError { code, message }
    }
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl Error for LspError {}

#[derive(Debug)]
pub enum Task {
    Respond(Response),
    Diagnostic(Vec<FileDiagnostic>),
}

pub enum Event {
    Task(Task),
    Vfs(VfsTask),
    Msg(Message),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Event::Msg(Message::Notification(not)) = self {
            if notification_is::<DidOpenTextDocument>(not)
                || notification_is::<DidChangeTextDocument>(not)
            {
                let document_uri = not.params.pointer("/textDocument/uri").unwrap().to_string();
                return f
                    .debug_struct("Notification")
                    .field("method", &not.method)
                    .field("file", &document_uri)
                    .finish();
            }
        }
        match self {
            Event::Msg(it) => fmt::Debug::fmt(it, f),
            Event::Vfs(it) => fmt::Debug::fmt(it, f),
            Event::Task(it) => fmt::Debug::fmt(it, f),
        }
    }
}

pub fn main_loop(ws_root: PathBuf, config: Config, connection: &Connection) -> Result<()> {
    log::info!("starting example main loop");

    let mut loop_state = LoopState::default();
    let mut world_state = WorldState::new(ws_root, config);

    let pool = ThreadPool::default();
    let (task_sender, task_receiver) = unbounded::<Task>();

    log::info!("server initialized, serving requests");
    loop {
        let event = crossbeam_channel::select! {
            recv(&connection.receiver) -> message => match message {
                Ok(message) => Event::Msg(message),
                Err(_) => bail!("client exited without shutdown"),
            },
            recv(&task_receiver) -> task => Event::Task(task.unwrap()),
            recv(&world_state.fs_events_receiver) -> task => match task {
                Ok(task) => Event::Vfs(task),
                Err(_) => bail!("vfs died"),
            }
        };
        if let Event::Msg(Message::Request(req)) = &event {
            if connection.handle_shutdown(&req)? {
                break;
            }
        }
        loop_turn(
            &pool,
            &task_sender,
            &connection,
            &mut world_state,
            &mut loop_state,
            event,
        )?;
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct LoopState {
    next_request_id: u64,
    opened_files: OpenedFiles,
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
    pool: &ThreadPool,
    task_sender: &Sender<Task>,
    connection: &Connection,
    world_state: &mut WorldState,
    loop_state: &mut LoopState,
    event: Event,
) -> Result<()> {
    if matches!(event, Event::Msg(_)) {
        log::info!("loop turn = {:#?}", &event);
    }
    match event {
        Event::Task(task) => on_task(task, &connection.sender),
        Event::Vfs(vfs_task) => world_state.vfs.handle_task(vfs_task),
        Event::Msg(message) => match message {
            Message::Request(req) => {
                on_request(world_state, pool, task_sender, &connection.sender, req)?;
            }
            Message::Notification(not) => {
                on_notification(&connection.sender, world_state, loop_state, not)?;
            }
            Message::Response(resp) => {
                if Some(&resp.id) == loop_state.configuration_request_id.as_ref() {
                    loop_state.configuration_request_id = None;
                    log::info!("config update response: '{:?}", resp);
                    let Response { error, result, .. } = resp;

                    match (error, result) {
                        (Some(err), _) => {
                            log::error!("failed to fetch the server settings: {:?}", err)
                        }
                        (None, Some(configs)) => {
                            if let Some(new_config) = configs.get(0) {
                                let mut config = Config::default();
                                config.update(new_config);
                                *world_state =
                                    WorldState::new(world_state.ws_root.clone(), config);
                            }
                        }
                        (None, None) => {
                            log::error!("received empty server settings response from the client")
                        }
                    }
                }
            }
        },
    }
    let fs_state_changed = world_state.load_fs_changes();
    if fs_state_changed {
        log::info!("fs_state_changed = true, reextract diagnostics");
        update_file_notifications_on_threadpool(
            pool,
            world_state.analysis_host.analysis(),
            task_sender.clone(),
            loop_state.opened_files.files(),
        );
    }
    Ok(())
}

fn on_request(
    world_state: &mut WorldState,
    pool: &ThreadPool,
    task_sender: &Sender<Task>,
    msg_sender: &Sender<Message>,
    req: Request,
) -> Result<()> {
    let mut pool_dispatcher =
        PoolDispatcher::new(req, pool, world_state, msg_sender, task_sender);
    pool_dispatcher
        .on::<req::Completion>(handlers::handle_completion)?
        .finish();
    Ok(())
}

pub fn on_task(task: Task, msg_sender: &Sender<Message>) {
    match task {
        Task::Respond(response) => {
            msg_sender.send(response.into()).unwrap();
        }
        Task::Diagnostic(loc_ds) => {
            for loc_d in loc_ds {
                let uri = Url::from_file_path(loc_d.fpath).unwrap();
                let mut diagnostics = vec![];
                if loc_d.diagnostic.is_some() {
                    diagnostics.push(loc_d.diagnostic.unwrap());
                }
                let params = PublishDiagnosticsParams::new(uri, diagnostics, None);
                log::info!("Send diagnostic {:#?}", &params);

                let not = notification_new::<PublishDiagnostics>(params);
                msg_sender.send(not.into()).unwrap();
            }
        }
    }
}

fn on_notification(
    msg_sender: &Sender<Message>,
    world_state: &mut WorldState,
    loop_state: &mut LoopState,
    not: Notification,
) -> Result<()> {
    let not = match notification_cast::<DidOpenTextDocument>(not) {
        Ok(params) => {
            let uri = params.text_document.uri;
            let fpath = uri
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("invalid uri: {}", uri))?;
            let overlay = world_state
                .vfs
                .add_file_overlay(fpath.as_path(), params.text_document.text);
            assert!(
                overlay.is_some(),
                "Cannot find file {:?} in current roots",
                &fpath
            );
            loop_state
                .opened_files
                .add(leaked_fpath(fpath.to_str().unwrap()));
            return Ok(());
        }
        Err(not) => not,
    };
    let not = match notification_cast::<DidChangeTextDocument>(not) {
        Ok(mut params) => {
            let uri = params.text_document.uri;
            let fpath = uri
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("invalid uri: {}", uri))?;
            let text = params
                .content_changes
                .pop()
                .ok_or_else(|| anyhow::anyhow!("empty changes".to_string()))?
                .text;
            world_state.vfs.change_file_overlay(fpath.as_path(), text);
            loop_state
                .opened_files
                .add(leaked_fpath(fpath.to_str().unwrap()));
            return Ok(());
        }
        Err(not) => not,
    };
    let not = match notification_cast::<DidCloseTextDocument>(not) {
        Ok(params) => {
            let uri = params.text_document.uri;
            let fpath = uri
                .to_file_path()
                .map_err(|_| anyhow::anyhow!("invalid uri: {}", uri))?;
            loop_state
                .opened_files
                .remove(leaked_fpath(fpath.to_str().unwrap()));
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

fn update_file_notifications_on_threadpool(
    pool: &ThreadPool,
    analysis: Analysis,
    task_sender: Sender<Task>,
    files: Vec<FilePath>,
) {
    pool.execute(move || {
        log::info!("update_file_notifications_on_threadpool {:?}", files);
        for fpath in files {
            let text = analysis.db().tracked_files.get(fpath).unwrap();
            let mut diagnostics = analysis.check_with_libra_compiler(fpath, text);
            if diagnostics.is_empty() {
                diagnostics = vec![FileDiagnostic::new_empty(fpath)];
            }
            task_sender.send(Task::Diagnostic(diagnostics)).unwrap();
        }
    })
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

fn notification_is<N: lsp_types::notification::Notification>(
    notification: &Notification,
) -> bool {
    notification.method == N::METHOD
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
