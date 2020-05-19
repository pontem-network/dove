use std::collections::BTreeMap;

use anyhow::Result;
use codespan::ByteIndex;
use dfinance_language_e2e_tests::data_store::FakeDataStore;
use dfinance_libra_types::{
    access_path::AccessPath, account_address::AccountAddress, vm_error::VMStatus,
    write_set::WriteSet,
};
use dfinance_move_core_types::gas_schedule::{GasAlgebra, GasUnits};
use dfinance_move_ir_types::location::Loc;
use dfinance_move_lang::{
    cfgir,
    compiled_unit::CompiledUnit,
    errors::{Error, FilesSourceText},
    parser,
    parser::ast::Definition,
    parser::syntax,
    shared::Address,
    strip_comments_and_verify, to_bytecode,
};
use dfinance_move_vm_runtime::MoveVM;
use dfinance_move_vm_state::execution_context::SystemExecutionContext;
use dfinance_move_vm_types::{
    gas_schedule::zero_cost_schedule,
    loaded_data::types::FatStructType,
    transaction_metadata::TransactionMetadata,
    values::{GlobalValue, Value},
};
use dfinance_vm::{file_format::CompiledScript, CompiledModule};

use shared::bech32;
use shared::errors::{
    CompilerError, CompilerErrorPart, ExecCompilerError, Location, OffsetsMap, ProjectOffsetsMap,
};
use shared::results::{ExecResult, ExecutionError};
use utils::FilePath;

pub mod executor;
pub mod resources;

fn from_compiler_error(comp_error: CompilerError) -> Error {
    comp_error
        .parts
        .into_iter()
        .map(|part| {
            let CompilerErrorPart {
                location: Location { fpath, span },
                message,
            } = part;
            (
                Loc::new(
                    fpath,
                    codespan::Span::new(ByteIndex(span.0 as u32), ByteIndex(span.1 as u32)),
                ),
                message,
            )
        })
        .collect()
}

pub fn report_errors(files: FilesSourceText, errors: Vec<CompilerError>) -> ! {
    let errors = errors.into_iter().map(from_compiler_error).collect();
    dfinance_move_lang::errors::report_errors(files, errors)
}

fn into_compiler_error(error: Error) -> CompilerError {
    let mut parts = vec![];
    for (loc, message) in error {
        let part = CompilerErrorPart {
            location: Location {
                fpath: loc.file(),
                span: (loc.span().start().to_usize(), loc.span().end().to_usize()),
            },
            message,
        };
        parts.push(part);
    }
    CompilerError { parts }
}

pub fn into_exec_compiler_error(
    errors: Vec<Error>,
    offsets_map: ProjectOffsetsMap,
) -> ExecCompilerError {
    let mut compiler_errors = vec![];
    for error in errors {
        compiler_errors.push(into_compiler_error(error));
    }
    ExecCompilerError(compiler_errors, offsets_map)
}

fn parse_file(
    fname: FilePath,
    text: &str,
) -> Result<(Vec<Definition>, OffsetsMap), ExecCompilerError> {
    let (no_comments_source, comment_map) =
        strip_comments_and_verify(fname, text).map_err(|errors| {
            into_exec_compiler_error(
                errors,
                ProjectOffsetsMap::with_file_map(fname, OffsetsMap::new()),
            )
        })?;
    let (bech32_converted_source, offsets_map) =
        bech32::replace_bech32_addresses(&no_comments_source);
    let (defs, _) = syntax::parse_file_string(fname, &bech32_converted_source, comment_map)
        .map_err(|errors| {
            into_exec_compiler_error(
                errors,
                ProjectOffsetsMap::with_file_map(fname, offsets_map.clone()),
            )
        })?;
    Ok((defs, offsets_map))
}

pub fn parse_files(
    current: (FilePath, String),
    deps: &[(FilePath, String)],
) -> Result<(Vec<Definition>, Vec<Definition>, ProjectOffsetsMap), ExecCompilerError> {
    let (s_fpath, s_text) = current;
    let mut exec_compiler_error = ExecCompilerError::default();

    let mut project_offsets_map = ProjectOffsetsMap::default();
    let script_defs = match parse_file(s_fpath, &s_text) {
        Ok((defs, offsets_map)) => {
            project_offsets_map.0.insert(s_fpath, offsets_map);
            defs
        }
        Err(error) => {
            exec_compiler_error.extend(error);
            vec![]
        }
    };

    let mut dep_defs = vec![];
    for (fpath, text) in deps.iter() {
        let defs = match parse_file(fpath, text) {
            Ok((defs, offsets_map)) => {
                project_offsets_map.0.insert(fpath, offsets_map);
                defs
            }
            Err(error) => {
                exec_compiler_error.extend(error);
                vec![]
            }
        };
        dep_defs.extend(defs);
    }
    if !exec_compiler_error.0.is_empty() {
        return Err(exec_compiler_error);
    }
    Ok((script_defs, dep_defs, project_offsets_map))
}

pub fn check_and_generate_bytecode(
    fname: FilePath,
    text: &str,
    deps: &[(FilePath, String)],
    sender: [u8; AccountAddress::LENGTH],
) -> Result<(CompiledScript, Vec<CompiledModule>), ExecCompilerError> {
    let (mut script_defs, modules_defs, project_offsets_map) =
        parse_files((fname, text.to_owned()), deps)?;
    script_defs.extend(modules_defs);

    let sender = Address::new(sender);
    let program = check_defs(script_defs, vec![], sender)
        .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
    generate_bytecode(program)
        .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map))
}

pub fn serialize_script(script: CompiledScript) -> Result<Vec<u8>> {
    let mut serialized = vec![];
    script.serialize(&mut serialized)?;
    Ok(serialized)
}

pub fn prepare_fake_network_state(
    modules: Vec<CompiledModule>,
    genesis_write_set: WriteSet,
) -> FakeDataStore {
    let mut network_state = FakeDataStore::default();
    for module in modules {
        network_state.add_module(&module.self_id(), &module);
    }
    network_state.add_write_set(&genesis_write_set);
    network_state
}

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

type ChangedMoveResources = BTreeMap<AccessPath, Option<(FatStructType, GlobalValue)>>;

fn vm_status_into_exec_status(vm_status: VMStatus) -> ExecutionError {
    ExecutionError {
        status: format!("{:?}", vm_status.major_status),
        sub_status: vm_status.sub_status,
        message: vm_status.message,
    }
}

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
) -> ExecResult<ChangedMoveResources> {
    let mut exec_context = SystemExecutionContext::new(data_store, GasUnits::new(1_000_000));
    let zero_cost_table = zero_cost_schedule();
    let txn_metadata = get_transaction_metadata(sender_address);

    let vm = MoveVM::new();
    vm.execute_script(
        script,
        &zero_cost_table,
        &mut exec_context,
        &txn_metadata,
        vec![],
        args,
    )
    .map_err(vm_status_into_exec_status)?;
    Ok(exec_context.data_map())
}

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
    dfinance_move_lang::check_program(Ok(ast_program), Some(sender))
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

pub fn parse_account_address(s: &str) -> Result<AccountAddress> {
    AccountAddress::from_hex_literal(s)
}

pub fn parse_address(s: &str) -> Result<Address> {
    Ok(Address::new(parse_account_address(s)?.into()))
}

pub fn check_with_compiler(
    current: (FilePath, String),
    deps: Vec<(FilePath, String)>,
    sender: &str,
) -> Result<(), Vec<CompilerError>> {
    let (script_defs, dep_defs, offsets_map) =
        parse_files(current, &deps).map_err(|errors| errors.apply_offsets())?;

    let sender_address = parse_address(sender).expect("Checked before");
    match check_defs(script_defs, dep_defs, sender_address) {
        Ok(_) => Ok(()),
        Err(errors) => Err(into_exec_compiler_error(errors, offsets_map).apply_offsets()),
    }
}
