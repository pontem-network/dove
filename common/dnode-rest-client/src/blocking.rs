use http::Uri;
use anyhow::Result;
use libra::prelude::*;
use crate::ds::*;

pub fn get_module<'a, T: Into<&'a Uri>>(module_id: &ModuleId, url: T) -> Result<Vec<u8>> {
    let path = AccessPath::code_access_path(module_id);
    data_request(&path, url)
}

pub fn get_resource<'a, T: Into<&'a Uri>>(res: &ResourceKey, url: T) -> Result<Vec<u8>> {
    let path = AccessPath::resource_access_path(res);
    data_request(&path, url)
}

pub fn data_request<'a, T: Into<&'a Uri>>(path: &AccessPath, url: T) -> Result<Vec<u8>> {
    let url = format!(
        "{base_url}vm/data/{address}/{path}",
        base_url = url.into(),
        address = hex::encode(&path.address),
        path = hex::encode(&path.path)
    );

    trace!("req: {} : {}", url, &path);

    let resp = reqwest::blocking::get(&url)?;
    let status = resp.status();
    let res: Response = resp.json()?;

    trace!("res: ({}) {:#?}", status, res);

    match res.body {
        ResponseBody::Result { value } => Ok(hex::decode(&value)?),
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
    use libra::vm::StructTag;

    pub struct DnodeRestClient {
        uri: Uri,
    }

    impl DnodeRestClient {
        pub fn new<T: Into<Uri>>(uri: T) -> Self {
            Self { uri: uri.into() }
        }
    }

    impl RemoteCache for DnodeRestClient {
        fn get_module(&self, id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
            let res = get_module(id, &self.uri).ok();
            if res.is_some() && res.as_ref().unwrap().is_empty() {
                error!("Err: empty module for {}", id);
            }
            Ok(res)
        }

        fn get_resource(
            &self,
            addr: &AccountAddress,
            tag: &StructTag,
        ) -> PartialVMResult<Option<Vec<u8>>> {
            let key = ResourceKey::new(*addr, tag.to_owned());
            let res = get_resource(&key, &self.uri).ok();
            if res.is_some() && res.as_ref().unwrap().is_empty() {
                error!("Err: empty resource for {:?}", key);
            }
            Ok(res)
        }
    }
}
