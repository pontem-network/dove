use anyhow::Error;
use structopt::StructOpt;

use lang::compiler::file::{load_move_files, MoveFile};
use move_executor::executor::{Executor, render_execution_result};

use crate::cmd::{Cmd, load_dependencies};
use crate::context::Context;

/// Run script.
#[derive(StructOpt, Debug)]
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
}

impl Cmd for Run {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let script_dir = ctx.path_for(&ctx.manifest.layout.script_dir);
        let script = script_dir.join(self.script).with_extension("move");
        if !script.exists() {
            return Err(anyhow!("Cannot open {:?}", script));
        }
        let module_dir = ctx.path_for(&ctx.manifest.layout.module_dir);

        let mut index = ctx.build_index()?;

        let dep_set = index.make_dependency_set(&[&script, &module_dir])?;
        let mut dep_list = load_dependencies(dep_set)?;
        dep_list.extend(load_move_files(&[module_dir])?);

        let signers = self
            .signers
            .iter()
            .map(|signer| ctx.dialect.parse_address(signer))
            .collect::<Result<Vec<_>, Error>>()?;

        let sender = self
            .signers
            .get(0)
            .map(|addr| ctx.dialect.parse_address(addr))
            .unwrap_or_else(|| ctx.account_address())?;

        let executor = Executor::new(ctx.dialect.as_ref(), sender, dep_list);
        let script = MoveFile::load(script)?;

        render_execution_result(executor.execute_script(script, Some(signers), self.args))
    }
}
