#[derive(Debug, Clone)]
pub struct UnitTestingConfig {
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

    /// Source files
    #[structopt(name = "sources")]
    pub source_files: Vec<String>,

    /// Use the stackless bytecode interpreter to run the tests and cross check its results with
    /// the execution result from Move VM.
    #[structopt(long = "stackless")]
    pub check_stackless_vm: bool,

    /// Verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for UnitTestingConfig {
    fn default() -> Self {
        U
    }
}
