use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_core_types::parser::parse_transaction_argument;
use move_core_types::transaction_argument::TransactionArgument;
use move_lang::errors::report_errors;

use lang::compiler::dialects::Dialect;
use lang::compiler::error::CompilerError;

use crate::explain::{PipelineExecutionResult, StepExecutionResult};
use crate::format::format_step_result;

pub struct Executor<'d> {
    dialect: &'d dyn Dialect,
    sender: AccountAddress,
    deps: Vec<String>,
}

impl<'d> Executor<'d> {
    pub fn new(
        dialect: &'d dyn Dialect,
        sender: AccountAddress,
        deps: Vec<String>,
    ) -> Executor<'d> {
        Executor {
            dialect,
            sender,
            deps,
        }
    }

    pub fn script_name(mvf: &String) -> Result<String, Error> {
        PathBuf::from(mvf)
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".move"))
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("Failed to extract script name:{}", mvf))
    }

    pub fn execute_script(
        &self,
        script: String,
        signers: Option<Vec<AccountAddress>>,
        args: Vec<String>,
    ) -> Result<PipelineExecutionResult, Error> {
        let script_args = parse_script_arguments(args)?;

        let mut sources = Vec::with_capacity(self.deps.len() + 1);
        sources.push(script);

        for dep in &self.deps {
            sources.push(dep.clone());
        }

        todo!()
        // let session = SessionBuilder::new(self.dialect, self.sender).build(sources, false)?;
        // session.execute(signers, script_args, self.dialect.cost_table())
    }
}

fn convert_txn_arg(arg: TransactionArgument) -> Result<Vec<u8>> {
    match arg {
        TransactionArgument::U64(v) => bcs::to_bytes(&v),
        TransactionArgument::Address(v) => bcs::to_bytes(&v),
        TransactionArgument::Bool(v) => bcs::to_bytes(&v),
        TransactionArgument::U8Vector(v) => bcs::to_bytes(&v),
        TransactionArgument::U8(v) => bcs::to_bytes(&v),
        TransactionArgument::U128(v) => bcs::to_bytes(&v),
    }
    .map_err(|err| err.into())
}

fn parse_script_arguments(passed_args: Vec<String>) -> Result<Vec<Vec<u8>>> {
    passed_args
        .into_iter()
        .map(|arg| parse_transaction_argument(&arg).and_then(convert_txn_arg))
        .collect()
}

pub fn render_test_result(
    test_name: &str,
    result: Result<PipelineExecutionResult, Error>,
) -> Result<bool> {
    let exec_result = result.map_err(|err| match err.downcast::<CompilerError>() {
        Ok(compiler_error) => report_errors(compiler_error.source_map, compiler_error.errors),
        Err(error) => error,
    })?;

    Ok(match exec_result.last() {
        None => {
            println!("{} ....... SCRIPT_NOT_FOUND", test_name);
            false
        }
        Some(step_result) => match step_result {
            StepExecutionResult::Error(_) => {
                println!("{} .......", test_name);

                for step_result in exec_result.step_results {
                    print!(
                        "{}",
                        textwrap::indent(&format_step_result(step_result, true, false), "    ")
                    );
                }
                println!();
                true
            }
            StepExecutionResult::ExpectedError(_) | StepExecutionResult::Success(_) => {
                println!("{} ....... ok", test_name);
                false
            }
        },
    })
}

pub fn render_execution_result(result: Result<PipelineExecutionResult, Error>) -> Result<()> {
    match result {
        Ok(exec_result) => {
            let step_results = exec_result.step_results;
            for (i, step_result) in step_results.into_iter().enumerate() {
                if i > 0 {
                    println!();
                }
                print!("{}", format_step_result(step_result, true, true));
            }
            Ok(())
        }
        Err(err) => match err.downcast::<CompilerError>() {
            Ok(compiler_error) => report_errors(compiler_error.source_map, compiler_error.errors),
            Err(error) => Err(error),
        },
    }
}
