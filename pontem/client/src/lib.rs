use std::fs;
use anyhow::{Error, Result};
use libloading::Library;
use url::Url;

/// Path to the compiled library
#[cfg(target_os = "linux")]
const LIB_PONTEMAPI: &[u8] = include_bytes!("../../pontemapi/target/release/libpontemapi.so");

/// Path to the compiled library
#[cfg(target_os = "macos")]
const LIB_PONTEMAPI: &[u8] = include_bytes!("../../pontemapi/target/release/libpontemapi.dylib");

/// Path to the compiled library
#[cfg(target_os = "windows")]
const LIB_PONTEMAPI: &[u8] = include_bytes!("../../pontemapi/target/release/libpontemapi.dll");

/// Library Version pontemapi
#[cfg(not(doc))]
const LIB_VERSION: &str = hash_project::version!("pontem/pontemapi");

/// Docgen is launched from the current directory
#[cfg(doc)]
const LIB_VERSION: &str = hash_project::version!("../pontemapi");

/// Type of function from the library
type FnInterface = unsafe fn(&str, &str, u64, &str) -> Result<String>;

/// Client for publishing module, bundle, transactions to node
pub struct PontemClient {
    lib: Library,
    /// url: Node address. ws://127.0.0.1:9944
    url: Url,
}

impl PontemClient {
    /// Creating a temporary library file and initializing the library
    ///     url: Node address. ws://127.0.0.1:9944
    pub fn new(url: &str) -> Result<PontemClient, Error> {
        let url = Url::parse(url)?;
        let lib = PontemClient::load()?;

        Ok(PontemClient { lib, url })
    }

    /// Publishing the module.
    ///     package_path: The path to the module file. PATH/TO/MODULE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     key_phrase: secret keyphrase
    pub fn tx_mvm_publish_module(
        &self,
        module_path: &str,
        gas: u64,
        key_phrase: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> = self.lib.get(b"tx_mvm_publish_module")?;
            func(module_path, self.url.as_str(), gas, key_phrase)
        }
    }

    /// (DEV) Publishing the module.
    ///     package_path: The path to the module file. PATH/TO/MODULE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    pub fn tx_mvm_publish_module_dev(
        &self,
        module_path: &str,
        gas: u64,
        signer: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> =
                self.lib.get(b"tx_mvm_publish_module_dev")?;
            func(module_path, self.url.as_str(), gas, signer)
        }
    }

    /// Transaction execution
    ///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     key_phrase: secret keyphrase
    pub fn tx_mvm_execute(
        &self,
        transaction_path: &str,
        gas: u64,
        key_phrase: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> = self.lib.get(b"tx_mvm_execute")?;
            func(transaction_path, self.url.as_str(), gas, key_phrase)
        }
    }

    /// (DEV) Transaction execution
    ///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     test_account: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    pub fn tx_mvm_execute_dev(
        &self,
        transaction_path: &str,
        gas: u64,
        test_account: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> = self.lib.get(b"tx_mvm_execute_dev")?;
            func(transaction_path, self.url.as_str(), gas, test_account)
        }
    }

    /// Publishing the package
    ///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     key_phrase: secret keyphrase
    pub fn tx_mvm_publish_package(
        &self,
        package_path: &str,
        gas: u64,
        key_phrase: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> =
                self.lib.get(b"tx_mvm_publish_package")?;
            func(package_path, self.url.as_str(), gas, key_phrase)
        }
    }

    /// (DEV) Publishing the package
    ///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
    ///     gas: Gas limit for transaction execution.
    ///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    pub fn tx_mvm_publish_package_dev(
        &self,
        package_path: &str,
        gas: u64,
        signer: &str,
    ) -> Result<String> {
        unsafe {
            let func: libloading::Symbol<FnInterface> =
                self.lib.get(b"tx_mvm_publish_package_dev")?;
            func(package_path, self.url.as_str(), gas, signer)
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
    /// <TMP_PATH>/libpontemapi.so
    fn load() -> Result<Library> {
        let name_file = format!("libpontemapi_{}", LIB_VERSION);
        let path = std::env::temp_dir().join(name_file);
        if !path.exists() {
            fs::write(&path, LIB_PONTEMAPI)?;
        }
        let lib = unsafe { libloading::Library::new(path.as_os_str())? };
        Ok(lib)
    }
}

impl Default for PontemClient {
    fn default() -> Self {
        PontemClient::new("ws://127.0.0.1:9944").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::PontemClient;

    #[test]
    fn test_version() {
        let version = PontemClient::default().version().unwrap();
        println!("version: {}", &version);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_module_dev() {
        env_logger::init();

        let client = PontemClient::default();
        let result = client
            .tx_mvm_publish_module_dev("../pontemapi/Alice_Store.mv", 100, "//Alice")
            .unwrap();
        debug!("result: {}", result);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_execute_dev() {
        env_logger::init();

        let client = PontemClient::default();
        let result = client
            .tx_mvm_execute_dev("../pontemapi/Alice_Main.mvt", 100, "alice")
            .unwrap();
        debug!("result: {}", result);
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_package_dev() {
        env_logger::init();

        let client = PontemClient::default();
        let result = client
            .tx_mvm_publish_package_dev(
                "../pontemapi/Alice_Store.pac",
                1000,
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            )
            .unwrap();
        debug!("result: {}", result);
    }
}
