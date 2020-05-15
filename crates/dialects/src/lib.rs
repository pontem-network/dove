use anyhow::Result;

use crate::dfinance::types::{Definition, Error};
use move_lang::shared::Address;
use move_lang::{cfgir, parser, to_bytecode};

use crate::dfinance::into_compiler_errors;
use crate::errors::CompilerError;
use move_lang::compiled_unit::CompiledUnit;
use utils::FilePath;
use vm::file_format::CompiledScript;
use vm::CompiledModule;

pub mod changes;
pub mod dfinance;
pub mod errors;
pub mod executor;

type PreBytecodeProgram = cfgir::ast::Program;

pub fn check_defs(
    source_definitions: Vec<Definition>,
    lib_definitions: Vec<Definition>,
    sender: Address,
) -> Result<PreBytecodeProgram, Vec<Error>> {
    let ast_program = parser::ast::Program {
        source_definitions,
        lib_definitions,
    };
    move_lang::check_program(Ok(ast_program), Some(sender))
}

pub fn generate_bytecode(
    program: PreBytecodeProgram,
) -> Result<(CompiledScript, Vec<CompiledModule>), Vec<Error>> {
    let mut units = to_bytecode::translate::program(program)?;
    let script = match units.remove(units.len() - 1) {
        CompiledUnit::Script { script, .. } => script,
        CompiledUnit::Module { .. } => unreachable!(),
    };
    let modules = units
        .into_iter()
        .map(|unit| match unit {
            CompiledUnit::Module { module, .. } => module,
            CompiledUnit::Script { .. } => unreachable!(),
        })
        .collect();
    Ok((script, modules))
}

pub fn check_with_compiler(
    current: (FilePath, String),
    deps: Vec<(FilePath, String)>,
    sender: [u8; dfinance::types::AccountAddress::LENGTH],
) -> Result<(), Vec<CompilerError>> {
    let (script_defs, dep_defs, offsets_map) =
        dfinance::parse_files(current, &deps).map_err(|errors| errors.apply_offsets())?;

    let sender = Address::new(sender);
    match check_defs(script_defs, dep_defs, sender) {
        Ok(_) => Ok(()),
        Err(errors) => Err(into_compiler_errors(errors, offsets_map).apply_offsets()),
    }
}

pub trait Dialect {
    fn validate_sender_address(s: String) -> Result<String>;
}

pub struct DFinanceDialect;

impl Dialect for DFinanceDialect {
    fn validate_sender_address(s: String) -> Result<String> {
        dfinance::types::AccountAddress::from_hex_literal(&s)?;
        Ok(s)
    }
}
