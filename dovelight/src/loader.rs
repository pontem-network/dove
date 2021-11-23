use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::{Error, anyhow, bail};
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag};
use move_core_types::account_address::AccountAddress;
use lang::compiler::address::ss58::address_to_ss58;
use lang::tx::parser::parse_tp_param;
use crate::lang::deps::DependencyLoader;
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

/// Generate a JSON request to get a resource
pub fn get_request_resource_by_xpath(account: &str, xpath: &str) -> Result<Value, Error> {
    let account = AccountAddress::from_hex_literal(account)?;
    let tag = pars_xpath(xpath)?;
    let req = Request {
        id: 1,
        jsonrpc: "2.0",
        method: "mvm_getResource",
        params: vec![
            address_to_ss58(&account),
            format!("0x{}", hex::encode(bcs::to_bytes(&tag)?)),
        ],
    };

    Ok(serde_json::to_value(&req)?)
}

fn pars_xpath(xpath: &str) -> Result<StructTag, Error> {
    match parse_tp_param(xpath)? {
        TypeTag::Struct(tag) => Ok(tag),
        _ => bail!("The parameter is set incorrectly {xpath:?}. Expected: ADDRESS::MODEL_NAME::RESOURCE_NAME<TYPE_1, TYPE_2...TYPE_N>", xpath = xpath),
    }
}

#[test]
fn test_get_request_resource_by_xpath() {
    let mut tag =
        get_request_resource_by_xpath("0x1", "0x1::Account::Balance<0x1::Coins::ETH>").unwrap();

    assert_eq!(
        r#"{"id":1,"jsonrpc":"2.0","method":"mvm_getResource","params":["5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUqAsg","0x0000000000000000000000000000000000000000000000000000000000000001074163636f756e740742616c616e63650107000000000000000000000000000000000000000000000000000000000000000105436f696e730345544800"]}"#,
        &tag
    );

    tag = get_request_resource_by_xpath("0x1", "0x1::Account::Balance").unwrap();

    assert_eq!(
        r#"{"id":1,"jsonrpc":"2.0","method":"mvm_getResource","params":["5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUqAsg","0x0000000000000000000000000000000000000000000000000000000000000001074163636f756e740742616c616e636500"]}"#,
        &tag
    );
}
