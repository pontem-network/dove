use anyhow::Result;
use move_core_types::gas_schedule::CostTable;
use crate::shared::errors::FileSourceMap;
use crate::shared::ProvidedAccountAddress;

pub trait Dialect {
    fn name(&self) -> &str;

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String;
}
