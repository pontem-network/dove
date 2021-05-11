use log::*;
use keyring::sr25519::sr25519::Pair;
use substrate_api_client::Api;
use anyhow::{Error, Result};
use http::Uri;
use move_core_types::language_storage::ModuleId;
use crate::BytesForBlock;

/// Block number
pub type Block = sp_core::H256;

pub const MODULE: &str = "Mvm";
pub const STORAGE: &str = "VMStorage";

pub fn data_request_with(
    client: &Api<Pair>,
    path: Vec<u8>,
    height: Option<Block>,
) -> Result<BytesForBlock> {
    debug!("data request: path: {:?}", path);

    let storagekey = client
        .metadata
        .storage_map_key::<Vec<u8>, Vec<u8>>(MODULE, STORAGE, path)
        .unwrap();

    let height = height
        .or_else(|| {
            trace!("request actual height");
            client.get_finalized_head()
        })
        .ok_or_else(|| Error::msg("Cannot get finalized head"))?;
    debug!("height: {:?}", height);

    debug!("storage key: 0x{}", hex::encode(storagekey.0.clone()));
    let result: Option<Vec<u8>> = client.get_storage_by_key_hash(storagekey, Some(height));
    debug!("data: {:?}", result);

    result
        .ok_or_else(|| Error::msg("not found"))
        .map(|result| BytesForBlock(result, height.to_string()))
}

#[allow(dead_code)]
pub fn get_module(
    module_id: &ModuleId,
    host: Uri,
    height: Option<Block>,
) -> Result<BytesForBlock> {
    let mut client = Api::new(host.to_string());
    get_module_with(&mut client, module_id, height)
}

pub fn get_module_with(
    client: &mut Api<Pair>,
    module_id: &ModuleId,
    height: Option<Block>,
) -> Result<BytesForBlock> {
    // same as AccessPath::code_access_path(module_id)
    let path = module_id.access_vector();
    debug!("get module: {} path: {:?}", module_id, path);
    let res = data_request_with(client, path, height);
    debug!("get module result: {:?}", res);
    res
}
