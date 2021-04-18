use std::str::FromStr;

use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::CostTable;

use crate::compiler::dialects::dfinance::DFinanceDialect;
use crate::compiler::dialects::diem::DiemDialect;
use crate::compiler::dialects::pontem::PontemDialect;
use crate::compiler::mut_string::MutString;
use crate::compiler::source_map::FileOffsetMap;

pub mod dfinance;
pub mod diem;
pub mod line_endings;
pub mod pontem;

pub trait Dialect {
    fn name(&self) -> &str;

    /// Returns the bytecode in the dialect format.
    fn adapt_to_target(&self, bytecode: &mut Vec<u8>) -> Result<()>;

    /// Returns the bytecode in the basis format.
    fn adapt_to_basis(&self, bytecode: &mut Vec<u8>) -> Result<()>;

    fn normalize_account_address(&self, addr: &str) -> Result<AccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(
        &self,
        source_text: &str,
        mut_str: &mut MutString,
        source_map: &mut FileOffsetMap,
    );
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DialectName {
    Libra,
    DFinance,
    Polkadot,
}

impl DialectName {
    pub fn get_dialect(&self) -> Box<dyn Dialect> {
        match self {
            DialectName::Libra => Box::new(DiemDialect::default()),
            DialectName::DFinance => Box::new(DFinanceDialect::default()),
            DialectName::Polkadot => Box::new(PontemDialect::default()),
        }
    }
}

impl FromStr for DialectName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "libra" => Ok(DialectName::Libra),
            "dfinance" => Ok(DialectName::DFinance),
            "polkadot" => Ok(DialectName::Polkadot),
            _ => Err(anyhow::format_err!("Invalid dialect {:?}", s)),
        }
    }
}
