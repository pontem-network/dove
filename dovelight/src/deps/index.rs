use std::collections::{HashSet, HashMap};
use move_core_types::language_storage::ModuleId;
use serde::{Serialize, Deserialize};
use anyhow::{Error, ensure};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Index {
    deps: HashMap<ModuleId, HashSet<ModuleId>>,
}

impl Index {
    pub fn contains(&self, key: &ModuleId) -> bool {
        self.deps.contains_key(key)
    }

    pub fn insert(&mut self, id: ModuleId, deps: HashSet<ModuleId>) {
        self.deps.insert(id, deps);
    }

    pub fn all_deps(&self, prefix: &str) -> Vec<String> {
        let deps = self.deps.iter().fold(HashSet::new(), |mut acc, (k, v)| {
            acc.insert(format!("{}{}", prefix, id_to_str(k)));
            for item in v {
                acc.insert(format!("{}{}", prefix, id_to_str(item)));
            }
            acc
        });

        deps.into_iter().collect()
    }
}

pub fn id_to_str(id: &ModuleId) -> String {
    format!("{:x}::{}", id.address, id.name)
}

pub fn str_to_id(id: &str) -> Result<ModuleId, Error> {
    let address_hex_len = AccountAddress::LENGTH * 2;
    ensure!(
        id.len() > address_hex_len + 2,
        "Invalid module id len. The length must be greater than {}.",
        address_hex_len + 2
    );
    let address = AccountAddress::from_hex(&id[0..address_hex_len])?;
    let id = Identifier::new(&id[address_hex_len + 2..])?;
    Ok(ModuleId::new(address, id))
}

#[cfg(test)]
mod tests {
    use move_core_types::language_storage::ModuleId;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use crate::deps::index::{id_to_str, str_to_id};

    #[test]
    pub fn encode_decode() {
        let id = ModuleId::new(
            AccountAddress::from_hex_literal("0x42").unwrap(),
            Identifier::new("Pont").unwrap(),
        );
        let encoded = id_to_str(&id);
        assert_eq!(id, str_to_id(&encoded).unwrap())
    }
}
