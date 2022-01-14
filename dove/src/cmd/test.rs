use structopt::StructOpt;
use move_core_types::errmap::ErrorMapping;
use move_cli::{run_cli, Command as MoveCommand};
use move_cli::package::cli::PackageCommand;
use diem_types::account_address::AccountAddress;
use crate::cmd::Cmd;
use crate::cmd::build::run_internal_build;
use crate::context::Context;

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
    instruction_execution_bound: u64,

    /// A filter string to determine which unit tests to run
    #[structopt(name = "filter", short = "f", long = "filter")]
    filter: Option<String>,

    /// List all tests
    #[structopt(name = "list", short = "l", long = "list")]
    list: bool,

    /// Number of threads to use for running tests.
    #[structopt(
        name = "num_threads",
        default_value = "8",
        short = "t",
        long = "threads"
    )]
    num_threads: usize,

    /// Report test statistics at the end of testing
    #[structopt(name = "report_statistics", short = "s", long = "statistics")]
    report_statistics: bool,

    /// Show the storage state at the end of execution of a failing test
    #[structopt(name = "global_state_on_error", short = "g", long = "state_on_error")]
    report_storage_on_error: bool,

    /// Use the stackless bytecode interpreter to run the tests and cross check its results with
    /// the execution result from Move VM.
    #[structopt(long = "stackless")]
    check_stackless_vm: bool,

    /// Verbose mode
    #[structopt(long = "verbose")]
    verbose_mode: bool,

    /// Compute coverage
    #[structopt(long = "coverage")]
    compute_coverage: bool,
}

impl Cmd for Test {
    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        // Build a project
        // In order for the dependencies to be loaded
        run_internal_build(ctx)?;

        // for Move-cli
        let error_descriptions: ErrorMapping =
            bcs::from_bytes(move_stdlib::error_descriptions())?;

        let cmd = MoveCommand::Package {
            cmd: PackageCommand::UnitTest {
                instruction_execution_bound: self.instruction_execution_bound,
                filter: self.filter.clone(),
                list: self.list,
                num_threads: self.num_threads,
                report_statistics: self.report_statistics,
                report_storage_on_error: self.report_storage_on_error,
                check_stackless_vm: self.check_stackless_vm,
                verbose_mode: self.verbose_mode,
                compute_coverage: self.compute_coverage,
            },
        };

        run_cli(
            move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1").unwrap()),
            &error_descriptions,
            &ctx.move_args,
            &cmd,
        )?;

        Ok(())
    }
}
