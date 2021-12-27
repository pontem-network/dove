use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use move_cli::Move;
use subxt_client::SubxtClient;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;

/// Publishing a module or package.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(
    usage = "$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS] --account [ADDRESS] --url [URL]"
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
    usage = "$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS] --account [ADDRESS] --url [URL]\n
    Examples:
    $ dove publish module --file PATH/TO/MODULE.mv  --gas 100 
    $ dove publish package --file ./PATH/TO/PACKAGE.mv --gas 300 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
    $ dove publish module --file /PATH/TO/MODULE.mv  --gas 200 --account alice --url ws://127.0.0.1:9944
"
)]
pub struct PublicationParameters {
    /// The path to the module or package file.
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
                let client = SubxtClient::new(params.url.as_str(), &params.account)?;
                client.tx_mvm_publish_module_dev(
                    &params.file.as_os_str().to_string_lossy().to_string(),
                    params.gas,
                )
            }
            Publish::Package { params } => {
                let client = SubxtClient::new(params.url.as_str(), &params.account)?;
                client.tx_mvm_publish_package_dev(
                    &params.file.as_os_str().to_string_lossy().to_string(),
                    params.gas,
                )
            }
        }
        .map(|address| {
            println!("Address: {}", address);
            ()
        })
    }
}
