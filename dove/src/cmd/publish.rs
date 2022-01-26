use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use move_cli::Move;
use pontem_client::PontemClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;

/// Publishing a module or package.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS]  --secret --account [ADDRESS] --url [URL]"
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
    $ dove publish module --file /PATH/TO/MODULE.mv --gas 200 --secret
    "
)]
pub struct PublicationParameters {
    /// The path to the transaction.
    #[structopt(short, long = "file", parse(from_os_str))]
    file_path: PathBuf,
    /// Account from whom to publish. Example: //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    #[structopt(long = "account", short = "t")]
    test_account: Option<String>,
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
        match self {
            Publish::Module { params } => {
                let client = PontemClient::new(params.url_to_node.as_str())?;
                let file = &params.file_path.as_os_str().to_string_lossy().to_string();

                if params.secret_phrase {
                    let key_phrase = cli_entering_a_secret_phrase()?;
                    client.tx_mvm_publish_module(file, params.gas_limit, &key_phrase)
                } else if let Some(test_account) = &params.test_account {
                    client.tx_mvm_publish_module_dev(file, params.gas_limit, test_account)
                } else {
                    bail!("Enter a secret phrase or a test account")
                }
            }
            Publish::Package { params } => {
                let client = PontemClient::new(params.url_to_node.as_str())?;
                let file = &params.file_path.as_os_str().to_string_lossy().to_string();

                if params.secret_phrase {
                    let key_phrase = cli_entering_a_secret_phrase()?;
                    client.tx_mvm_publish_package(file, params.gas_limit, &key_phrase)
                } else if let Some(test_account) = &params.test_account {
                    client.tx_mvm_publish_package_dev(file, params.gas_limit, test_account)
                } else {
                    bail!("Enter a secret phrase or a test account")
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
        .filter(|s| s.is_empty())
        .collect();
    if key_phrase.is_empty() {
        bail!("Secret phrase cannot be empty");
    }

    if ![12, 18, 24].into_iter().any(|num| num == key_phrase.len()) {
        bail!("Wrong number of words");
    }

    Ok(key_phrase.join(" "))
}
