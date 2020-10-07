use anyhow::{Context, Result};

use move_core_types::account_address::AccountAddress;

use move_lang::{compiled_unit::CompiledUnit, errors::Error, to_bytecode, FileCommentMap};

use vm::file_format::CompiledScript;
use vm::CompiledModule;

use std::collections::BTreeMap;
use utils::location;
use dialects::lang::{PreBytecodeProgram, ProgramCommentsMap};
use dialects::shared::ProvidedAccountAddress;
use move_ir_types::location::Loc;
use move_core_types::parser::parse_transaction_argument;
use move_core_types::transaction_argument::TransactionArgument;
use move_vm_types::values::Value;
use crate::oracles::oracle_metadata;
use move_core_types::language_storage::StructTag;

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
    pub oracle_prices: Vec<(StructTag, u128)>,
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
            "price" => {
                if !val.contains(' ') {
                    eprintln!("Invalid ticker price doc comment: {}", comment);
                    return;
                }
                let (tickers, value) = split_around(val, " ");
                if !tickers.contains('_') {
                    eprintln!("Invalid ticker price doc comment: {}", comment);
                    return;
                }
                let (ticker_left, ticker_right) = split_around(&tickers, "_");
                let price_struct_tag = oracle_metadata(ticker_left, ticker_right);
                self.oracle_prices
                    .push((price_struct_tag, value.parse().unwrap()))
            }
            _ => todo!("Unimplemented meta key"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecutionUnit {
    Module(CompiledModule),
    Script((String, CompiledScript, ExecutionMeta)),
}

pub struct ExecutionSession {
    units: Vec<ExecutionUnit>,
    arguments: Vec<String>,
}

impl ExecutionSession {
    pub fn new(units: Vec<ExecutionUnit>, arguments: Vec<String>) -> Self {
        ExecutionSession { units, arguments }
    }

    // pub fn into_first_script(self) -> (CompiledScript, ExecutionMeta) {
    //     for unit in self.units {
    //         if let ExecutionUnit::Script((name, script, meta)) = unit {
    //             return (script, meta);
    //         }
    //     }
    //     unreachable!()
    // }

    pub fn modules(&self) -> Vec<CompiledModule> {
        let mut modules = vec![];
        for unit in &self.units {
            if let ExecutionUnit::Module(module) = unit {
                modules.push(module.to_owned())
            }
        }
        modules
    }

    pub fn scripts(&self) -> Vec<(String, CompiledScript, ExecutionMeta)> {
        let mut scripts = vec![];
        for unit in &self.units {
            if let ExecutionUnit::Script((name, script, meta)) = unit {
                scripts.push((name.to_owned(), script.to_owned(), meta.to_owned()));
            }
        }
        scripts
    }

    pub fn arguments(&self) -> Result<Vec<Value>> {
        let mut script_args = Vec::with_capacity(self.arguments.len());
        for passed_arg in &self.arguments {
            let transaction_argument = parse_transaction_argument(passed_arg)?;
            let script_arg = convert_txn_arg(transaction_argument);
            script_args.push(script_arg);
        }
        Ok(script_args)
    }
}

pub fn serialize_script(script: &CompiledScript) -> Result<Vec<u8>> {
    let mut serialized = vec![];
    script
        .serialize(&mut serialized)
        .context("Script serialization error")?;
    Ok(serialized)
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

/// Convert the transaction arguments into move values.
pub fn convert_txn_arg(arg: TransactionArgument) -> Value {
    match arg {
        TransactionArgument::U64(i) => Value::u64(i),
        TransactionArgument::Address(a) => Value::address(a),
        TransactionArgument::Bool(b) => Value::bool(b),
        TransactionArgument::U8Vector(v) => Value::vector_u8(v),
        _ => unimplemented!(),
    }
}

pub fn init_execution_session(
    program: PreBytecodeProgram,
    program_doc_comments: ProgramCommentsMap,
    provided_sender: ProvidedAccountAddress,
    args: Vec<String>,
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
    Ok(ExecutionSession::new(
        execution_units.into_iter().map(|(_, unit)| unit).collect(),
        args,
    ))
}
