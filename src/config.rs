use std::path::PathBuf;

use move_lang::shared::Address;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MoveDialect {
    Libra,
    DFinance,
}

#[derive(Debug)]
pub struct Config {
    pub dialect: MoveDialect,
    pub module_folders: Vec<PathBuf>,
    pub sender_address: Address,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dialect: MoveDialect::Libra,
            module_folders: vec![],
            sender_address: Address::default(),
        }
    }
}

impl Config {
    pub fn update(&mut self, value: &serde_json::Value) {
        log::info!("Config::update({:#})", value);

        set(value, "/dialect", &mut self.dialect);
        set(value, "/modules_folders", &mut self.module_folders);
        self.sender_address = match get(value, "/sender_address") {
            None => {
                log::info!("Using default account address 0x0");
                Address::default()
            }
            Some(address) => match Address::parse_str(address) {
                Ok(acc_address) => acc_address,
                Err(error) => {
                    log::error!("Invalid sender_address string: {:?}", error);
                    log::info!("Using default account address 0x0");
                    Address::default()
                }
            },
        };
        log::info!("Config::update() = {:#?}", self);

        fn get<'a, T: Deserialize<'a>>(value: &'a serde_json::Value, pointer: &str) -> Option<T> {
            value
                .pointer(pointer)
                .and_then(|it| T::deserialize(it).ok())
        }

        fn set<'a, T: Deserialize<'a>>(value: &'a serde_json::Value, pointer: &str, slot: &mut T) {
            if let Some(new_value) = get(value, pointer) {
                *slot = new_value
            }
        }
    }
}
