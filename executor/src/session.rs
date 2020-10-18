use std::collections::BTreeMap;

use anyhow::Error;
use move_core_types::gas_schedule::{CostTable, GasAlgebra, GasUnits};
use move_ir_types::location::Loc;
use move_lang::{compiled_unit::CompiledUnit, FileCommentMap};
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::values::Value;
use vm::CompiledModule;
use vm::file_format::CompiledScript;

use crate::execution::{execute_script, FakeRemoteCache};
use crate::explain::PipelineExecutionResult;
use crate::explain::StepExecutionResult;
use crate::meta::ExecutionMeta;
use lang::compiler::address::ProvidedAccountAddress;
use lang::compiler::parser::ParsingMeta;
use lang::compiler::{CompileFlow, CheckerResult, Step, compile, location};
use move_lang::errors::Errors;
use lang::compiler::dialects::Dialect;
use lang::compiler::file::MoveFile;
use lang::compiler::error::CompilerError;

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
    ) -> Result<PipelineExecutionResult, Error> {
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
    let script_start_line = file.position(script_loc.span().start()).unwrap().line;

    let mut doc_comment_candidate_line = match script_start_line.checked_sub(1) {
        Some(line) => line,
        None => {
            return vec![];
        }
    };
    let mut doc_comments = vec![];
    for (span, comment) in file_comments.iter().rev() {
        let comment_start_line = file.position(span.start()).unwrap().line;
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

pub struct SessionBuilder<'a> {
    dialect: &'a dyn Dialect,
    sender: &'a ProvidedAccountAddress,
    loc_map: Option<BTreeMap<String, Loc>>,
}

impl<'a> SessionBuilder<'a> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: &'a ProvidedAccountAddress,
    ) -> SessionBuilder<'a> {
        SessionBuilder {
            dialect,
            sender,
            loc_map: None,
        }
    }

    pub fn build(
        self,
        sources: &[MoveFile],
        deps: &[MoveFile],
    ) -> Result<ExecutionSession, CompilerError> {
        compile(self.dialect, sources, deps, Some(&self.sender), self)
    }
}

impl<'a> CompileFlow<Result<ExecutionSession, CompilerError>> for SessionBuilder<'a> {
    fn after_check(
        &mut self,
        meta: ParsingMeta,
        check_result: CheckerResult,
    ) -> Step<Result<ExecutionSession, CompilerError>, (ParsingMeta, CheckerResult)> {
        if check_result.is_ok() {
            let prog = check_result.as_ref().unwrap();
            let script_loc_map = prog
                .scripts
                .iter()
                .map(|(key, s)| (key.to_owned(), s.loc.to_owned()))
                .collect::<BTreeMap<_, _>>();
            self.loc_map = Some(script_loc_map);
        }
        Step::Next((meta, check_result))
    }

    fn after_translate(
        &mut self,
        meta: ParsingMeta,
        translation_result: Result<Vec<CompiledUnit>, Errors>,
    ) -> Result<ExecutionSession, CompilerError> {
        let mut execution_units = vec![];

        let ParsingMeta {
            source_map,
            offsets_map,
            comments,
        } = meta;
        let loc_map = self.loc_map.take().unwrap_or_default();

        let units = match translation_result {
            Ok(units) => units,
            Err(errors) => {
                return Err(CompilerError {
                    source_map,
                    errors: offsets_map.transform(errors),
                })
            }
        };

        for unit in units {
            let (loc, execution_unit) = match unit {
                CompiledUnit::Module { module, .. } => (None, ExecutionUnit::Module(module)),

                CompiledUnit::Script {
                    loc, script, key, ..
                } => {
                    let script_loc = loc_map.get(&key).unwrap().to_owned();
                    let mut meta = ExecutionMeta::default();
                    if let Some(comments) = comments.get(loc.file()) {
                        let source = source_map.get(loc.file()).map(|s| s.as_str()).unwrap_or("");
                        let doc_comments =
                            extract_script_doc_comments(script_loc, source, comments);
                        for doc_comment in doc_comments {
                            meta.apply_meta_comment(doc_comment)
                        }
                    }
                    // first signer is "sender" if no explicit "signer:" clauses passed
                    if meta.signers.is_empty() {
                        meta.signers.push(self.sender.as_account_address());
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
}
