mod addresses;

use anyhow::Result;
use diem_move_core_types::account_address::AccountAddress as LibraAccountAddress;
use crate::compiler::dialects::Dialect;
use diem::move_core_types::gas_schedule::{CostTable};
use crate::compiler::source_map::FileOffsetMap;
use crate::compiler::dialects::libra::addresses::replace_libra_address;
use crate::compiler::address::ProvidedAccountAddress;
use std::ops::Deref;

#[derive(Default)]
pub struct LibraDialect;

impl Dialect for LibraDialect {
    fn name(&self) -> &str {
        "libra"
    }

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress> {
        let address = if addr.starts_with("0x") {
            LibraAccountAddress::from_hex_literal(addr)?
        } else {
            LibraAccountAddress::from_hex_literal(&format!("0x{}", addr))?
        };
        let normalized_address = format!("0x{}", address);
        let lowered = format!("0x00000000{}", address);
        Ok(ProvidedAccountAddress::new(
            addr.to_string(),
            normalized_address,
            lowered,
        ))
    }

    fn cost_table(&self) -> CostTable {
        diem::vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
            .deref()
            .clone()
    }

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileOffsetMap) -> String {
        replace_libra_address(source_text, source_map)
    }
}
