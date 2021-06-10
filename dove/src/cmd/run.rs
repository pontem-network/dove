use anyhow::Error;
use structopt::StructOpt;

use lang::compiler::file::{load_move_files, MoveFile};
use move_executor::executor::{Executor, render_execution_result};

use crate::cmd::{Cmd, load_dependencies};
use crate::context::Context;
use crate::stdoutln;

/// Run script.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Run {
    #[structopt(help = "Script file name.")]
    script: String,
    #[structopt(name = "Script signers.", long = "signers", short = "s")]
    signers: Vec<String>,
    #[structopt(
        help = r#"Number of script main() function arguments in quotes, e.g. "10 20 30""#,
        name = "Script args.",
        long = "args",
        short = "a"
    )]
    args: Vec<String>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Run {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let scripts_dir = ctx.path_for(&ctx.manifest.layout.scripts_dir);
        let script = scripts_dir.join(self.script).with_extension("move");
        if !script.exists() {
            return Err(anyhow!("Cannot open {:?}", script));
        }
        stdoutln!("Script: {}", script.display());
        let module_dir = ctx.path_for(&ctx.manifest.layout.modules_dir);

        stdoutln!("Build project index...");
        let mut index = ctx.build_index()?;

        let dep_set = index.make_dependency_set(&[&script, &module_dir])?;
        stdoutln!("Load dependencies...");
        let mut dep_list = load_dependencies(dep_set)?;
        dep_list.extend(load_move_files(&[module_dir])?);

        let mut signers = self
            .signers
            .iter()
            .map(|signer| ctx.dialect.parse_address(signer))
            .collect::<Result<Vec<_>, Error>>()?;

        if signers.is_empty() {
            signers.push(ctx.account_address()?);
        }
        stdoutln!("Load move files...");
        let executor = Executor::new(ctx.dialect.as_ref(), signers[0], dep_list);
        let script = MoveFile::load(script)?;

        stdoutln!();
        render_execution_result(executor.execute_script(script, Some(signers), self.args))
    }
}
