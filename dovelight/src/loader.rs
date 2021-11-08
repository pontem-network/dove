use crate::lang::deps::DependencyLoader;
use move_core_types::language_storage::ModuleId;
use anyhow::{Error, anyhow};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use crate::env::http::http_request;
use crate::env::http::Request as HttpRequest;

#[derive(Clone)]
pub struct Loader {
    chain_api: String,
}

impl Loader {
    pub fn new(chain_api: String) -> Loader {
        Loader { chain_api }
    }
}

impl DependencyLoader for Loader {
    fn get_module(&self, id: &ModuleId) -> Result<Vec<u8>, Error> {
        let req = Request {
            id: 1,
            jsonrpc: "2.0",
            method: "mvm_getModule",
            params: vec![format!("0x{}", hex::encode(bcs::to_bytes(id)?))],
        };

        let resp = http_request(HttpRequest {
            method: "POST".to_string(),
            url: self.chain_api.to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some(serde_json::to_string(&req)?),
        })?;

        if resp.status != 200 {
            return Err(anyhow!(
                "Failed to get module :{}. Error:{}-{:?}",
                id,
                resp.status,
                resp.response
            ));
        }

        let resp: Response = serde_json::from_str(&resp.response)?;
        if let Some(err) = resp.error {
            return Err(Error::msg(err));
        }
        resp.result
            .ok_or_else(|| anyhow!("Module with id '{}' does not exist.", id))
            .and_then(|val| hex::decode(&val[2..]).map_err(Error::new))
    }
}

#[derive(Serialize)]
struct Request {
    id: u64,
    jsonrpc: &'static str,
    method: &'static str,
    params: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Response {
    id: u64,
    jsonrpc: String,
    result: Option<String>,
    error: Option<ErrorMsg>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ErrorMsg {
    code: Option<i64>,
    message: Option<String>,
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.message.as_ref() {
            write!(f, "{}; Code:{}", msg, self.code.unwrap_or_default())
        } else {
            write!(f, "Code:{}", self.code.unwrap_or_default())
        }
    }
}
