#[macro_use]
extern crate serde;
#[macro_use]
extern crate wasm_bindgen;

use std::fmt::{Display, Formatter};

use crate::file::*;
use crate::project::*;

pub mod file;
pub mod project;

#[derive(Debug, Deserialize, Serialize)]
pub struct Empty;

transport! {
    // Returns projects list.
    ProjectList|project_list: Empty => ProjectList;
    // Returns project info by its id.
    ProjectInfo|project_info: Id => ProjectInfo;

    // Returns file by file id.
    GetFile|get_file: FileIdentifier => File;
    // Remove file by file id.
    RemoveFile|remove_file: FileIdentifier => Empty;
    // Create a new file.
    CreateFile|create_file: CreateFsEntry => CreateFileResult;
    // Create a new directory.
    CreateDirectory|create_directory: CreateFsEntry => ProjectInfo;
    // Rename file and return new file id. Flush before rename;
    RenameFile|rename_file: RenameFile => FId;
    // Rename directory. Flush before rename;
    RenameDirectory|rename_directory: RenameDirectory => ProjectInfo;
    // Remove directory.
    RemoveDirectory|remove_directory: RemoveDirectory => ProjectInfo;
    // Flush changes.
    Flush|flush: Flush => FlushResult;
    // Reload project from disk.
    Sync|sync_project: Id => ProjectInfo;
    // clean|build|test project
    ProjectActionRequest|project_action: ProjectActionRequest => ProjectActionResponse;
}

#[macro_export]
macro_rules! transport {
    ($($name:ident|$fun_name:ident : $req:ident => $resp:ident;)*) => {

        #[derive(Debug, Serialize)]
        pub enum SentRequest<'a> {
             $(
                $name(&'a $req),
             )*
        }

        #[derive(Debug, Deserialize)]
        pub enum ReceivedRequest {
             $(
                $name($req),
             )*
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub enum Response {
             $(
                $name($resp),
             )*
        }

        pub trait OnRequest {
            $(
             fn $fun_name(&self, req: $req) -> Result<$resp, anyhow::Error>;
             )*

             fn handle(&self, buff: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
                let resp = bcs::from_bytes::<ReceivedRequest>(buff)
                    .map_err(|err| err.into())
                    .and_then(|req| {
                        match req {
                        $(
                            ReceivedRequest::$name(req) => self.$fun_name(req)
                            .map(Response::$name),
                        )*
                        }
                     })
                    .map_err(Error::from);

                Ok(bcs::to_bytes(&resp)?)
            }
        }

        pub async fn perform(url: &str, req: &SentRequest<'_>) -> Result<Response, anyhow::Error> {
            let resp = reqwest::Client::builder().build()?
                .post(url)
                .body(bcs::to_bytes(req)?)
                .send().await?;
            if resp.status().is_success() {
                bcs::from_bytes::<Result<Response, Error>>(resp.bytes().await?.as_ref())?
                .map_err(|err| anyhow::Error::msg(err.msg))
            } else {
                Err(anyhow::anyhow!("Failed to perform http request:{}", resp.status().as_str()))
            }
        }

        $(
            pub async fn $fun_name(url: &str, req: &$req) -> Result<$resp, anyhow::Error> {
                let req = SentRequest::$name(req);
                let resp = perform(url, &req).await?;
                match resp {
                    Response::$name(val) => Ok(val),
                    _ => anyhow::bail!("Type mismatch."),
                }
            }
        )*
    };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub msg: String,
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error {
            msg: err.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
