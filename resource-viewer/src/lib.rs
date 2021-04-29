extern crate anyhow;

use std::str::FromStr;

use anyhow::{Error, anyhow};

use lang::compiler::dialects::{dfinance::DFinanceDialect, DialectName, pontem::PontDialect};

use lang::compiler::dialects::Dialect as DialectTrait;

pub mod ser;
mod sp_client;
pub mod tte;

pub mod net {
    #[cfg(any(feature = "dfinance_address", feature = "libra_address"))]
    pub use dnclient::blocking::{client::DnodeRestClient as NodeClient, get_resource};

    #[cfg(feature = "ps_address")]
    pub use super::sp_client::*;
}

pub enum Dialect {
    Pont(PontDialect),
    Dfinance(DFinanceDialect),
}

impl AsRef<dyn DialectTrait> for Dialect {
    fn as_ref(&self) -> &dyn DialectTrait {
        match self {
            Dialect::Pont(d) => d,
            Dialect::Dfinance(d) => d,
        }
    }
}

impl FromStr for Dialect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match DialectName::from_str(s)? {
            DialectName::Diem => Err(anyhow!("Unexpected dialect")),
            DialectName::DFinance => Ok(Dialect::Dfinance(DFinanceDialect)),
            DialectName::Pont => Ok(Dialect::Pont(PontDialect)),
        }
    }
}
