use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use move_cli::Move;
use pontem_client::PontemClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::cmd::publish::{cli_entering_a_secret_phrase, cli_name_to_key};
use crate::context::Context;

/// Execute a transaction
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove execute --file [FILE_NAME] --gas [GAS] --secret --account [NAME_OR_ADDRESS] --url [URL]\n
    Examples:
    $ dove execute --file PATH/TO/TRANSACTION.mvt  --gas 120 
    $ dove execute --file ./PATH/TO/TRANSACTION.mvt --gas 220 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
    $ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 110 --account alice --url ws://127.0.0.1:9944
    $ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 130 --account NAME_SECRET_KEY
    $ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 140 --secret
    "
)]
pub struct Execute {
    /// The path to the transaction.
    #[structopt(short, long = "file", parse(from_os_str))]
    file_path: PathBuf,

    /// Account from whom to publish. Address or test account name or name secret key. Example: //Alice, alice, bob, NAME_SECRET_KEY... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    #[structopt(long = "account", short = "t")]
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

impl Cmd for Execute {
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
        let client = PontemClient::new(self.url_to_node.as_str())?;
        let transaction_path = self.file_path.as_os_str().to_string_lossy().to_string();

        if self.secret_phrase {
            // Request secret phrases
            let phrase = cli_entering_a_secret_phrase()?;
            // Transaction execution (secret phrase)
            client.tx_mvm_execute(&transaction_path, self.gas_limit, &phrase)
        } else if let Some(account_or_name_key) = &self.account {
            match cli_name_to_key(account_or_name_key)? {
                Some(phrase) => {
                    // Transaction execution (secret phrase)
                    client.tx_mvm_execute(&transaction_path, self.gas_limit, &phrase)
                }
                None => {
                    // Transaction execution (test account)
                    client.tx_mvm_execute_dev(
                        &transaction_path,
                        self.gas_limit,
                        account_or_name_key,
                    )
                }
            }
        } else {
            bail!("Specify name of key or name of test account or secret phrase")
        }
        .map(|address| {
            println!("Address: {}", address);
        })
    }
}
