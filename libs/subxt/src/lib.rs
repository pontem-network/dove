use std::fs;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use anyhow::{Result, bail, anyhow};
use log::debug;
use url::Url;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Ss58Codec;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, EventSubscription, Metadata, PairSigner};

/// Library version with a short hash
const VERSION: &str = hash_project::version!(".");

/// metadata for encoding and decoding
mod pontem;
const _: () = {
    use pontem_api::runtime_types::polkadot_parachain::primitives::Id;

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

use pontem::api as pontem_api;
use crate::pontem_api::runtime_types::sp_runtime::DispatchError;

/// Public interface for publishing the module
///     module_path: The path to the module file. PATH/TO/MODULE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_publish_module_dev"]
pub fn tx_mvm_publish_module_dev(
    module_path: &str,
    url_str: &str,
    gas: u64,
    signer: &str,
) -> Result<String> {
    let url = Url::from_str(url_str)?;
    let signer_keyring = keyring_from_str(signer)?;

    let mut module_path = PathBuf::from_str(module_path)?;
    if !module_path.exists() {
        bail!(
            "The module for publication was not found. Wrong way. \n\
            Path: {path}",
            path = module_path.display(),
        );
    }
    module_path = module_path.canonicalize()?;

    debug!(
        "fn tx_mvm_publish_module_dev:\n\
        module path: {path}\n\
        Url: {url}\n\
        Gas: {gas}\n\
        Signer:{signer}\n\
        Keyring: {keyring}",
        path = module_path.display(),
        gas = &gas,
        signer = signer,
        keyring = signer_keyring.to_account_id().to_string(),
        url = &url
    );

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_module_dev(&module_path, &url, gas, &signer_keyring));

    result
}

/// Public interface for transaction execution
///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_execute_dev"]
pub fn tx_mvm_execute_dev(
    transaction_path: &str,
    url_str: &str,
    gas: u64,
    signer: &str,
) -> Result<String> {
    let url = Url::from_str(url_str)?;
    let signer_keyring = keyring_from_str(signer)?;

    let mut transaction_path = PathBuf::from_str(transaction_path)?;
    if !transaction_path.exists() {
        bail!(
            "The transaction for publication was not found. Wrong way. \n\
            Path: {path}",
            path = transaction_path.display(),
        );
    }
    transaction_path = transaction_path.canonicalize()?;

    debug!(
        "fn tx_mvm_execute_dev:\n\
        transaction path: {path}\n\
        Url: {url}\n\
        Gas: {gas}\n\
        Signer:{signer}\n\
        Keyring: {keyring}",
        path = transaction_path.display(),
        gas = &gas,
        signer = signer,
        keyring = signer_keyring.to_account_id().to_string(),
        url = &url
    );

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(execute_dev(&transaction_path, &url, gas, &signer_keyring));

    result
}

/// Public interface for publishing the package
///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: alias or ss58 address of the test account. //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[export_name = "tx_mvm_publish_package_dev"]
pub fn tx_mvm_publish_package_dev(
    package_path: &str,
    url_str: &str,
    gas: u64,
    signer: &str,
) -> Result<String> {
    let url = Url::from_str(url_str)?;
    let signer_keyring = keyring_from_str(signer)?;

    let mut package_path = PathBuf::from_str(package_path)?;
    if !package_path.exists() {
        bail!(
            "The package for publication was not found. Wrong way. \n\
            Path: {path}",
            path = package_path.display(),
        );
    }
    package_path = package_path.canonicalize()?;

    debug!(
        "fn tx_mvm_publish_package_dev:\n\
        package path: {path}\n\
        Url: {url}\n\
        Gas: {gas}\n\
        Signer:{signer}\n\
        Keyring: {keyring}",
        path = package_path.display(),
        gas = &gas,
        signer = signer,
        keyring = signer_keyring.to_account_id().to_string(),
        url = &url
    );

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(pb_package_dev(&package_path, &url, gas, &signer_keyring));

    result
}

/// Library Version
#[no_mangle]
pub fn version() -> String {
    VERSION.to_string()
}

/// Publish a module
///     module_path: The path to the module file. PATH/TO/MODULE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: keyring to the test account
async fn pb_module_dev(
    module_path: &Path,
    url: &Url,
    gas: u64,
    signer: &AccountKeyring,
) -> Result<String> {
    debug!("Reading a file: {}", module_path.display());
    let module = fs::read(module_path)?;

    debug!("Getting a key pair");
    let signer_pair = PairSigner::new(signer.pair());

    let api = ClientBuilder::new()
        .set_url(url.to_string())
        .build()
        .await?
        .to_runtime_api::<pontem_api::RuntimeApi<pontem_api::DefaultConfig>>();

    let hash = api
        .tx()
        .mvm()
        .publish_module(module, gas)
        .sign_and_submit(&signer_pair)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !is_ws(url) {
        return Ok(hash);
    }

    // It is necessary to decrypt the message
    let metadata = api.client.metadata();
    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem_api::DefaultConfig>::new(sub, decoder);

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
                let answer =
                    <pontem_api::system::events::ExtrinsicFailed as codec::Decode>::decode(
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

/// transaction execution
///     transaction_path: The path to the transaction file. PATH/TO/TRANSACTION/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: keyring to the test account
async fn execute_dev(
    transaction_path: &Path,
    url: &Url,
    gas: u64,
    signer: &AccountKeyring,
) -> Result<String> {
    debug!("Reading a file: {}", transaction_path.display());
    let transaction = fs::read(transaction_path)?;

    debug!("Getting a key pair");
    let signer_pair = PairSigner::new(signer.pair());

    let api = ClientBuilder::new()
        .set_url(url.to_string())
        .build()
        .await?
        .to_runtime_api::<pontem_api::RuntimeApi<pontem_api::DefaultConfig>>();

    let hash = api
        .tx()
        .mvm()
        .execute(transaction, gas)
        .sign_and_submit(&signer_pair)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !is_ws(url) {
        return Ok(hash);
    }

    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem_api::DefaultConfig>::new(sub, decoder);
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
                let answer =
                    <pontem_api::system::events::ExtrinsicFailed as codec::Decode>::decode(
                        &mut &raw.data[..],
                    )?;
                return Err(anyhow!(dispatcherror_to_string(answer.0, metadata)));
            }
            _ => {}
        }
    }
}

/// Publish a package
///     package_path: The path to the package file. PATH/TO/PACKAGE/FILE.mv
///     url: Node address. ws://127.0.0.1:9944
///     gas: Gas limit for transaction execution.
///     signer: keyring to the test account
async fn pb_package_dev(
    package_path: &Path,
    url: &Url,
    gas: u64,
    signer: &AccountKeyring,
) -> Result<String> {
    debug!("Reading a file: {}", package_path.display());
    let package = fs::read(package_path)?;

    debug!("Getting a key pair");
    let signer_pair = PairSigner::new(signer.pair());

    let api = ClientBuilder::new()
        .set_url(url.to_string())
        .build()
        .await?
        .to_runtime_api::<pontem_api::RuntimeApi<pontem_api::DefaultConfig>>();

    let hash = api
        .tx()
        .mvm()
        .publish_package(package, gas)
        .sign_and_submit(&signer_pair)
        .await?
        .to_string();

    // Only for Websocket you can get the result of publishing
    if !is_ws(url) {
        return Ok(hash);
    }

    // Subscribe to events
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<pontem_api::DefaultConfig>::new(sub, decoder);
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
                let answer =
                    <pontem_api::system::events::ExtrinsicFailed as codec::Decode>::decode(
                        &mut &raw.data[..],
                    )?;
                return Err(anyhow!(dispatcherror_to_string(answer.0, metadata)));
            }
            _ => {}
        }
    }
}

/// Converting a test account alias or ss58 address into a keyring
fn keyring_from_str(signer: &str) -> Result<AccountKeyring> {
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
    use crate::pontem_api::runtime_types::sp_runtime::{ArithmeticError, TokenError};
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

fn is_ws(url: &Url) -> bool {
    use url::Origin;

    match url.origin() {
        Origin::Tuple(protocol, _, _) => &protocol.to_lowercase() == "ws",
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use log::debug;
    use crate::{
        keyring_from_str, tx_mvm_publish_module_dev, tx_mvm_execute_dev,
        tx_mvm_publish_package_dev, version,
    };

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_module_dev_ws() {
        env_logger::init();

        tx_mvm_publish_module_dev("./0_Store.mv", "ws://127.0.0.1:9944", 100, "alice").unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_execute_dev_ws() {
        env_logger::init();

        tx_mvm_execute_dev("./main.mvt", "ws://127.0.0.1:9944", 100, "//Alice").unwrap();
    }

    #[test]
    #[ignore]
    fn test_tx_mvm_publish_package_dev_ws() {
        env_logger::init();

        tx_mvm_publish_package_dev(
            "./move_store.pac",
            "ws://127.0.0.1:9944",
            1000,
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        )
        .unwrap();
    }

    #[test]
    fn test_to_key_pair() {
        env_logger::init();

        assert!(keyring_from_str("alice").is_ok());
        assert!(keyring_from_str("Alice").is_ok());
        assert!(keyring_from_str("//Alice").is_ok());
        assert!(keyring_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").is_ok());
    }

    #[test]
    fn test_version() {
        env_logger::init();

        debug!("{}", version());
    }
}
