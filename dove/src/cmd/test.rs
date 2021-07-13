use anyhow::Error;
use structopt::StructOpt;

use lang::compiler::file::{find_move_files, MoveFile};
use move_executor::executor::{Executor, render_test_result};

use crate::cmd::Cmd;
use crate::context::Context;

/// Run tests.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Test {
    #[structopt(
        short = "k",
        long = "name-pattern",
        help = "Specify test name to run (or substring)"
    )]
    name_pattern: Option<String>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Test {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let tests_dir = ctx.path_for(&ctx.manifest.layout.tests_dir);
        if !tests_dir.exists() {
            return Ok(());
        }

        let mut deps = ctx.build_index()?.into_deps_roots();
        deps.push(
            ctx.path_for(&ctx.manifest.layout.scripts_dir)
                .to_string_lossy()
                .to_string(),
        );
        deps.push(
            ctx.path_for(&ctx.manifest.layout.modules_dir)
                .to_string_lossy()
                .to_string(),
        );

        let deps = find_move_files(&deps)
            .map(MoveFile::load)
            .collect::<Result<Vec<_>, _>>()?;

        let executor = Executor::new(ctx.dialect.as_ref(), ctx.account_address()?, deps);

        let mut has_failures = false;

        for test in find_move_files(&[tests_dir]) {
            let test = MoveFile::load(&test)?;
            let test_name = Executor::script_name(&test)?;

            if let Some(pattern) = &self.name_pattern {
                if !test_name.contains(pattern) {
                    continue;
                }
            }

            let is_err =
                render_test_result(&test_name, executor.execute_script(test, None, vec![]))?;
            if is_err {
                has_failures = true;
            }
        }

        if has_failures {
            Err(anyhow!("tests failed:{}", ctx.project_name()))
        } else {
            Ok(())
        }
    }
}
