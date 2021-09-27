use crate::deps::DependencyLoader;
use move_core_types::language_storage::ModuleId;
use anyhow::{Error, anyhow};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use web_sys::{XmlHttpRequest, Blob, BlobPropertyBag};
use wasm_bindgen::JsValue;

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

        let err = |err| anyhow!("Failed to send http request:{:?}", err);
        let xhr = XmlHttpRequest::new().map_err(err)?;
        xhr.open_with_async("POST", &self.chain_api, false)
            .map_err(err)?;
        xhr.set_request_header("Content-Type", "application/json")
            .map_err(err)?;

        let blob = Blob::new_with_str_sequence_and_options(
            &JsValue::from_serde(&vec![serde_json::to_string(&req)?])?,
            &BlobPropertyBag::new().type_("application/json"),
        )
        .map_err(err)?;
        xhr.send_with_opt_blob(Some(&blob)).map_err(err)?;
        let status = xhr.status().map_err(err)?;
        if status != 200 {
            return Err(anyhow!(
                "Failed to get module :{}. Error:{}-{:?}",
                id,
                status,
                xhr.status_text().map_err(err)
            ));
        }
        let resp: Response =
            serde_json::from_str(&xhr.response_text().map_err(err)?.unwrap_or_default())?;
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
