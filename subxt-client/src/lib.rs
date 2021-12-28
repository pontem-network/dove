use std::fs;
use anyhow::{Error, Result};
use libloading::Library;
use url::Url;

/// Path to the compiled library
#[cfg(target_os = "linux")]
const LIB_SUBXT: &[u8] = include_bytes!("../../libs/subxt/target/release/libsubxt.so");

/// Path to the compiled library
#[cfg(target_os = "macos")]
const LIB_SUBXT: &[u8] = include_bytes!("../../libs/subxt/target/release/libsubxt.dylib");

/// Path to the compiled library
#[cfg(target_os = "windows")]
const LIB_SUBXT: &[u8] = include_bytes!("../../libs/subxt/target/release/libsubxt.dll");

/// Library Version subxt
#[cfg(not(doc))]
const LIB_VERSION: &str = hash_project::version!("libs/subxt");

/// Docgen is launched from the current directory
#[cfg(doc)]
const LIB_VERSION: &str = hash_project::version!("../libs/subxt");

/// Type of function from the library
type FnInterface = unsafe fn(&str, &str, u64, &str) -> Result<String>;

/// Client for node
/// Only for test accounts.
pub struct SubxtClient {
    lib: Library,
    /// signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    signer: String,
    /// url: Node address. ws://127.0.0.1:9944
    url: Url,
}

impl SubxtClient {
    /// Creating a temporary library file and initializing the library
    ///     url: Node address. ws://127.0.0.1:9944
    ///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    pub fn new(url: &str, sigrer: &str) -> Result<SubxtClient, Error> {
        let url = Url::parse(url)?;
        let signer = sigrer.to_string();
        let lib = SubxtClient::load()?;

        Ok(SubxtClient { lib, url, signer })
    }

    /// Publishing the module.
    ///     package_path: The path to the module file. PATH/TO/MODULE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    pub fn tx_mvm_publish_module_dev(&self, module_path: &str, gas: u64) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> =
                self.lib.get(b"tx_mvm_publish_module_dev")?;
            func(module_path, self.url.as_str(), gas, &self.signer)
        }
    }

    /// Transaction execution
    ///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
    ///     gas: Gas limit for transaction execution.
    pub fn tx_mvm_execute_dev(&self, transaction_path: &str, gas: u64) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> = self.lib.get(b"tx_mvm_execute_dev")?;
            func(transaction_path, self.url.as_str(), gas, &self.signer)
        }
    }

    /// Publishing the package
    ///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    pub fn tx_mvm_publish_package_dev(&self, package_path: &str, gas: u64) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> =
                self.lib.get(b"tx_mvm_publish_package_dev")?;
            func(package_path, self.url.as_str(), gas, &self.signer)
        }
    }

    /// Library Version
    pub fn version(&self) -> Result<String> {
        let result = unsafe {
            let func: libloading::Symbol<unsafe fn() -> String> = self.lib.get(b"version")?;
            func()
        };
        Ok(result)
    }

    /// Create a library in a temporary folder
    /// <TMP_PATH>/libsubxt.so
    fn load() -> Result<Library> {
        let name_file = format!("libsubxt_{}", LIB_VERSION);
        let path = std::env::temp_dir().join(name_file);
        if !path.exists() {
            fs::write(&path, LIB_SUBXT)?;
        }
        let lib = unsafe { libloading::Library::new(path.as_os_str())? };
        Ok(lib)
    }
}

impl Default for SubxtClient {
    fn default() -> Self {
        SubxtClient::new("ws://127.0.0.1:9944", "alice").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::SubxtClient;

    #[test]
    fn test_version() {
        let version = SubxtClient::default().version().unwrap();
        println!("version: {}", &version);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_module_dev() {
        env_logger::init();

        let client = SubxtClient::default();
        let result = client
            .tx_mvm_publish_module_dev("../libs/subxt/0_Store.mv", 100)
            .unwrap();
        debug!("result: {}", result);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_execute_dev() {
        env_logger::init();

        let client = SubxtClient::default();
        let result = client
            .tx_mvm_execute_dev("../libs/subxt/main.mvt", 100)
            .unwrap();
        debug!("result: {}", result);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_package_dev() {
        env_logger::init();

        let client = SubxtClient::default();
        let result = client
            .tx_mvm_publish_package_dev("../libs/subxt/move_store.pac", 1000)
            .unwrap();
        debug!("result: {}", result);
    }
}
