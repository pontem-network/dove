use anyhow::Result;

use language_e2e_tests::data_store::FakeDataStore;

use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;

use libra_types::write_set::WriteSet;
use move_core_types::gas_schedule::{GasAlgebra, GasUnits};

use move_lang::errors::{Error, FilesSourceText};
use move_lang::parser::ast::Definition;
use move_lang::parser::syntax;

use move_lang::strip_comments_and_verify;
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::SystemExecutionContext;
use move_vm_types::gas_schedule::zero_cost_schedule;
use move_vm_types::loaded_data::types::FatStructType;
use move_vm_types::transaction_metadata::TransactionMetadata;
use move_vm_types::values::{GlobalValue, Value};
use std::collections::BTreeMap;
use utils::FilePath;

use crate::dfinance::types::Loc;
use crate::errors::{
    CompilerError, CompilerErrorPart, CompilerErrors, Location, OffsetsMap, ProjectOffsetsMap,
};
use crate::{check_defs, generate_bytecode};
use codespan::ByteIndex;
use move_lang::shared::Address;
use vm::errors::VMResult;
use vm::file_format::CompiledScript;
use vm::CompiledModule;

pub mod bech32;
pub mod types;

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
    move_lang::errors::report_errors(files, errors)
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

pub fn into_compiler_errors(
    errors: Vec<Error>,
    offsets_map: ProjectOffsetsMap,
) -> CompilerErrors {
    let mut compiler_errors = vec![];
    for error in errors {
        compiler_errors.push(into_compiler_error(error));
    }
    CompilerErrors::new(compiler_errors, offsets_map)
}

fn parse_file(
    fname: FilePath,
    text: &str,
) -> Result<(Vec<Definition>, OffsetsMap), CompilerErrors> {
    let (no_comments_source, comment_map) =
        strip_comments_and_verify(fname, text).map_err(|errors| {
            into_compiler_errors(
                errors,
                ProjectOffsetsMap::with_file_map(fname, OffsetsMap::new()),
            )
        })?;
    let (bech32_converted_source, offsets_map) =
        bech32::replace_bech32_addresses(&no_comments_source);
    let (defs, _) = syntax::parse_file_string(fname, &bech32_converted_source, comment_map)
        .map_err(|errors| {
            into_compiler_errors(
                errors,
                ProjectOffsetsMap::with_file_map(fname, offsets_map.clone()),
            )
        })?;
    Ok((defs, offsets_map))
}

pub fn parse_files(
    current: (FilePath, String),
    deps: &[(FilePath, String)],
) -> Result<
    (
        Vec<types::Definition>,
        Vec<types::Definition>,
        ProjectOffsetsMap,
    ),
    CompilerErrors,
> {
    let (s_fpath, s_text) = current;
    let mut parse_errors = CompilerErrors::default();

    let mut project_offsets_map = ProjectOffsetsMap::default();
    let script_defs = match parse_file(s_fpath, &s_text) {
        Ok((defs, offsets_map)) => {
            project_offsets_map.0.insert(s_fpath, offsets_map);
            defs
        }
        Err(errors) => {
            parse_errors.extend(errors);
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
            Err(errors) => {
                parse_errors.extend(errors);
                vec![]
            }
        };
        dep_defs.extend(defs);
    }
    if !parse_errors.0.is_empty() {
        return Err(parse_errors);
    }
    Ok((script_defs, dep_defs, project_offsets_map))
}

pub fn check_and_generate_bytecode(
    fname: FilePath,
    text: &str,
    deps: &[(FilePath, String)],
    sender: [u8; AccountAddress::LENGTH],
) -> Result<(CompiledScript, Vec<CompiledModule>), Vec<CompilerError>> {
    let (mut script_defs, modules_defs, project_offsets_map) =
        parse_files((fname, text.to_owned()), deps).map_err(|errors| errors.apply_offsets())?;
    script_defs.extend(modules_defs);
    let sender = Address::new(sender);
    let program = check_defs(script_defs, vec![], sender).map_err(|errors| {
        into_compiler_errors(errors, project_offsets_map.clone()).apply_offsets()
    })?;
    generate_bytecode(program)
        .map_err(|errors| into_compiler_errors(errors, project_offsets_map).apply_offsets())
}

pub fn serialize_script(script: CompiledScript) -> Vec<u8> {
    let mut serialized = vec![];
    script.serialize(&mut serialized).unwrap();
    serialized
}

pub fn prepare_fake_network_state(
    modules: Vec<CompiledModule>,
    genesis_changes_writeset: WriteSet,
) -> FakeDataStore {
    let mut network_state = FakeDataStore::default();
    for module in modules {
        network_state.add_module(&module.self_id(), &module);
    }
    network_state.add_write_set(&genesis_changes_writeset);
    network_state
}

fn get_transaction_metadata(sender_address: AccountAddress) -> TransactionMetadata {
    let mut metadata = TransactionMetadata::default();
    metadata.sender = sender_address;
    metadata
}

type ChangedMoveResources = BTreeMap<AccessPath, Option<(FatStructType, GlobalValue)>>;

pub fn execute_script(
    sender_address: AccountAddress,
    data_store: &FakeDataStore,
    script: Vec<u8>,
    args: Vec<Value>,
) -> VMResult<ChangedMoveResources> {
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
    )?;
    Ok(exec_context.data_map())
}
