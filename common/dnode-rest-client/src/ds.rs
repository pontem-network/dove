use serde::{Serialize, Deserialize};

/// Block number
pub type Block = u128;

/// Data Api response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// Current block number
    #[serde(deserialize_with = "block::deserialize")]
    pub height: Block,
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
