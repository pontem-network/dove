use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;

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

        let mut dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);

        dirs.push(tests_dir.clone());

        let index = ctx.build_index()?;

        // let dep_set = index.make_dependency_set(&dirs)?;
        // let mut dep_list = load_dependencies(dep_set)?;
        //
        // dep_list.extend(load_move_files(&dirs[..dirs.len() - 1])?);
        //
        // let executor = Executor::new(ctx.dialect.as_ref(), ctx.account_address()?, dep_list);
        //
        // let mut has_failures = false;
        //
        // for test in load_move_files(&[tests_dir])? {
        //     let test_name = Executor::script_name(&test)?;
        //
        //     if let Some(pattern) = &self.name_pattern {
        //         if !test_name.contains(pattern) {
        //             continue;
        //         }
        //     }
        //
        //     let is_err =
        //         render_test_result(&test_name, executor.execute_script(test, None, vec![]))?;
        //     if is_err {
        //         has_failures = true;
        //     }
        // }
        //
        // if has_failures {
        //     Err(anyhow!("tests failed:{}", ctx.project_name()))
        // } else {
        //     Ok(())
        // }
        todo!()
    }
}
