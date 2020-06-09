use anyhow::Result;

use crate::base::Dialect;
use crate::lang::gas::{dfinance_cost_table, libra_cost_table};
use crate::shared::bech32::bech32_into_libra;
use crate::shared::errors::FileSourceMap;
use crate::shared::ProvidedAccountAddress;
use anyhow::Context;
use libra_move_core_types::account_address::AccountAddress as LibraAccountAddress;
use move_core_types::gas_schedule::CostTable;

#[derive(Default)]
pub struct LibraDialect;

impl Dialect for LibraDialect {
    fn name(&self) -> &str {
        "libra"
    }

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress> {
        let address = LibraAccountAddress::from_hex_literal(&addr)?;
        let normalized_address = format!("0x{}", address);
        let lowered = format!("{}00000000", normalized_address);
        Ok(ProvidedAccountAddress::new(
            addr.to_string(),
            normalized_address,
            lowered,
        ))
    }

    fn cost_table(&self) -> CostTable {
        libra_cost_table()
    }

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String {
        crate::shared::addresses::replace_16_bytes_libra(source_text, source_map)
    }
}

#[derive(Default)]
pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn name(&self) -> &str {
        "dfinance"
    }

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress> {
        let lowered_libra_address = bech32_into_libra(addr).with_context(|| {
            format!("Address {:?} is not a valid wallet1 bech32 address", addr)
        })?;
        Ok(ProvidedAccountAddress::new(
            addr.to_string(),
            addr.to_string(),
            lowered_libra_address,
        ))
    }

    fn cost_table(&self) -> CostTable {
        dfinance_cost_table()
    }

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String {
        crate::shared::bech32::replace_bech32_addresses(&source_text, source_map)
    }
}
