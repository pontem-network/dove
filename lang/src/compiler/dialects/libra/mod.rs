mod addresses;

use anyhow::Result;
use libra_move_core_types::account_address::AccountAddress as LibraAccountAddress;
use crate::compiler::dialects::Dialect;
use move_core_types::gas_schedule::{CostTable, GasCost, GasConstants};
use crate::compiler::source_map::FileSourceMap;
use crate::compiler::dialects::libra::addresses::replace_libra_address;
use crate::compiler::address::ProvidedAccountAddress;

#[derive(Default)]
pub struct LibraDialect;

impl Dialect for LibraDialect {
    fn name(&self) -> &str {
        "libra"
    }

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress> {
        let address = LibraAccountAddress::from_hex_literal(&addr)?;
        let normalized_address = format!("0x{}", address);
        let lowered = format!("0x00000000{}", address);
        Ok(ProvidedAccountAddress::new(
            addr.to_string(),
            normalized_address,
            lowered,
        ))
    }

    fn cost_table(&self) -> CostTable {
        let instructions_table_bytes = vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
            .0
            .clone();
        let instruction_table: Vec<GasCost> = lcs::from_bytes(&instructions_table_bytes).unwrap();

        let native_table_bytes = vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
            .1
            .clone();
        let native_table: Vec<GasCost> = lcs::from_bytes(&native_table_bytes).unwrap();

        CostTable {
            instruction_table,
            native_table,
            gas_constants: GasConstants::default(),
        }
    }

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String {
        replace_libra_address(source_text, source_map)
    }
}
