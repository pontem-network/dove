use serde::{Serialize, Deserialize};

/// Data Api response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// Current block number
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
