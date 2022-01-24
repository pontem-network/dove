use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use move_cli::Move;
use pontem_client::PontemClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::cmd::publish::cli_entering_a_secret_phrase;
use crate::context::Context;

/// Execute a transaction
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove execute --file [FILE_NAME] --gas [GAS] --secret --account [ADDRESS] --url [URL]\n
    Examples:
    $ dove execute --file PATH/TO/TRANSACTION.mvt  --gas 120 
    $ dove execute --file ./PATH/TO/TRANSACTION.mvt --gas 220 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
    $ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 110 --account alice --url ws://127.0.0.1:9944
    $ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 140 --secret
    "
)]
pub struct Execute {
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
        let file = self.file_path.as_os_str().to_string_lossy().to_string();

        if self.secret_phrase {
            let key_phrase = cli_entering_a_secret_phrase()?;
            client.tx_mvm_execute(&file, self.gas_limit, &key_phrase)
        } else if let Some(test_account) = &self.test_account {
            client.tx_mvm_execute_dev(&file, self.gas_limit, test_account)
        } else {
            bail!("Enter a secret phrase or a test account")
        }
        .map(|address| {
            println!("Address: {}", address);
        })
    }
}
