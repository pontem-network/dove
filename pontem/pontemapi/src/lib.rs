use std::fs;
use std::str::FromStr;
use std::path::PathBuf;
use anyhow::{Result, anyhow, ensure};
use log::debug;
use url::{Url, Origin};
use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_core::crypto::Pair;
use sp_core::sr25519::Pair as sr25519Pair;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

/// Library version with a short hash
const VERSION: &str = hash_project::version!(".");

// metadata for encoding and decoding
#[subxt::subxt(
    runtime_metadata_path = "metadata/pontem.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod pontem {}

/// Implementation of the missing "traits"
const _: () = {
    use pontem::runtime_types::polkadot_parachain::primitives::Id;

    impl PartialEq for Id {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Id {}

    impl PartialOrd for Id {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Id {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
};

use crate::pontem::DefaultConfig;

/// Public interface for publishing the module
///     module_path: The path to the module file. PATH/TO/MODULE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     key_phrase: secret keyphrase
#[export_name = "tx_mvm_publish_module"]
pub fn tx_mvm_publish_module(
    module_path: &str,
    url_str: &str,
    gas: u64,
    key_phrase: &str,
) -> Result<String> {
    let context = Context::from_keyphrase(module_path, url_str, gas, key_phrase)?;
    debug!("fn tx_mvm_publish_module:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_module(context));

    result
}

/// (DEV) Public interface for publishing the module
///     module_path: The path to the module file. PATH/TO/MODULE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     test_signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_publish_module_dev"]
pub fn tx_mvm_publish_module_dev(
    module_path: &str,
    url_str: &str,
    gas: u64,
    test_signer: &str,
) -> Result<String> {
    let context = Context::from_dev(module_path, url_str, gas, test_signer)?;
    debug!("fn tx_mvm_publish_module_dev:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_module(context));

    result
}

/// Public interface for transaction execution
///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     key_phrase: secret keyphrase
#[export_name = "tx_mvm_execute"]
pub fn tx_mvm_execute(
    transaction_path: &str,
    url_str: &str,
    gas: u64,
    key_phrase: &str,
) -> Result<String> {
    let context = Context::from_keyphrase(transaction_path, url_str, gas, key_phrase)?;
    debug!("fn tx_mvm_execute:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(execute(context));

    result
}

/// (DEV) Public interface for transaction execution
///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     test_signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_execute_dev"]
pub fn tx_mvm_execute_dev(
    transaction_path: &str,
    url_str: &str,
    gas: u64,
    test_signer: &str,
) -> Result<String> {
    let context = Context::from_dev(transaction_path, url_str, gas, test_signer)?;
    debug!("fn tx_mvm_execute_dev:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(execute(context));

    result
}

/// Public interface for publishing the package
///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     key_phrase: secret keyphrase
#[export_name = "tx_mvm_publish_package"]
pub fn tx_mvm_publish_package(
    package_path: &str,
    url_str: &str,
    gas: u64,
    key_phrase: &str,
) -> Result<String> {
    let context = Context::from_keyphrase(package_path, url_str, gas, key_phrase)?;
    debug!("fn tx_mvm_publish_package:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_package_dev(context));

    result
}

/// (DEV) Public interface for publishing the package
///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     test_signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_publish_package_dev"]
pub fn tx_mvm_publish_package_dev(
    package_path: &str,
    url_str: &str,
    gas: u64,
    test_signer: &str,
) -> Result<String> {
    let context = Context::from_dev(package_path, url_str, gas, test_signer)?;
    debug!("fn tx_mvm_publish_package_dev:\n{}", context.debug());

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_package_dev(context));

    result
}

/// Library Version
#[no_mangle]
pub fn version() -> String {
    VERSION.to_string()
}

/// Publish a module
async fn pb_module(context: Context) -> Result<String> {
    debug!("Reading a file: {}", context.path_file.display());
    let module = fs::read(&context.path_file)?;
    let signer_pair: PairSigner<DefaultConfig, sr25519Pair> =
        PairSigner::new(context.pair.clone());

    let api = ClientBuilder::new()
        .set_url(context.url.clone())
        .build()
        .await?
        .to_runtime_api::<pontem::RuntimeApi<pontem::DefaultConfig>>();

    let published = api.tx().mvm().publish_module(module, context.gas);

    if !context.is_connection_ws() {
        return Ok(published.sign_and_submit(&signer_pair).await?.to_string());
    }

    let hash = published
        .sign_and_submit_then_watch(&signer_pair)
        .await?
        .wait_for_in_block()
        .await?
        .wait_for_success()
        .await?
        .block_hash()
        .to_string();

    Ok(hash)
}

/// Transaction execution
async fn execute(context: Context) -> Result<String> {
    debug!("Reading a file: {}", context.path_file.display());
    let transaction = fs::read(&context.path_file)?;
    let signer_pair: PairSigner<DefaultConfig, sr25519Pair> =
        PairSigner::new(context.pair.clone());

    let api = ClientBuilder::new()
        .set_url(context.url.clone())
        .build()
        .await?
        .to_runtime_api::<pontem::RuntimeApi<pontem::DefaultConfig>>();

    let published = api.tx().mvm().execute(transaction, context.gas);

    if !context.is_connection_ws() {
        return Ok(published.sign_and_submit(&signer_pair).await?.to_string());
    }

    let hash = published
        .sign_and_submit_then_watch(&signer_pair)
        .await?
        .wait_for_in_block()
        .await?
        .wait_for_success()
        .await?
        .block_hash()
        .to_string();

    Ok(hash)
}

/// Publish a package
async fn pb_package_dev(context: Context) -> Result<String> {
    debug!("Reading a file: {}", context.path_file.display());
    let package = fs::read(&context.path_file)?;
    let signer_pair: PairSigner<DefaultConfig, sr25519Pair> =
        PairSigner::new(context.pair.clone());

    let api = ClientBuilder::new()
        .set_url(context.url.clone())
        .build()
        .await?
        .to_runtime_api::<pontem::RuntimeApi<pontem::DefaultConfig>>();

    let published = api.tx().mvm().publish_package(package, context.gas);

    if !context.is_connection_ws() {
        return Ok(published.sign_and_submit(&signer_pair).await?.to_string());
    }

    let hash = published
        .sign_and_submit_then_watch(&signer_pair)
        .await?
        .wait_for_in_block()
        .await?
        .wait_for_success()
        .await?
        .block_hash()
        .to_string();

    Ok(hash)
}

/// Converting a test account alias or ss58 address into a keyring
fn test_keyring_from_str(signer: &str) -> Result<AccountKeyring> {
    let signer_lowercase = signer.strip_prefix("//").unwrap_or(signer).to_lowercase();

    let keyring = match signer_lowercase.as_str() {
        "alice" => AccountKeyring::Alice,
        "bob" => AccountKeyring::Bob,
        "charlie" => AccountKeyring::Charlie,
        "dave" => AccountKeyring::Dave,
        "eve" => AccountKeyring::Eve,
        "ferdie" => AccountKeyring::Ferdie,
        "one" => AccountKeyring::One,
        "two" => AccountKeyring::Two,
        _ => {
            let account_id =
                AccountId32::from_string(signer).map_err(|err| anyhow!("{:?}", err))?;
            AccountKeyring::from_account_id(&account_id)
                .ok_or(anyhow!(r#"Failed to get "keyring""#))?
        }
    };
    Ok(keyring)
}

struct Context {
    /// The path to the module|package|transaction file. PATH/TO/FILE.mv
    pub path_file: PathBuf,
    /// Node address. ws://127.0.0.1:9944
    pub url: Url,
    /// Gas limit for transaction execution.
    pub gas: u64,
    /// ss58 address
    pub signer: String,
    /// Keypair. An Schnorrkel/Ristretto x25519 ("sr25519") key pair.
    pub pair: sr25519Pair,
}

impl Context {
    /// Create Context
    ///     path_str: The path to the module|package|transaction file. PATH/TO/FILE.mv
    ///     url_str: Node address. ws://127.0.0.1:9944
    ///     gas: Gas limit for transaction execution.
    ///     test_signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    pub fn from_dev(
        path_str: &str,
        url_str: &str,
        gas: u64,
        test_signer: &str,
    ) -> Result<Context> {
        let pair = test_keyring_from_str(test_signer)?.pair();
        Self::from_pair(path_str, url_str, gas, pair)
    }

    /// Create Context
    ///     path_str: The path to the module file. PATH/TO/FILE.mv
    ///     url_str: Node address. ws://127.0.0.1:9944
    ///     gas: Gas limit for transaction execution.
    ///     key_phrase: secret keyphrase
    pub fn from_keyphrase(
        path_str: &str,
        url_str: &str,
        gas: u64,
        key_phrase: &str,
    ) -> Result<Context> {
        let pair =
            sr25519Pair::from_string(key_phrase, None).map_err(|err| anyhow!("{:?}", err))?;
        Self::from_pair(path_str, url_str, gas, pair)
    }

    /// Create Context
    ///     path_str: The path to the module file. PATH/TO/FILE.mv
    ///     url_str: Node address. ws://127.0.0.1:9944
    ///     gas: Gas limit for transaction execution.
    ///     pair: An Schnorrkel/Ristretto x25519 ("sr25519") key pair.
    pub fn from_pair(
        path_str: &str,
        url_str: &str,
        gas: u64,
        pair: sr25519Pair,
    ) -> Result<Context> {
        let url = Url::from_str(url_str)?;
        let signer = AccountId32::new(pair.public().0).to_ss58check();

        let mut path_file = PathBuf::from_str(path_str)?;
        ensure!(
            path_file.exists(),
            "File not found for publication. \n\
            Path: {path}",
            path = path_file.display(),
        );
        path_file = path_file.canonicalize()?;

        Ok(Context {
            path_file,
            pair,
            url,
            gas,
            signer,
        })
    }

    /// Returns an object as a string
    pub fn debug(&self) -> String {
        format!(
            "path: {path}\n\
            Url: {url}\n\
            Gas: {gas}\n\
            Signer:{signer}",
            path = self.path_file.display(),
            gas = self.gas,
            signer = &self.signer,
            url = &self.url
        )
    }

    /// is the connection via a web socket
    pub fn is_connection_ws(&self) -> bool {
        match self.url.origin() {
            Origin::Tuple(protocol, _, _) => &protocol.to_lowercase() == "ws",
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::{
        test_keyring_from_str, tx_mvm_publish_module_dev, tx_mvm_execute_dev,
        tx_mvm_publish_package_dev, version, tx_mvm_publish_module,
    };

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_module_dev_ws() {
        tx_mvm_publish_module_dev(
            "./for_test/bytecode_modules/Demo1v.mv",
            "ws://127.0.0.1:9944",
            100,
            "alice",
        )
        .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_module_dev_http() {
        tx_mvm_publish_module_dev(
            "./for_test/bytecode_modules/Demo1v.mv",
            "http://127.0.0.1:9933",
            100,
            "alice",
        )
        .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_execute_dev_ws() {
        tx_mvm_execute_dev("./Alice_Main.mvt", "ws://127.0.0.1:9944", 100, "//Alice").unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_package_dev_ws() {
        tx_mvm_publish_package_dev(
            "./Alice_Store.pac",
            "ws://127.0.0.1:9944",
            1000,
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        )
        .unwrap();
    }

    #[test]
    fn test_to_key_pair() {
        assert!(test_keyring_from_str("alice").is_ok());
        assert!(test_keyring_from_str("Alice").is_ok());
        assert!(test_keyring_from_str("//Alice").is_ok());
        assert!(
            test_keyring_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").is_ok()
        );
    }

    #[test]
    fn test_version() {
        debug!("{}", version());
    }

    #[test]
    #[ignore]
    fn test_key_pair() {
        // demo account
        // 5DeyRkpWxXkdDHKqsqtYZLG6M3fHdqpAb55W5DPNQSaZPeg4
        // net exotic exchange stadium camp mind walk cart infant hospital will address
        // net … … stadium … … walk … … hospital … …
        // sr25519

        let result = tx_mvm_publish_module(
            "./for_test/bytecode_modules/Demo1v.mv",
            "ws://127.0.0.1:9944",
            100,
            "net exotic exchange stadium camp mind walk cart infant hospital will address",
        )
        .unwrap();
        println!("{}", result);
    }
}
