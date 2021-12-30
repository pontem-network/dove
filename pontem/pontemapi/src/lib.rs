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
use subxt::{ClientBuilder, EventSubscription, Metadata, PairSigner};

/// Library version with a short hash
const VERSION: &str = hash_project::version!(".");

/// metadata for encoding and decoding
#[subxt::subxt(
    runtime_metadata_path = "metadata/pontem.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod pontem {}
// mod pontem;

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
use crate::pontem::runtime_types::sp_runtime::DispatchError;

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

/// Public interface for publishing the module
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
    let pair_signer: PairSigner<DefaultConfig, sr25519Pair> =
        PairSigner::new(context.pair.clone());

    let api = ClientBuilder::new()
        .set_url(context.url.clone())
        .build()
        .await?
        .to_runtime_api::<pontem::RuntimeApi<pontem::DefaultConfig>>();

    let hash = api
        .tx()
        .mvm()
        .publish_module(module, context.gas)
        .sign_and_submit(&pair_signer)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !context.is_connection_ws() {
        return Ok(hash);
    }

    // It is necessary to decrypt the message
    let metadata = api.client.metadata();
    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem::DefaultConfig>::new(sub, decoder);

    let mut last = 0;
    loop {
        let raw = sub.next().await.ok_or(anyhow!("No response received"))??;

        match raw.variant.as_str() {
            // The event is triggered 3 times. If the event was triggered after "ModulePublished", it is final
            "ExtrinsicSuccess" => {
                if last == 1 {
                    return Ok(hash);
                }
            }
            // Called when a module publishing error occurs
            "ExtrinsicFailed" => {
                let answer = <pontem::system::events::ExtrinsicFailed as codec::Decode>::decode(
                    &mut &raw.data[..],
                )?;
                return Err(anyhow!(dispatcherror_to_string(answer.0, metadata)));
            }
            // The module is published. Not the last event
            "ModulePublished" | "Event" => last = 1,
            _ => {}
        }
    }
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

    let hash = api
        .tx()
        .mvm()
        .execute(transaction, context.gas)
        .sign_and_submit(&signer_pair)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !context.is_connection_ws() {
        return Ok(hash);
    }

    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem::DefaultConfig>::new(sub, decoder);
    // It is necessary to decrypt the message
    let metadata = api.client.metadata();

    let mut last = 0;
    loop {
        let raw = sub.next().await.ok_or(anyhow!("No response received"))??;

        debug!("event {}", raw.variant.as_str());
        match raw.variant.as_str() {
            // The event is triggered 4 times
            "ExtrinsicSuccess" => {
                last += 1;
                if last == 4 {
                    return Ok(hash);
                }
            }
            // Called when a module execute error occurs
            "ExtrinsicFailed" => {
                let answer = <pontem::system::events::ExtrinsicFailed as codec::Decode>::decode(
                    &mut &raw.data[..],
                )?;
                return Err(anyhow!(dispatcherror_to_string(answer.0, metadata)));
            }
            _ => {}
        }
    }
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

    let hash = api
        .tx()
        .mvm()
        .publish_package(package, context.gas)
        .sign_and_submit(&signer_pair)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !context.is_connection_ws() {
        return Ok(hash);
    }

    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem::DefaultConfig>::new(sub, decoder);
    // It is necessary to decrypt the message
    let metadata = api.client.metadata();

    let mut last = 0;
    loop {
        let raw = sub.next().await.ok_or(anyhow!("No response received"))??;

        debug!("event {}", raw.variant.as_str());
        match raw.variant.as_str() {
            // The event is triggered 4 times
            "ExtrinsicSuccess" => {
                last += 1;
                if last == 4 {
                    return Ok(hash);
                }
            }
            // Called when a module publishing error occurs
            "ExtrinsicFailed" => {
                let answer = <pontem::system::events::ExtrinsicFailed as codec::Decode>::decode(
                    &mut &raw.data[..],
                )?;
                return Err(anyhow!(dispatcherror_to_string(answer.0, metadata)));
            }
            _ => {}
        }
    }
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

/// Converting an error to a string. Error when calling an external function in the node
fn dispatcherror_to_string(error: DispatchError, meta: &Metadata) -> String {
    use crate::pontem::runtime_types::sp_runtime::{ArithmeticError, TokenError};
    match error {
        DispatchError::Other => "Other".to_string(),
        DispatchError::CannotLookup => "CannotLookup".to_string(),
        DispatchError::BadOrigin => "BadOrigin".to_string(),
        DispatchError::Module { index, error } => match meta.error(index, error) {
            Ok(ok) => format!(
                "Pallet: {pallet}\n\
                Error: {error}\n\
                Description: {description}",
                pallet = ok.pallet(),
                error = ok.error(),
                description = ok.description().join(" ")
            ),
            Err(_) => format!("Error not found: {} {}", index, error),
        },
        DispatchError::ConsumerRemaining => "ConsumerRemaining".to_string(),
        DispatchError::NoProviders => "NoProviders".to_string(),
        DispatchError::Token(value) => match value {
            TokenError::NoFunds => "Token.NoFunds",
            TokenError::WouldDie => "Token.WouldDie",
            TokenError::BelowMinimum => "Token.BelowMinimum",
            TokenError::CannotCreate => "Token.CannotCreate",
            TokenError::UnknownAsset => "Token.UnknownAsset",
            TokenError::Frozen => "Token.Frozen",
            TokenError::Unsupported => "Token.Unsupported",
        }
        .to_string(),
        DispatchError::Arithmetic(value) => match value {
            ArithmeticError::Underflow => "Arithmetic.Underflow",
            ArithmeticError::Overflow => "Arithmetic.Overflow",
            ArithmeticError::DivisionByZero => "Arithmetic.DivisionByZero",
        }
        .to_string(),
    }
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
        env_logger::init();

        tx_mvm_publish_module_dev("./Alice_Store.mv", "ws://127.0.0.1:9944", 100, "alice")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_execute_dev_ws() {
        env_logger::init();

        tx_mvm_execute_dev("./Alice_Main.mvt", "ws://127.0.0.1:9944", 100, "//Alice").unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_package_dev_ws() {
        env_logger::init();

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
        env_logger::init();

        assert!(test_keyring_from_str("alice").is_ok());
        assert!(test_keyring_from_str("Alice").is_ok());
        assert!(test_keyring_from_str("//Alice").is_ok());
        assert!(
            test_keyring_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").is_ok()
        );
    }

    #[test]
    fn test_version() {
        env_logger::init();

        debug!("{}", version());
    }

    #[test]
    #[ignore]
    fn test_key_pair() {
        env_logger::init();
        // use sp_core::crypto::Pair;

        // demo account
        // 5DeyRkpWxXkdDHKqsqtYZLG6M3fHdqpAb55W5DPNQSaZPeg4
        // net exotic exchange stadium camp mind walk cart infant hospital will address
        // net … … stadium … … walk … … hospital … …
        // sr25519

        let result = tx_mvm_publish_module(
            "./Demo_Store.mv",
            "ws://127.0.0.1:9944",
            100,
            "net exotic exchange stadium camp mind walk cart infant hospital will address",
        )
        .unwrap();
        println!("{}", result);
    }
}
