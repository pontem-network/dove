use std::collections::BTreeMap;

use anyhow::Result;
use move_core_types::gas_schedule::{CostTable, GasAlgebra, GasUnits};
use move_ir_types::location::Loc;
use move_lang::{compiled_unit::CompiledUnit, errors::Error, FileCommentMap, to_bytecode};
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;
use vm::CompiledModule;
use vm::file_format::CompiledScript;
use utils::location;

use crate::execution::{execute_script, FakeRemoteCache};
use crate::explain::PipelineExecutionResult;
use crate::explain::StepExecutionResult;
use crate::meta::ExecutionMeta;
use lang::compiler::address::ProvidedAccountAddress;
use lang::compiler::parser::ProgramCommentsMap;

#[derive(Debug, Clone)]
pub enum ExecutionUnit {
    Module(CompiledModule),
    Script((String, CompiledScript, ExecutionMeta)),
}

pub struct ExecutionSession {
    units: Vec<ExecutionUnit>,
}

impl ExecutionSession {
    pub fn is_executable(&self) -> bool {
        for unit in &self.units {
            if let ExecutionUnit::Script(_) = unit {
                return true;
            }
        }
        false
    }

    pub fn execute(
        self,
        script_args: Vec<Value>,
        cost_table: CostTable,
    ) -> Result<PipelineExecutionResult> {
        let mut data_store = FakeRemoteCache::new(self.modules())?;
        let mut script_args = script_args;

        let mut overall_gas_spent = 0;
        let mut step_results = vec![];
        for (name, script, meta) in self.scripts() {
            let total_gas = 1_000_000;
            let mut cost_strategy =
                CostStrategy::transaction(&cost_table, GasUnits::new(total_gas));
            let step_result = execute_script(
                meta,
                &mut data_store,
                script,
                script_args,
                &mut cost_strategy,
            )?;
            script_args = vec![];

            let gas_spent = total_gas - cost_strategy.remaining_gas().get();
            overall_gas_spent += gas_spent;

            let is_error = matches!(step_result, StepExecutionResult::Error(_));
            step_results.push((name, step_result));
            if is_error {
                break;
            }
        }
        Ok(PipelineExecutionResult::new(
            step_results,
            overall_gas_spent,
        ))
    }

    fn modules(&self) -> Vec<CompiledModule> {
        let mut modules = vec![];
        for unit in &self.units {
            if let ExecutionUnit::Module(module) = unit {
                modules.push(module.to_owned())
            }
        }
        modules
    }

    fn scripts(&self) -> Vec<(String, CompiledScript, ExecutionMeta)> {
        let mut scripts = vec![];
        for unit in &self.units {
            if let ExecutionUnit::Script((name, script, meta)) = unit {
                scripts.push((name.to_owned(), script.to_owned(), meta.to_owned()));
            }
        }
        scripts
    }
}

pub fn extract_script_doc_comments(
    script_loc: Loc,
    file_content: &str,
    file_comments: &FileCommentMap,
) -> Vec<String> {
    let file = location::File::new(file_content);
    let script_start_line = file
        .position(script_loc.span().start().to_usize())
        .unwrap()
        .line;

    let mut doc_comment_candidate_line = match script_start_line.checked_sub(1) {
        Some(line) => line,
        None => {
            return vec![];
        }
    };
    let mut doc_comments = vec![];
    for (span, comment) in file_comments.iter().rev() {
        let comment_start_line = file.position(span.start().to_usize()).unwrap().line;
        if comment_start_line == doc_comment_candidate_line {
            doc_comments.push(comment.trim().to_string());
            doc_comment_candidate_line = match doc_comment_candidate_line.checked_sub(1) {
                Some(line) => line,
                None => {
                    break;
                }
            }
        }
    }
    doc_comments.reverse();
    doc_comments
}

pub fn init_execution_session(
    program: PreBytecodeProgram,
    program_doc_comments: ProgramCommentsMap,
    provided_sender: ProvidedAccountAddress,
) -> Result<ExecutionSession, Vec<Error>> {
    let script_loc_map: BTreeMap<_, _> = program
        .scripts
        .iter()
        .map(|(key, s)| (key.to_owned(), s.loc.to_owned()))
        .collect();
    let units = to_bytecode::translate::program(program)?;

    let mut execution_units = vec![];
    for unit in units {
        let (loc, execution_unit) = match unit {
            CompiledUnit::Module { module, .. } => (None, ExecutionUnit::Module(module)),

            CompiledUnit::Script {
                loc, script, key, ..
            } => {
                let script_loc = script_loc_map.get(&key).unwrap().to_owned();

                let mut meta = ExecutionMeta::default();
                if let Some((file_content, comments)) = program_doc_comments.get(loc.file()) {
                    let doc_comments =
                        extract_script_doc_comments(script_loc, file_content, comments);
                    for doc_comment in doc_comments {
                        meta.apply_meta_comment(doc_comment)
                    }
                }
                // first signer is "sender" if no explicit "signer:" clauses passed
                if meta.signers.is_empty() {
                    meta.signers.push(provided_sender.as_account_address());
                }
                (Some(script_loc), ExecutionUnit::Script((key, script, meta)))
            }
        };
        execution_units.push((loc, execution_unit));
    }
    execution_units.sort_by_key(|(loc, _)| match loc {
        Some(loc) => loc.span().end().to_usize(),
        None => 0,
    });
    Ok(ExecutionSession {
        units: execution_units.into_iter().map(|(_, unit)| unit).collect(),
    })
}
