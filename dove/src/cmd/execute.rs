use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use move_cli::Move;
use subxt_client::SubxtClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;

/// Execute a transaction
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove execute --file [FILE_NAME] --gas [GAS] --account [ADDRESS] --url [URL]\n\
    Examples:
    $ dove execute --file PATH/TO/MODULE.mv  --gas 120 
    $ dove execute --file ./PATH/TO/PACKAGE.mv --gas 220 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
    $ dove execute --file /PATH/TO/MODULE.mv  --gas 110 --account alice --url ws://127.0.0.1:9944"
)]
pub struct Execute {
    /// The path to the transaction.
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
    /// Account from whom to publish
    #[structopt(long, default_value = "Alice")]
    account: String,
    /// The url of the substrate node to query
    #[structopt(long, parse(try_from_str), default_value = "ws://localhost:9944")]
    url: url::Url,
    /// Limitation of gas consumption per operation
    #[structopt(long)]
    gas: u64,
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
        let client = SubxtClient::new(self.url.as_str(), &self.account)?;
        client
            .tx_mvm_execute_dev(
                &self.file.as_os_str().to_string_lossy().to_string(),
                self.gas,
            )
            .map(|address| {
                println!("Address: {}", address);
                ()
            })
    }
}
