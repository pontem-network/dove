use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use std::fmt::Debug;
use std::fs;
use crate::stdoutln;
use crate::transaction::{TransactionBuilder, Transaction};

/// Create transaction.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove tx [call] [OPTIONS]\n
    Examples:
    $ dove tx 'script_name<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)' --f file_name
    $ dove tx --file file_name --name script_name -a [10,10] true 68656c6c6f776f726c64 100 0x1 -type 0x01::Dfinance::USD
")]
pub struct CreateTransactionCmd {
    #[structopt(help = "Script call declaration.\
     Example: 'create_balance<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)'")]
    call: Option<String>,
    #[structopt(help = "Script name.", long = "name", short = "n")]
    script_name: Option<String>,
    #[structopt(help = "Output file name.", long = "output", short = "o")]
    output: Option<String>,
    #[structopt(help = "Script file name.", long = "file", short = "f")]
    file_name: Option<String>,
    #[structopt(
        help = r#"Script type parametrs, e.g. 0x1::Dfinance::USD"#,
        name = "Script type parameters.",
        long = "type",
        short = "t"
    )]
    type_parameters: Option<Vec<String>>,
    #[structopt(
        help = r#"Script arguments, e.g. 10 20 30"#,
        name = "Script arguments.",
        long = "args",
        short = "a"
    )]
    args: Option<Vec<String>>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for CreateTransactionCmd {
    fn apply(mut self, ctx: Context) -> Result<(), Error> {
        let output_filename = self.output.take();

        let builder = TransactionBuilder::from_create_transaction_cmd(self, &ctx)?;
        let (script_name, transaction) = builder.to_transaction()?;

        store_transaction(&ctx, &output_filename.unwrap_or(script_name), transaction)
    }
}

impl<'a> TransactionBuilder<'a> {
    /// Create a TransactionBuilder based on the transmitted data
    pub fn from_create_transaction_cmd(
        cmd: CreateTransactionCmd,
        ctx: &'a Context,
    ) -> Result<TransactionBuilder, Error> {
        let mut trbuild = Self::new(ctx, true);
        trbuild.script_file_name = cmd.file_name;
        trbuild
            .with_cmd_call(cmd.call.clone())?
            .with_cmd_script_name(cmd.script_name)
            .with_cmd_type_parameters(cmd.type_parameters)?
            .with_cmd_args(cmd.args);

        Ok(trbuild)
    }
}

fn store_transaction(ctx: &Context, name: &str, tx: Transaction) -> Result<(), Error> {
    let tx_dir = ctx.path_for(&ctx.manifest.layout.transactions_output);
    if !tx_dir.exists() {
        fs::create_dir_all(&tx_dir)?;
    }

    let mut tx_file = tx_dir.join(name);
    if !name.to_lowercase().ends_with(".mvt") {
        tx_file.set_extension("mvt");
    }

    if tx_file.exists() {
        fs::remove_file(&tx_file)?;
    }
    stdoutln!("Store transaction:{:?}", tx_file);
    Ok(fs::write(&tx_file, bcs::to_bytes(&tx)?)?)
}
