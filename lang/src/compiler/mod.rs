pub use anyhow::Result;
use codespan_reporting::term::termcolor::Buffer;
use move_core_types::account_address::AccountAddress;
use move_lang::{move_compile, move_check};
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::{Errors, FilesSourceText};
use move_model::{run_model_builder};
use move_model::model::GlobalEnv;
use crate::compiler::dialects::Dialect;
use crate::compiler::preprocessor::BuilderPreprocessor;
use move_lang::shared::Flags;
use codespan_reporting::diagnostic::Severity;
use move_ir_types::location::Loc;
use codespan::Span;

pub mod address;
pub mod dialects;
pub mod error;
pub mod file;
pub mod location;
pub mod metadata;
pub mod mut_string;
pub mod parser;
pub mod preprocessor;
pub mod source_map;

pub fn build_global_env(
    targets: Vec<String>,
    deps: Vec<String>,
    dialect: &dyn Dialect,
    sender: AccountAddress,
) -> anyhow::Result<GlobalEnv> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, Some(sender));

    let env: GlobalEnv = run_model_builder(&targets, &deps, &mut preprocessor)?;

    let mut error_writer = Buffer::no_color();
    if env.has_errors() {
        env.report_diag(&mut error_writer, Severity::Warning);
        println!("{}", String::from_utf8_lossy(&error_writer.into_inner()));
        return Err(anyhow!("exiting with checking errors"));
    }

    Ok(env)
}

pub fn build(
    targets: &[String],
    deps: &[String],
    dialect: &dyn Dialect,
    sender: Option<AccountAddress>,
    interface_files_dir: Option<String>,
    flags: Flags,
) -> anyhow::Result<(FilesSourceText, Result<Vec<CompiledUnit>, Errors>)> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);

    let (_, units_res) =
        move_compile(targets, deps, interface_files_dir, flags, &mut preprocessor)?;

    let units_res = units_res.map_err(|errors| preprocessor.transform(errors));
    Ok((preprocessor.into_source(), units_res))
}

pub fn check(
    targets: &[String],
    deps: &[String],
    dialect: &dyn Dialect,
    sender: Option<AccountAddress>,
    interface_files_dir: Option<String>,
    flags: Flags,
) -> Result<(), Errors> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);
    let (_, res) = move_check(targets, deps, interface_files_dir, flags, &mut preprocessor)
        .map_err(|err| vec![vec![(Loc::new("", Span::initial()), err.to_string())]])?;
    res.map_err(|errors| preprocessor.transform(errors))
}
