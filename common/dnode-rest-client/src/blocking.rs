use http::Uri;
use anyhow::Result;
use libra::prelude::*;
use crate::ds::*;

pub type BlockOpt = Option<Block>;

pub struct BytesForBlock(Vec<u8>, Block);

impl BytesForBlock {
    pub fn block(&self) -> u128 {
        self.1
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

pub fn get_module<'a, T>(module_id: &ModuleId, url: T, height: BlockOpt) -> Result<BytesForBlock>
where
    T: Into<&'a Uri>,
{
    let path = AccessPath::code_access_path(module_id);
    data_request(&path, url, height)
}

pub fn get_resource<'a, T>(res: &ResourceKey, url: T, height: BlockOpt) -> Result<BytesForBlock>
where
    T: Into<&'a Uri>,
{
    let path = AccessPath::resource_access_path(res);
    data_request(&path, url, height)
}

pub fn data_request<'a, T>(path: &AccessPath, url: T, height: BlockOpt) -> Result<BytesForBlock>
where
    T: Into<&'a Uri>,
{
    let url = format!(
        "{base_url}vm/data/{address}/{path}{height}",
        base_url = url.into(),
        address = hex::encode(&path.address),
        path = hex::encode(&path.path),
        height = height.map(|i| format!("?height={}", i)).unwrap_or_default()
    );

    trace!("req: {} : {}", url, &path);

    let resp = reqwest::blocking::get(&url)?;
    let status = resp.status();
    let res: Response = resp.json()?;
    let height = res.height;

    debug!("res: ({}) {:#?}", status, res);

    match res.body {
        ResponseBody::Result { value } => {
            Ok(hex::decode(&value).map(|v| BytesForBlock(v, height))?)
        }
        ResponseBody::Error { message } => Err(anyhow!(
            "Failed to load:'{}' ({}) [{}]",
            url,
            status,
            message
        )),
    }
}

pub mod client {
    use super::*;
    use libra::move_core_types::language_storage::StructTag;

    pub struct DnodeRestClient {
        uri: Uri,
        height: BlockOpt,
    }

    impl DnodeRestClient {
        pub fn new<T: Into<Uri>>(uri: T, height: BlockOpt) -> Self {
            Self {
                uri: uri.into(),
                height,
            }
        }
    }

    impl RemoteCache for DnodeRestClient {
        fn get_module(&self, id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
            let res = get_module(id, &self.uri, self.height).map(|v| v.0).ok();
            if res.is_some() && res.as_ref().unwrap().is_empty() {
                error!("Empty module for {}", id);
            }
            Ok(res)
        }

        fn get_resource(
            &self,
            addr: &AccountAddress,
            tag: &StructTag,
        ) -> PartialVMResult<Option<Vec<u8>>> {
            let key = ResourceKey::new(*addr, tag.to_owned());
            let res = get_resource(&key, &self.uri, self.height).map(|v| v.0).ok();
            if res.is_some() && res.as_ref().unwrap().is_empty() {
                error!("Empty resource for {:?}", key);
            }
            Ok(res)
        }
    }
}
