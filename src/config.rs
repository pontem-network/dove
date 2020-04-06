use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MoveDialect {
    Libra,
    DFinance,
}

impl Default for MoveDialect {
    fn default() -> Self {
        MoveDialect::Libra
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ServerConfig {
    pub dialect: MoveDialect,
    pub stdlib_path: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            dialect: MoveDialect::default(),
            stdlib_path: None,
        }
    }
}
