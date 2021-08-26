use anyhow::{anyhow, Result};
use http::Uri;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::vm_status::StatusCode;
use substrate_api_client::Api;
use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMResult};

use lang::compiler::dialects::Dialect as DialectTrait;
use lang::compiler::dialects::DialectName;

use crate::dnode::DnodeNet;
use crate::pont::PontNet;
use move_vm_runtime::data_cache::MoveStorage;

mod dnode;
mod pont;

pub type Block = String;

pub fn make_net<T>(uri: T, name: DialectName) -> Result<Box<dyn Net>>
where
    T: Into<Uri>,
{
    let uri = uri.into();
    match name {
        DialectName::Diem => Err(anyhow!("Unexpected dialect")),
        DialectName::DFinance => Ok(Box::new(DnodeNet {
            dialect: name.get_dialect(),
            uri,
        })),
        DialectName::Pont => Ok(Box::new(PontNet {
            dialect: name.get_dialect(),
            api: Api::new(uri.to_string()),
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
    fn dialect(&self) -> &dyn DialectTrait;
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
}

impl MoveStorage for NetView {
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        self.net
            .get_module(module_id, &self.block)
            .map_err(|err| {
                PartialVMError::new(StatusCode::MISSING_DATA)
                    .with_message(err.to_string())
                    .finish(Location::Undefined)
            })
            .map(|bytes| bytes.map(|bytes| bytes.0))
    }

    fn get_resource(
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
