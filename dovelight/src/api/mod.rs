#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod js;
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub mod wasi;

use std::str::FromStr;
use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use serde::{Serialize, Deserialize};
use lang::compiler::dialects::DialectName;
use lang::tx::fn_call::Config;
use crate::lang::abi::make_module_abi;
use crate::lang::compiler;
use crate::lang::compiler::source_map::SourceMap;
use crate::lang::deps::index::id_to_str;
use crate::lang::deps::resolver::DependencyResolver;
use crate::loader::Loader;
use crate::storage::EnvStorage;
use crate::lang::abi::ModuleAbi;
use crate::lang::tx::Context;
use crate::lang::tx;

#[derive(Serialize, Deserialize, Debug)]
pub struct Units {
    pub units: Vec<Unit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub bytecode: Vec<u8>,
}

pub fn build(
    chain_api: String,
    source_map: SourceMap,
    dialect: String,
    sender: String,
) -> Result<Units, Error> {
    let cache = EnvStorage::new_in_family("dove_cache_")?;
    let loader = Loader::new(chain_api);

    let units = compiler::build(loader, cache, source_map, &dialect, &sender)?
        .into_iter()
        .map(|(name, bytecode)| Unit { name, bytecode })
        .collect();
    Ok(Units { units })
}

pub fn module_abi(
    chain_api: String,
    dialect: String,
    address: String,
    module_name: String,
) -> Result<ModuleAbi, Error> {
    let store = EnvStorage::new_in_family("dove_cache_")?;
    let loader = Loader::new(chain_api);
    let dialect = DialectName::from_str(&dialect)?.get_dialect();
    let resolver = DependencyResolver::new(dialect.as_ref(), loader, store);
    let module_id = ModuleId::new(
        AccountAddress::from_hex_literal(&address)?,
        Identifier::new(module_name)?,
    );
    let bytecode = resolver.load_bytecode(&id_to_str(&module_id))?;
    make_module_abi(&bytecode)
}

pub fn make_abi(
    chain_api: String,
    source_map: SourceMap,
    dialect: String,
    address: String,
) -> Result<Vec<ModuleAbi>, Error> {
    let units = build(chain_api, source_map, dialect, address)?;
    units
        .units
        .into_iter()
        .map(|u| make_module_abi(&u.bytecode))
        .collect::<Result<Vec<_>, _>>()
}

pub fn tx(
    chain_api: String,
    source_map: SourceMap,
    dialect: String,
    call: String,
) -> Result<Unit, Error> {
    let context = Context {
        dialect: DialectName::from_str(&dialect)?.get_dialect(),
        chain_api,
        cfg: Config::for_tx(),
        ..Context::default()
    };
    tx::make_transaction(&source_map, &context, &call, None)
}
