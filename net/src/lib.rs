use anyhow::{anyhow, Result};
use http::Uri;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::vm_status::StatusCode;
use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMResult};
use dialect::Dialect;
use crate::dnode::DnodeNet;
use crate::pont::PontNet;

mod dnode;
mod pont;

pub type Block = String;

pub fn make_net<T>(uri: T, dialect: Dialect) -> Result<Box<dyn Net>>
where
    T: Into<Uri>,
{
    let uri = uri.into();
    match dialect {
        Dialect::Diem => Err(anyhow!("Unexpected dialect")),
        Dialect::DFinance => Ok(Box::new(DnodeNet { dialect, uri })),
        Dialect::Pont => Ok(Box::new(PontNet {
            api: uri.to_string(),
        })),
    }
}

#[derive(Debug)]
pub struct BytesForBlock(pub Vec<u8>, pub Block);

pub trait Net {
    fn get_module(
        &self,
        module_id: &ModuleId,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>>;
    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>>;
}

pub struct NetView {
    net: Box<dyn Net>,
    block: Option<Block>,
}

impl NetView {
    pub fn new(net: Box<dyn Net>, block: Option<Block>) -> NetView {
        NetView { net, block }
    }

    pub fn set_block(&mut self, block: Option<Block>) {
        self.block = block;
    }

    pub fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        self.net
            .get_module(module_id, &self.block)
            .map_err(|err| {
                PartialVMError::new(StatusCode::MISSING_DATA)
                    .with_message(err.to_string())
                    .finish(Location::Undefined)
            })
            .map(|bytes| bytes.map(|bytes| bytes.0))
    }

    pub fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        self.net
            .get_resource(address, tag, &self.block)
            .map_err(|err| {
                PartialVMError::new(StatusCode::MISSING_DATA).with_message(err.to_string())
            })
            .map(|bytes| bytes.map(|bytes| bytes.0))
    }
}
