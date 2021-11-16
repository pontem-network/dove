use structopt::StructOpt;

// use lang::compiler::file::find_move_files;

use crate::cmd::Cmd;
use crate::context::Context;
// use lang::compiler::preprocessor::BuilderPreprocessor;
// use move_unit_test::UnitTestingConfig;

/// Run tests.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Test {
    /// Bound the number of instructions that can be executed by any one test.
    #[structopt(
        name = "instructions",
        default_value = "5000",
        short = "i",
        long = "instructions"
    )]
    pub instruction_execution_bound: u64,

    /// A filter string to determine which unit tests to run
    #[structopt(name = "filter", short = "f", long = "filter")]
    pub filter: Option<String>,

    /// List all tests
    #[structopt(name = "list", short = "l", long = "list")]
    pub list: bool,

    /// Number of threads to use for running tests.
    #[structopt(
        name = "num_threads",
        default_value = "8",
        short = "t",
        long = "threads"
    )]
    pub num_threads: usize,

    /// Report test statistics at the end of testing
    #[structopt(name = "report_statistics", short = "s", long = "statistics")]
    pub report_statistics: bool,

    /// Show the storage state at the end of execution of a failing test
    #[structopt(name = "global_state_on_error", short = "g", long = "state_on_error")]
    pub report_storage_on_error: bool,

    /// Use the stackless bytecode interpreter to run the tests and cross check its results with
    /// the execution result from Move VM.
    #[structopt(long = "stackless")]
    pub check_stackless_vm: bool,

    /// Color mode.
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Test {
    fn apply(&mut self, ctx: Context) -> anyhow::Result<()> where Self: Sized {
        todo!()
    }
    // fn apply(self, ctx: Context) -> Result<(), Error> {
    //     let tests_dir = ctx.path_for(&ctx.manifest.layout.tests_dir);
    //     if !tests_dir.exists() {
    //         return Ok(());
    //     }
    //
    //     let mut deps = ctx.build_index()?.0.into_deps_roots();
    //     deps.push(tests_dir.to_string_lossy().to_string());
    //     deps.push(
    //         ctx.path_for(&ctx.manifest.layout.modules_dir)
    //             .to_string_lossy()
    //             .to_string(),
    //     );
    //
    //     let source_files = find_move_files(&deps)
    //         .into_iter()
    //         .map(|p| p.to_string_lossy().to_string())
    //         .collect::<Vec<_>>();
    //
    //     let unit_test_config = UnitTestingConfig {
    //         instruction_execution_bound: self.instruction_execution_bound,
    //         filter: self.filter,
    //         list: self.list,
    //         num_threads: self.num_threads,
    //         report_statistics: self.report_statistics,
    //         report_storage_on_error: self.report_storage_on_error,
    //         source_files,
    //         check_stackless_vm: self.check_stackless_vm,
    //         verbose: self.verbose,
    //     };
    //
    //     let address = ctx.account_address_str()?;
    //     let mut preprocessor = BuilderPreprocessor::new(ctx.dialect.as_ref(), &address);
    //     let test_plan = unit_test_config.build_test_plan(&mut preprocessor);
    //     if let Some(test_plan) = test_plan {
    //         let (_, is_ok) =
    //             unit_test_config.run_and_report_unit_tests(test_plan, std::io::stdout())?;
    //         if !is_ok {
    //             bail!("Tests failed: {}", ctx.project_name());
    //         }
    //     }
    //
    //     Ok(())
    // }
}
