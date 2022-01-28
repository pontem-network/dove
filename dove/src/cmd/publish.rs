use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use log::debug;
use move_cli::Move;
use pontem_client::PontemClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;
use crate::secret_phrase;

/// Publishing a module or package.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS]  --secret --account [NAME_OR_ADDRESS] --url [URL]"
)]
pub enum Publish {
    /// Publishing a module
    #[structopt(name = "module")]
    Module {
        /// Parameters for publishing a module
        #[structopt(flatten)]
        params: PublicationParameters,
    },

    /// Publishing a package
    #[structopt(name = "package")]
    Package {
        /// Parameters for publishing a package
        #[structopt(flatten)]
        params: PublicationParameters,
    },
}

/// Parameters for publishing a module or package  
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS] --secret --account [ADDRESS] --url [URL]\n
    Examples:
    $ dove publish module --file PATH/TO/MODULE.mv --gas 100 
    $ dove publish package --file ./PATH/TO/PACKAGE.pac --gas 300 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
    $ dove publish module --file /PATH/TO/MODULE.mv --gas 200 --account alice --url ws://127.0.0.1:9944
    $ dove publish module --file /PATH/TO/MODULE.mv --gas 130 --account NAME_SECRET_KEY
    $ dove publish module --file /PATH/TO/MODULE.mv --gas 220 --secret
    "
)]
pub struct PublicationParameters {
    /// The path to the transaction.
    #[structopt(short, long = "file", parse(from_os_str))]
    file_path: PathBuf,

    /// Account from whom to publish. Address or test account name or name secret key. Example: //Alice, alice, bob, NAME_SECRET_KEY... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    #[structopt(long = "account")]
    account: Option<String>,

    /// Secret phrase. If a secret phrase is specified, you do not need to specify an account
    #[structopt(long = "secret", short)]
    secret_phrase: bool,

    /// The url of the substrate node to query
    #[structopt(
        long = "url",
        short,
        parse(try_from_str),
        default_value = "ws://localhost:9944"
    )]
    url_to_node: url::Url,

    /// Limitation of gas consumption per operation
    #[structopt(long = "gas", short)]
    gas_limit: u64,
}

impl Cmd for Publish {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        Ok(Context {
            project_dir,
            move_args,
            manifest: default_sourcemanifest(),
            manifest_hash: 0,
        })
    }

    fn apply(&mut self, _ctx: &mut Context) -> Result<()>
    where
        Self: Sized,
    {
        match &self {
            Publish::Module { params } => {
                let request = DataRequest::try_from(params)?;

                match &request.access {
                    // Publish Module (secret phrase)
                    AccessType::SecretPhrase(secret) => {
                        request
                            .client
                            .tx_mvm_publish_module(&request.file, request.gas, secret)
                    }

                    // publishing a module (test account)
                    AccessType::TestAccount(test_account) => request
                        .client
                        .tx_mvm_publish_module_dev(&request.file, request.gas, test_account),
                }
            }
            Publish::Package { params } => {
                let request = DataRequest::try_from(params)?;

                match &request.access {
                    // Publishing a package (secret phrase)
                    AccessType::SecretPhrase(secret) => {
                        request
                            .client
                            .tx_mvm_publish_package(&request.file, request.gas, secret)
                    }

                    // Publishing a package (test account)
                    AccessType::TestAccount(test_account) => request
                        .client
                        .tx_mvm_publish_package_dev(&request.file, request.gas, test_account),
                }
            }
        }
        .map(|address| {
            println!("Address: {}", address);
        })
    }
}

/// CLI: Entering a secret phrase
pub fn cli_entering_a_secret_phrase() -> Result<String> {
    println!("Please enter secret phrase:");

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;

    let key_phrase: Vec<&str> = buffer
        .trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if key_phrase.is_empty() {
        bail!("Secret phrase cannot be empty");
    }

    if ![12, 18, 24].into_iter().any(|num| num == key_phrase.len()) {
        bail!("Wrong number of words");
    }

    Ok(key_phrase.join(" "))
}

/// Checking for a key with this name and getting the content
pub fn cli_name_to_key(key_name: &str) -> Result<Option<String>> {
    // Checking for a saved key with this name
    if !secret_phrase::isset(key_name) {
        return Ok(None);
    }

    // Trying to get secret phrases without a password
    let mut phrase = secret_phrase::get(key_name, None);
    if phrase.is_err() {
        // Password required
        println!("Please enter password for key:");
        let password = rpassword::read_password()?.trim().to_string();
        phrase = secret_phrase::get(key_name, Some(&password)).map_err(|err| {
            debug!("{:?}", err);
            anyhow!("Invalid password")
        })
    }
    phrase.map(Some)
}

struct DataRequest {
    /// Client for connecting to "Pontem"
    client: PontemClient,

    /// Path to the file to be published
    file: String,

    /// Limitation of gas consumption per operation
    gas: u64,

    /// Access type - by secret phrase or through a test account
    access: AccessType,
}

impl TryFrom<&PublicationParameters> for DataRequest {
    type Error = anyhow::Error;

    fn try_from(params: &PublicationParameters) -> std::result::Result<Self, Self::Error> {
        let client = PontemClient::new(params.url_to_node.as_str())?;
        let file = params
            .file_path
            .canonicalize()
            .map_err(|err| {
                anyhow!(
                    r#"Path "{}" - {}"#,
                    params.file_path.display(),
                    err.to_string()
                )
            })?
            .to_string_lossy()
            .to_string();

        let access = if params.secret_phrase {
            // Request secret phrases
            let secret = cli_entering_a_secret_phrase()?;
            AccessType::SecretPhrase(secret)
        } else if let Some(test_account_or_name_key) = &params.account {
            match cli_name_to_key(test_account_or_name_key)? {
                Some(secret) => AccessType::SecretPhrase(secret),
                None => AccessType::TestAccount(test_account_or_name_key.to_owned()),
            }
        } else {
            bail!("Specify name of key or name of test account or secret phrase")
        };

        Ok(DataRequest {
            client,
            file,
            access,
            gas: params.gas_limit,
        })
    }
}

/// Access type - by secret phrase or through a test account
enum AccessType {
    SecretPhrase(String),
    TestAccount(String),
}
