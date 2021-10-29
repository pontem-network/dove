use std::fmt::{Display, Formatter};
use anyhow::{bail, Result};
use serde::{Serialize, Deserialize};
use lang::compiler::dialects::Dialect;
use lang::compiler::address::ss58::address_to_ss58;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::account_address::AccountAddress;
use crate::{Net, BytesForBlock};

pub type Block = String;

pub struct PontNet {
    pub(crate) dialect: Box<dyn Dialect>,
    pub(crate) api: String,
}

impl Net for PontNet {
    fn get_module(
        &self,
        module_id: &ModuleId,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        let req = Request {
            id: 1,
            jsonrpc: "2.0",
            method: "mvm_getModule",
            params: vec![format!("0x{}", hex::encode(bcs::to_bytes(module_id)?))],
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let response = reqwest::blocking::Client::new()
            .post(&self.api)
            .headers(headers)
            .json(&req)
            .send()?;

        if response.status() != 200 {
            bail!(
                "Failed to get module :{}. Error:{}",
                module_id,
                response.status()
            );
        }
        let resp = response.json::<Response>()?;
        if let Some(err) = resp.error {
            bail!("{:?}", err);
        }
        if let Some(result) = resp.result {
            let result = hex::decode(&result[2..])?;
            Ok(Some(BytesForBlock(
                result,
                height.clone().unwrap_or_default(),
            )))
        } else {
            Ok(None)
        }
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
        height: &Option<Block>,
    ) -> Result<Option<BytesForBlock>> {
        let req = Request {
            id: 1,
            jsonrpc: "2.0",
            method: "mvm_getResource",
            params: vec![
                address_to_ss58(address),
                format!("0x{}", hex::encode(bcs::to_bytes(&tag)?)),
            ],
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let response = reqwest::blocking::Client::new()
            .post(&self.api)
            .headers(headers)
            .json(&req)
            .send()?;
        if response.status() != 200 {
            bail!(
                "Failed to get resource :{:?} {:?}. Error:{}",
                &address,
                &tag,
                response.status()
            );
        }

        let resp = response.json::<Response>()?;
        if let Some(err) = resp.error {
            bail!("{:?}", err);
        }
        if let Some(result) = resp.result {
            let result = hex::decode(&result[2..])?;
            Ok(Some(BytesForBlock(
                result,
                height.clone().unwrap_or_default(),
            )))
        } else {
            Ok(None)
        }
    }

    fn dialect(&self) -> &dyn Dialect {
        self.dialect.as_ref()
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use lang::compiler::dialects::DialectName;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::{ModuleId, StructTag};
    use super::PontNet;
    use crate::Net;
    use lang::compiler::address::ss58::ss58_to_address;

    /// If the node is raised to "localhost:9933".
    #[ignore]
    #[test]
    fn test_get_module() {
        let dialect_name = DialectName::from_str("pont").unwrap();
        let api = PontNet {
            dialect: dialect_name.get_dialect(),
            api: "http://localhost:9933".to_string(),
        };
        let module = api
            .get_module(
                &ModuleId {
                    address: AccountAddress::from_hex_literal("0x1").unwrap(),
                    name: Identifier::new("Hash").unwrap(),
                },
                &None,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            [
                161, 28, 235, 11, 2, 0, 0, 0, 6, 1, 0, 2, 3, 2, 10, 5, 12, 3, 7, 15, 23, 8, 38,
                32, 12, 70, 8, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 1, 10, 2, 4, 72, 97, 115, 104,
                8, 115, 104, 97, 50, 95, 50, 53, 54, 8, 115, 104, 97, 51, 95, 50, 53, 54, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 1, 2, 0, 1, 1, 2, 0, 0
            ],
            module.0.as_slice()
        );
    }

    /// If the node is raised to "localhost:9933"
    ///     and there is a resource on "5grwvaef5zxb26fz9rcqpdws57cterhpnehxcpcnohgkutqy::Store::U64".
    #[ignore]
    #[test]
    fn test_get_resource() {
        let dialect_name = DialectName::from_str("pont").unwrap();
        let api = PontNet {
            dialect: dialect_name.get_dialect(),
            api: "http://localhost:9933".to_string(),
        };

        let adr = ss58_to_address("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
        let module = api
            .get_resource(
                &adr,
                &StructTag {
                    address: adr,
                    module: Identifier::new("Store").unwrap(),
                    name: Identifier::new("U64").unwrap(),
                    type_params: vec![],
                },
                &None,
            )
            .unwrap()
            .unwrap();
        assert_eq!(module.0, [100, 0, 0, 0, 0, 0, 0, 0]);
    }
}
