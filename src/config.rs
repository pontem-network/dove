use std::path::PathBuf;

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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dialect: MoveDialect::Libra,
            module_folders: vec![],
        }
    }
}

impl Config {
    pub fn update(&mut self, value: &serde_json::Value) {
        log::info!("Config::update({:#})", value);

        set(value, "/dialect", &mut self.dialect);
        set(value, "/modules", &mut self.module_folders);

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
