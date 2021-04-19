use lang::compiler::dialects::Dialect;
use lang::compiler::file::MoveFile;
use anyhow::Error;
use std::path::PathBuf;
use anyhow::anyhow;
use anyhow::Result;
use move_core_types::parser::parse_transaction_argument;
use move_core_types::transaction_argument::TransactionArgument;
use move_vm_types::values::Value;
use crate::explain::{PipelineExecutionResult, StepExecutionResult};
use crate::session::SessionBuilder;
use lang::compiler::error::CompilerError;
use move_lang::errors::report_errors;
use crate::format::format_step_result;
use move_core_types::account_address::AccountAddress;

pub struct Executor<'d, 'n, 'c> {
    dialect: &'d dyn Dialect,
    sender: AccountAddress,
    deps: Vec<MoveFile<'n, 'c>>,
}

impl<'d, 'n, 'c> Executor<'d, 'n, 'c> {
    pub fn new(
        dialect: &'d dyn Dialect,
        sender: AccountAddress,
        deps: Vec<MoveFile<'n, 'c>>,
    ) -> Executor<'d, 'n, 'c> {
        Executor {
            dialect,
            sender,
            deps,
        }
    }

    pub fn script_name(mvf: &MoveFile) -> Result<String, Error> {
        PathBuf::from(mvf.name())
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".move"))
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("Failed to extract script name:{}", mvf.name()))
    }

    pub fn execute_script(
        &self,
        script: MoveFile,
        args: Vec<String>,
    ) -> Result<PipelineExecutionResult, Error> {
        let script_args = parse_script_arguments(args)?;

        let mut sources = Vec::with_capacity(self.deps.len() + 1);
        sources.push(script);
        sources.extend(self.deps.clone());

        let session = SessionBuilder::new(self.dialect, &self.sender).build(&sources, &[])?;
        session.execute(script_args, self.dialect.cost_table())
    }
}

fn convert_txn_arg(arg: TransactionArgument) -> Result<Value> {
    Ok(match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => {
            return Err(anyhow::Error::msg(format!(
                "Unexpected transaction argument: {:?}",
                arg
            )));
        }
    })
}

fn parse_script_arguments(passed_args: Vec<String>) -> Result<Vec<Value>> {
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
                        textwrap::indent(&format_step_result(step_result, true, false), "    ",)
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
