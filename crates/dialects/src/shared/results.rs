use crate::lang::executor::Event;
use crate::shared::ProvidedAccountAddress;
use core::fmt;
use move_core_types::language_storage::{StructTag, TypeTag};
use serde::export::Formatter;
use std::collections::HashMap;
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
    // pub layout: Vec<String>,
}

impl ResourceType {
    pub fn new(type_tag: TypeTag) -> Self {
        match type_tag {
            TypeTag::Struct(struct_tag) => {
                let StructTag {
                    address,
                    module,
                    name,
                    type_params,
                } = struct_tag;
                ResourceType {
                    address: format!("0x{}", address),
                    module: module.to_string(),
                    name: name.to_string(),
                    ty_args: type_params
                        .into_iter()
                        .map(|ty| format!("{}", ty))
                        .collect(),
                }
            }
            _ => unreachable!("Resources are always structs"),
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.address, self.module, self.name)
    }
}

#[derive(Debug)]
pub struct ChainStateChanges {
    pub resource_changes: Vec<ResourceChange>,
    pub gas_spent: u64,
    pub events: Vec<Event>,
}

#[derive(Default, Debug)]
pub struct AddressMap {
    provided_addresses: Vec<ProvidedAccountAddress>,
}

impl AddressMap {
    pub fn insert(&mut self, address: ProvidedAccountAddress) {
        self.provided_addresses.push(address);
    }

    pub fn forward(&self) -> HashMap<String, String> {
        self.provided_addresses
            .clone()
            .into_iter()
            .map(|addresses| {
                let lowered = addresses.lowered();
                (addresses.original, lowered)
            })
            .collect()
    }

    pub fn reversed(&self) -> HashMap<String, String> {
        self.provided_addresses
            .clone()
            .into_iter()
            .map(|addresses| (addresses.lowered(), addresses.original))
            .collect()
    }
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

    pub fn with_replaced_addresses(
        self,
        address_map: &HashMap<String, String>,
    ) -> ResourceChange {
        let ResourceChange { account, ty, op } = self;

        let account = address_map.get(&account).unwrap_or(&account).to_owned();
        let ty_address = address_map
            .get(&ty.address)
            .unwrap_or(&ty.address)
            .to_owned();

        ResourceChange {
            account,
            ty: ResourceType {
                address: ty_address,
                ..ty
            },
            op,
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
