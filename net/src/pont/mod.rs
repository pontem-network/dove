mod client;

use anyhow::Result;
use move_core_types::language_storage::{ModuleId, CODE_TAG, StructTag};

use lang::compiler::dialects::Dialect;

use crate::{Block, Net, BytesForBlock};
use crate::pont::client::data_request_with;
use sp_core::H256;
use sp_keyring::sr25519::sr25519::Pair;
use std::str::FromStr;
use substrate_api_client::Api;
use move_core_types::account_address::AccountAddress;

pub struct PontNet {
    pub(crate) dialect: Box<dyn Dialect>,
    pub(crate) api: Api<Pair>,
}

impl Net for PontNet {
    fn get_module(
        &self,
        module_id: &ModuleId,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        let mut key = vec![CODE_TAG];
        key.append(&mut self.dialect.adapt_address_to_target(*module_id.address()));
        key.append(&mut bcs::to_bytes(&module_id.name).unwrap());

        let height = match height {
            None => None,
            Some(block) => Some(H256::from_str(block)?),
        };
        let bytecode = data_request_with(&self.api, key, height).ok();

        match bytecode {
            None => Ok(None),
            Some(mut bytes) => {
                self.dialect.adapt_to_basis(&mut bytes.0)?;
                Ok(Some(bytes))
            }
        }
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        let mut key = self.dialect.adapt_address_to_target(*address);
        key.append(&mut tag.access_vector());

        let height = match height {
            None => None,
            Some(block) => Some(H256::from_str(block)?),
        };

        Ok(data_request_with(&self.api, key, height).ok())
    }

    fn dialect(&self) -> &dyn Dialect {
        self.dialect.as_ref()
    }
}
