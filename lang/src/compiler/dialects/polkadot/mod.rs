use crate::compiler::dialects::Dialect;
use crate::compiler::source_map::FileOffsetMap;
use move_core_types::account_address::AccountAddress;
use anyhow::Context;
use anyhow::Result;
use move_core_types::gas_schedule::CostTable;
use std::ops::Deref;
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::ss58::{replace_ss58_addresses, ss58_to_libra};

#[derive(Default)]
pub struct PolkadotDialect;

impl Dialect for PolkadotDialect {
    fn name(&self) -> &str {
        "polkadot"
    }

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress> {
        let address_res = if let Ok(libra_addr) = ss58_to_libra(addr) {
            Ok(ProvidedAccountAddress::new(
                addr.to_string(),
                addr.to_string(),
                libra_addr,
            ))
        } else if addr.starts_with("0x") {
            AccountAddress::from_hex_literal(addr).map(|address| {
                let lowered_addr = format!("0x{}", address);
                ProvidedAccountAddress::new(addr.to_string(), lowered_addr.clone(), lowered_addr)
            })
        } else {
            Err(anyhow::anyhow!(
                "Address is not valid libra or polkadot address"
            ))
        };
        address_res
            .with_context(|| format!("Address {:?} is not a valid libra/polkadot address", addr))
    }

    fn cost_table(&self) -> CostTable {
        // TODO: make compatible with Polkadot gas table.
        vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
            .deref()
            .clone()
    }

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileOffsetMap) -> String {
        replace_ss58_addresses(&source_text, source_map)
    }
}
