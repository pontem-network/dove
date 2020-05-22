use core::fmt;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ResourceChangeOp {
    SetValue { values: Vec<u8> },
    Delete,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ResourceType {
    pub address: String,
    pub module: String,
    pub name: String,
    pub ty_args: Vec<String>,
    pub layout: Vec<String>,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.address, self.module, self.name)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceChange {
    pub ty: ResourceType,
    pub op: ResourceChangeOp,
}

impl ResourceChange {
    pub fn new(ty: impl Into<ResourceType>, op: impl Into<ResourceChangeOp>) -> ResourceChange {
        ResourceChange {
            ty: ty.into(),
            op: op.into(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ExecutionError {
    /// String representation of StatusCode enum.
    pub status: String,

    /// The optional sub status.
    pub sub_status: Option<u64>,

    /// The optional message.
    pub message: Option<String>,
}

pub type ExecResult<T> = Result<T, ExecutionError>;

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ExecutionError {}
