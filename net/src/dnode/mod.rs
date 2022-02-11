#![cfg(feature = "dfinance")]

mod client;
use anyhow::Result;
use url::Url;

use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::account_address::AccountAddress;

use crate::{Net, Block, BytesForBlock};
use crate::dnode::client::data_request;

pub struct DnodeNet {
    pub(crate) uri: Url,
}

impl Net for DnodeNet {
    fn get_module(
        &self,
        module_id: &ModuleId,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        // does not work without dialect
        // let address = self.dialect.adapt_address_to_target(*module_id.address());
        // remove this
        let address = module_id.address().into_bytes();

        let bytes = data_request(&address, &module_id.access_vector(), &self.uri, height).ok();
        match bytes {
            None => Ok(None),
            Some(bytes) => {
                // does not work without dialect
                // self.dialect.adapt_to_basis(&mut bytes.0)?;
                Ok(Some(BytesForBlock(bytes.0, bytes.1)))
            }
        }
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        // does not work without dialect
        // let address = self.dialect.adapt_address_to_target(*address);
        // remove this
        let address = address.into_bytes();

        let access_vector = tag.access_vector();
        let bytes = data_request(&address, &access_vector, &self.uri, height).ok();
        match bytes {
            None => Ok(None),
            Some(bytes) => {
                // does not work without dialect
                // self.dialect.adapt_to_basis(&mut bytes.0)?;
                Ok(Some(BytesForBlock(bytes.0, bytes.1)))
            }
        }
    }
}
