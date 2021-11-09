use std::str::FromStr;

use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::CostTable;

use crate::compiler::dialects::dfinance::DFinanceDialect;
use crate::compiler::dialects::diem::DiemDialect;
use crate::compiler::dialects::pontem::PontDialect;
use crate::compiler::mut_string::MutString;
use crate::compiler::source_map::FileOffsetMap;
use std::fmt;

pub mod dfinance;
pub mod diem;
pub mod line_endings;
pub mod pontem;

pub trait Dialect {
    /// Returns maximum number of bytes in the address.
    fn address_length(&self) -> usize;

    /// Returns dialect name.
    fn name(&self) -> DialectName;

    /// Returns the bytecode in the dialect format.
    fn adapt_to_target(&self, bytecode: &mut Vec<u8>) -> Result<()>;

    /// Returns the bytecode in the basis format.
    fn adapt_to_basis(&self, bytecode: &mut Vec<u8>) -> Result<()>;

    fn adapt_address_to_target(&self, address: AccountAddress) -> Vec<u8>;

    fn adapt_address_to_basis(&self, address: &[u8]) -> Result<AccountAddress>;

    fn parse_address(&self, addr: &str) -> Result<AccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(
        &self,
        source_text: &str,
        mut_str: &mut MutString,
        source_map: &mut FileOffsetMap,
    );

    fn copy(&self) -> Box<dyn Dialect>;
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DialectName {
    Diem,
    DFinance,
    Pont,
}

impl DialectName {
    pub fn get_dialect(&self) -> Box<dyn Dialect> {
        match self {
            DialectName::Diem => Box::new(DiemDialect::default()),
            DialectName::DFinance => Box::new(DFinanceDialect::default()),
            DialectName::Pont => Box::new(PontDialect::default()),
        }
    }
}

impl FromStr for DialectName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "diem" => Ok(DialectName::Diem),
            "dfinance" => Ok(DialectName::DFinance),
            "pont" => Ok(DialectName::Pont),
            _ => Err(anyhow::format_err!("Invalid dialect {:?}", s)),
        }
    }
}

impl fmt::Display for DialectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DialectName::Diem => "diem",
                DialectName::DFinance => "dfinance",
                DialectName::Pont => "pont",
            }
        )
    }
}
