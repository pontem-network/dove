use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;
use anyhow::Result;
use url::Url;

use pontem_client::PontemClient;
use crate::cmd::key::cli_entering_a_secret_phrase;
use crate::wallet_key;
use crate::wallet_key::WalletKey;

#[derive(Parser, Debug)]
pub struct NodeAccessParams {
    /// Account from whom to publish. Address or test account name or name secret key.
    /// Example: //Alice, alice, bob, NAME_WALLET_KEY... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    #[clap(long = "account")]
    account: Option<String>,

    /// Secret phrase.
    /// If a secret phrase is specified, you do not need to specify an account
    #[clap(long = "secret", short)]
    secret_phrase: bool,

    /// The url of the substrate node to query
    #[clap(
        long = "url",
        short,
        parse(try_from_str),
        default_value = "ws://localhost:9944"
    )]
    url_to_node: Url,

    /// Limitation of gas consumption per operation
    #[clap(long = "gas", short)]
    gas_limit: Option<u64>,
}

impl NodeAccessParams {
    pub fn need_to_publish(&self) -> bool {
        self.account.is_some() || self.secret_phrase
    }
}

pub struct Publish {
    /// Client for connecting to "Pontem"
    client: PontemClient,

    /// Path to the file to be published
    file_path: PathBuf,

    /// Limitation of gas consumption per operation
    gas_limit: u64,

    /// Access type - by secret phrase or through a test account
    access: AccessType,
}

impl Publish {
    pub fn apply(&self) -> Result<String> {
        match self.file_type()? {
            FileType::Module => match &self.access {
                AccessType::SecretPhrase(secret) => self.client.tx_mvm_publish_module(
                    self.file_path_as_str()?,
                    self.gas_limit,
                    secret,
                ),
                AccessType::TestAccount(test_account) => self.client.tx_mvm_publish_module_dev(
                    self.file_path_as_str()?,
                    self.gas_limit,
                    test_account,
                ),
            },
            FileType::Bundle => match &self.access {
                AccessType::SecretPhrase(secret) => self.client.tx_mvm_publish_package(
                    self.file_path_as_str()?,
                    self.gas_limit,
                    secret,
                ),
                AccessType::TestAccount(test_account) => self.client.tx_mvm_publish_package_dev(
                    self.file_path_as_str()?,
                    self.gas_limit,
                    test_account,
                ),
            },
            FileType::TX => match &self.access {
                AccessType::SecretPhrase(secret) => {
                    self.client
                        .tx_mvm_execute(self.file_path_as_str()?, self.gas_limit, secret)
                }
                AccessType::TestAccount(test_account) => self.client.tx_mvm_execute_dev(
                    self.file_path_as_str()?,
                    self.gas_limit,
                    test_account,
                ),
            },
        }
    }
}

/// PublishParamsCmd - Connection parameters
/// PathBuf - The path to the file to be published (*.mvt, *.mv, *.pac)
impl TryFrom<(&NodeAccessParams, PathBuf)> for Publish {
    type Error = Error;

    fn try_from(value: (&NodeAccessParams, PathBuf)) -> std::result::Result<Self, Self::Error> {
        let (params, file_path) = value;
        let gas_limit = params
            .gas_limit
            .ok_or_else(|| anyhow!("Please specify gas limit"))?;
        let mut url_to_node = params.url_to_node.clone();

        let access = if params.secret_phrase {
            // Request secret phrases
            let secret = cli_entering_a_secret_phrase()?;
            AccessType::SecretPhrase(secret)
        } else if let Some(test_account_or_name_key) = &params.account {
            match cli_name_to_key(test_account_or_name_key)? {
                Some(WalletKey {
                    secret_phrase,
                    node_address,
                }) => {
                    url_to_node = node_address;
                    AccessType::SecretPhrase(secret_phrase)
                }
                None => AccessType::TestAccount(test_account_or_name_key.to_owned()),
            }
        } else {
            bail!("Specify name of key or name of test account or secret phrase")
        };

        let client = PontemClient::new(url_to_node.as_str())?;

        Ok(Publish {
            client,
            access,
            gas_limit,
            file_path,
        })
    }
}

impl Publish {
    fn file_type(&self) -> Result<FileType> {
        let ext = self
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_string();

        Ok(match ext.as_str() {
            "pac" => FileType::Bundle,
            "mv" => FileType::Module,
            "mvt" => FileType::TX,
            _ => bail!(
                "pac or mv extension was expected\n{}",
                self.file_path.display()
            ),
        })
    }

    fn file_path_as_str(&self) -> Result<&str> {
        self.file_path
            .to_str()
            .ok_or_else(|| anyhow!("Error converting path to string"))
    }
}

/// Access type - by secret phrase or through a test account
enum AccessType {
    SecretPhrase(String),
    TestAccount(String),
}

enum FileType {
    Bundle,
    Module,
    TX,
}

/// Checking for a key with this name and getting the content
fn cli_name_to_key(key_name: &str) -> Result<Option<WalletKey>> {
    // Checking for a saved key with this name
    if !wallet_key::existence(key_name) {
        return Ok(None);
    }

    // Trying to get secret phrases without a password
    let mut phrase = wallet_key::get(key_name, None);
    if phrase.is_err() {
        // Password required
        println!("Please enter password for key:");
        let password = rpassword::read_password()?.trim().to_string();
        phrase =
            wallet_key::get(key_name, Some(&password)).map_err(|_| anyhow!("Invalid password"))
    }
    phrase.map(Some)
}
