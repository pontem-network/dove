use anyhow::{Context, Result};

use move_core_types::account_address::AccountAddress;

use move_lang::{compiled_unit::CompiledUnit, errors::Error, to_bytecode, FileCommentMap};

use vm::file_format::CompiledScript;
use vm::CompiledModule;

use crate::lang::{PreBytecodeProgram, ProgramCommentsMap};
use std::collections::BTreeMap;
use crate::shared::ProvidedAccountAddress;
use move_ir_types::location::Loc;
use utils::location;

fn split_around<'s>(s: &'s str, p: &str) -> (&'s str, &'s str) {
    let parts: Vec<_> = s.splitn(2, p).collect();
    let key = parts[0].trim();
    let val = parts[1].trim();
    (key, val)
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionMeta {
    pub signers: Vec<AccountAddress>,
    pub max_gas: u64,
}

impl ExecutionMeta {
    pub fn apply_meta_comment(&mut self, comment: String) {
        if !comment.contains(':') {
            return;
        }
        let (key, val) = split_around(&comment, ":");
        match key {
            "signer" => self
                .signers
                .push(AccountAddress::from_hex_literal(val).unwrap()),
            "max_gas" => {
                self.max_gas = val.parse().unwrap();
            }
            _ => todo!("Unimplemented meta key"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecutionUnit {
    Module(CompiledModule),
    Script((CompiledScript, ExecutionMeta)),
}

pub struct ExecutionSession {
    units: Vec<ExecutionUnit>,
}

impl ExecutionSession {
    pub fn new(units: Vec<ExecutionUnit>) -> Self {
        ExecutionSession { units }
    }

    pub fn into_script(self) -> Result<(Vec<u8>, ExecutionMeta)> {
        let mut serialized = vec![];
        for unit in self.units {
            if let ExecutionUnit::Script((script, meta)) = unit {
                script
                    .serialize(&mut serialized)
                    .context("Script serialization error")?;
                return Ok((serialized, meta));
            }
        }
        unreachable!()
    }

    pub fn modules(&self) -> Vec<CompiledModule> {
        let mut modules = vec![];
        for unit in &self.units {
            if let ExecutionUnit::Module(module) = unit {
                modules.push(module.to_owned())
            }
        }
        modules
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

    let mut doc_comment_candidate_line = script_start_line - 1;
    let mut doc_comments = vec![];
    for (span, comment) in file_comments.iter().rev() {
        let comment_start_line = file.position(span.start().to_usize()).unwrap().line;
        if comment_start_line == doc_comment_candidate_line {
            doc_comments.push(comment.trim().to_string());
            doc_comment_candidate_line -= 1;
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
        let execution_unit = match unit {
            CompiledUnit::Module {
                // ident: Spanned { loc, .. },
                module,
                ..
            } => ExecutionUnit::Module(module),

            CompiledUnit::Script {
                loc, script, key, ..
            } => {
                let mut meta = ExecutionMeta::default();
                if let Some((file_content, comments)) = program_doc_comments.get(loc.file()) {
                    let script_loc = script_loc_map.get(&key).unwrap().to_owned();
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

                ExecutionUnit::Script((script, meta))
            }
        };
        execution_units.push(execution_unit);
    }
    Ok(ExecutionSession {
        units: execution_units,
    })
}
