mod client;

use std::convert::TryFrom;
use http::Uri;
use anyhow::Result;
use dialect::Dialect;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::account_address::AccountAddress;
use move_compat::{adapt_address_to_target, AddressType, adapt_to_basis};
use crate::{Net, Block, BytesForBlock};
use crate::dnode::client::data_request;

pub struct DnodeNet {
    pub(crate) dialect: Dialect,
    pub(crate) uri: Uri,
}

impl Net for DnodeNet {
    fn get_module(
        &self,
        module_id: &ModuleId,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        let address_type = AddressType::try_from(&self.dialect)?;
        let address = adapt_address_to_target(*module_id.address(), address_type);
        let bytes = data_request(&address, &module_id.access_vector(), &self.uri, height).ok();
        match bytes {
            None => Ok(None),
            Some(mut bytes) => {
                adapt_to_basis(&mut bytes.0, address_type)?;
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
        let address_type = AddressType::try_from(&self.dialect)?;
        let address = adapt_address_to_target(*address, address_type);
        let access_vector = tag.access_vector();
        let bytes = data_request(&address, &access_vector, &self.uri, height).ok();
        match bytes {
            None => Ok(None),
            Some(mut bytes) => {
                adapt_to_basis(&mut bytes.0, address_type)?;
                Ok(Some(BytesForBlock(bytes.0, bytes.1)))
            }
        }
    }
}
