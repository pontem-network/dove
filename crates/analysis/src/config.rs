use std::path::PathBuf;

use core::fmt;
use dialects::{DFinanceDialect, Dialect, MoveDialect};

use serde::export::fmt::Debug;
use serde::export::Formatter;
use serde::Deserialize;
use utils::io;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DialectName {
    Libra,
    DFinance,
}

#[derive(Clone)]
pub struct Config {
    pub dialect: DialectName,
    pub stdlib_folder: Option<PathBuf>,
    pub module_folders: Vec<PathBuf>,
    pub sender_address: String,
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("dialect", &self.dialect)
            .field("stdlib_folder", &self.stdlib_folder)
            .field("module_folders", &self.module_folders)
            .field("sender_address", &self.sender_address)
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dialect: DialectName::DFinance,
            stdlib_folder: None,
            module_folders: vec![],
            sender_address: "0x0".to_string(),
        }
    }
}

impl Config {
    fn dialect(&self) -> Box<dyn Dialect> {
        match self.dialect {
            DialectName::Libra => Box::new(MoveDialect::default()),
            DialectName::DFinance => Box::new(DFinanceDialect::default()),
        }
    }

    fn log_available_module_files(&self) {
        let stdlib_modules = self
            .stdlib_folder
            .as_ref()
            .map(io::iter_over_move_files)
            .unwrap_or_else(|| vec![]);
        log::info!(
            "available stdlib modules (from {:?}) = {:#?}",
            self.stdlib_folder,
            stdlib_modules
        );

        for folder in &self.module_folders {
            let files = io::iter_over_move_files(folder);
            log::info!("third party modules (from {:?}) = {:#?}", folder, files);
        }
    }

    pub fn update(&mut self, value: &serde_json::Value) {
        log::info!("Passed configuration = {:#}", value);

        set(value, "/dialect", &mut self.dialect);
        set(value, "/stdlib_folder", &mut self.stdlib_folder);
        set(value, "/modules_folders", &mut self.module_folders);

        self.sender_address = match get(value, "/sender_address") {
            None => {
                log::info!("Using default account address 0x0");
                "0x0"
            }
            Some(address) => match self.dialect().validate_sender_address(address) {
                Ok(_) => address,
                Err(error) => {
                    log::error!("Invalid sender_address string: {:?}", error);
                    log::info!("Using default account address 0x0");
                    "0x0"
                }
            },
        }
        .to_string();

        log::info!("Config updated to = {:#?}", self);
        self.log_available_module_files();

        fn get<'a, T: Deserialize<'a>>(value: &'a serde_json::Value, pointer: &str) -> Option<T> {
            value
                .pointer(pointer)
                .and_then(|it| T::deserialize(it).ok())
        }

        fn set<'a, T: Deserialize<'a>>(
            value: &'a serde_json::Value,
            pointer: &str,
            slot: &mut T,
        ) {
            if let Some(new_value) = get(value, pointer) {
                *slot = new_value
            }
        }
    }
}
