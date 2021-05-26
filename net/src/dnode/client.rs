use log::{trace, debug};
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};
use http::Uri;
use crate::{Block, BytesForBlock};

pub fn data_request(
    address: &[u8],
    path: &[u8],
    url: &Uri,
    height: &Option<Block>,
) -> Result<BytesForBlock> {
    let url = format!(
        "{base_url}vm/data/{address}/{path}{height}",
        base_url = url,
        address = hex::encode(address),
        path = hex::encode(path),
        height = height
            .as_ref()
            .map(|i| format!("?height={}", i))
            .unwrap_or_default()
    );

    trace!("req: {}", url);

    let resp = reqwest::blocking::get(&url)?;
    let status = resp.status();
    let res: Response = resp.json()?;
    debug!("res: ({}) {:#?}", status, res);

    match res.body {
        ResponseBody::Result { value } => {
            Ok(BytesForBlock(hex::decode(&value)?, res.height.to_string()))
        }
        ResponseBody::Error { message } => Err(anyhow!(
            "Failed to load:'{}' ({}) [{}]",
            url,
            status,
            message
        )),
    }
}

/// Data Api response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// Current block number
    #[serde(deserialize_with = "block::deserialize")]
    pub height: String,
    #[serde(flatten)]
    pub body: ResponseBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResponseBody {
    /// Success response
    Result {
        /// Hex encoded bytecode
        value: String,
    },

    ///Error response
    Error {
        #[serde(rename = "error")]
        message: String,
    },
}

mod block {
    use super::Block;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
