use core::fmt;
use serde::export::Formatter;
use std::fmt::{Debug, Display};

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

pub struct ChainStateChanges {
    pub resource_changes: Vec<ResourceChange>,
    pub gas_spent: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceChange {
    pub account: String,
    pub ty: ResourceType,
    pub op: ResourceChangeOp,
}

impl ResourceChange {
    pub fn new(
        account: String,
        ty: impl Into<ResourceType>,
        op: impl Into<ResourceChangeOp>,
    ) -> ResourceChange {
        ResourceChange {
            account,
            ty: ty.into(),
            op: op.into(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ExecutionError {
    /// String representation of StatusCode enum.
    pub status: String,

    /// The optional sub status.
    pub sub_status: Option<u64>,

    /// The optional message.
    pub message: Option<String>,
}

impl Debug for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut formatted_struct = f.debug_struct("ExecutionError");
        formatted_struct.field("status", &self.status);

        if let Some(sub_status) = self.sub_status {
            formatted_struct.field("sub_status", &sub_status);
        }
        if let Some(message) = &self.message {
            formatted_struct.field("message", &message);
        }
        formatted_struct.finish()
    }
}

pub type ExecResult<T> = Result<T, ExecutionError>;

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:#?}", self))
    }
}

impl std::error::Error for ExecutionError {}
