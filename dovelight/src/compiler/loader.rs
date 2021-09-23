use crate::deps::DependencyLoader;
use move_core_types::language_storage::ModuleId;
use anyhow::{
    Error, anyhow,
};
use crate::console_log;
use reqwest::Client;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use std::sync::mpsc::channel;

#[derive(Clone)]
pub struct Loader {
    chain_api: String,
    client: Client,
}

impl Loader {
    pub fn new(chain_api: String) -> Loader {
        Loader { chain_api, client: Client::new() }
    }

    async fn get_module_async(&self, id: &ModuleId) -> Result<Vec<u8>, Error> {
        let resp = self.client.post(&self.chain_api)
            .header("Content-Type", "application/json")
            .json(&Request {
                id: 1,
                jsonrpc: "2.0",
                method: "mvm_getModule",
                params: vec![bcs::to_bytes(id)?],
            }).send()
            .await?;
        if resp.status().is_success() {
            Ok(vec![])
        } else {
            Err(anyhow!("Failed to get module :{}. Error:{}-{}", id, resp.status(), resp.text().await?))
        }
    }
}

impl DependencyLoader for Loader {
    fn get_module(&self, id: &ModuleId) -> Result<Vec<u8>, Error> {
        let (tx, rx) = channel();
        let loader = self.clone();
        let id = id.clone();
        spawn_local(async move {
            if let Err(err) = tx.send(loader.get_module_async(&id).await) {
                console_log!("Failed to lead module:{:?}", err);
            }
        });
        rx.recv()?
    }
}

#[derive(Serialize)]
struct Request {
    id: u64,
    jsonrpc: &'static str,
    method: &'static str,
    params: Vec<Vec<u8>>,
}