use anyhow::Result;

use crate::base::Dialect;
use crate::impls::{DFinanceDialect, LibraDialect};

use serde::export::fmt::Debug;

use std::str::FromStr;

pub mod base;
pub mod gas;
pub mod impls;
pub mod lang;
pub mod shared;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DialectName {
    Libra,
    DFinance,
}

impl DialectName {
    pub fn get_dialect(&self) -> Box<dyn Dialect> {
        match self {
            DialectName::Libra => Box::new(LibraDialect::default()),
            DialectName::DFinance => Box::new(DFinanceDialect::default()),
        }
    }
}

impl FromStr for DialectName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "libra" => Ok(DialectName::Libra),
            "dfinance" => Ok(DialectName::DFinance),
            _ => Err(anyhow::format_err!("Invalid dialect {:?}", s)),
        }
    }
}
