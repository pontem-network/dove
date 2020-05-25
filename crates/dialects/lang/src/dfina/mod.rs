use std::path::PathBuf;

use anyhow::Result;
use codespan::ByteIndex;
use dfin_libra_types::account_address::AccountAddress;
use dfin_move_ir_types::location::Loc;
use dfin_move_lang::{
    cfgir,
    errors::{Error, FilesSourceText},
    parser,
    parser::ast::Definition,
    parser::syntax,
    shared::Address,
    strip_comments_and_verify,
};

use shared::bech32;
use shared::errors::{
    len_difference, CompilerError, CompilerErrorPart, ExecCompilerError, Location, OffsetsMap,
    ProjectOffsetsMap,
};
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
    dfin_move_lang::errors::report_errors(files, errors)
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

fn is_inside_libra_directory() -> bool {
    let path = PathBuf::from(file!());
    path.parent().unwrap().to_str().unwrap().ends_with("libra")
}

/// replace {{sender}} and {{ sender }} inside source code
fn replace_sender_placeholder(s: String, sender: &str, offsets_map: &mut OffsetsMap) -> String {
    assert!(
        sender.len() > 12,
        "Sender address length is too short: {}",
        sender.len()
    );
    let mut new_s = s;
    for template in &["{{sender}}", "{{ sender }}"] {
        while let Some(pos) = new_s.find(template) {
            new_s.replace_range(pos..pos + template.len(), sender);
            offsets_map.insert_layer(pos + sender.len(), len_difference(template, sender));
        }
    }
    new_s
}

fn parse_file(
    fname: FilePath,
    source_text: &str,
    sender: &str,
) -> Result<(Vec<Definition>, OffsetsMap), ExecCompilerError> {
    let (mut source_text, comment_map) =
        strip_comments_and_verify(fname, source_text).map_err(|errors| {
            into_exec_compiler_error(
                errors,
                ProjectOffsetsMap::with_file_map(fname, OffsetsMap::default()),
            )
        })?;

    let mut offsets_map = OffsetsMap::default();
    source_text = replace_sender_placeholder(source_text, sender, &mut offsets_map);
    if !is_inside_libra_directory() {
        source_text = bech32::replace_bech32_addresses(&source_text, &mut offsets_map);
    }

    let (defs, _) =
        syntax::parse_file_string(fname, &source_text, comment_map).map_err(|errors| {
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
    sender: String,
) -> Result<(Vec<Definition>, Vec<Definition>, ProjectOffsetsMap), ExecCompilerError> {
    let mut sender = sender;
    if sender.len() < 12 {
        // short form libra address
        let as_address = AccountAddress::from_hex_literal(&sender).unwrap();
        sender = format!("0x{}", as_address);
    }

    let (s_fpath, s_text) = current;
    let mut exec_compiler_error = ExecCompilerError::default();

    let mut project_offsets_map = ProjectOffsetsMap::default();
    let script_defs = match parse_file(s_fpath, &s_text, &sender) {
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
        let defs = match parse_file(fpath, text, &sender) {
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
    dfin_move_lang::check_program(Ok(ast_program), Some(sender))
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
    let (script_defs, dep_defs, offsets_map) = parse_files(current, &deps, sender.to_string())
        .map_err(|errors| errors.apply_offsets())?;

    let sender_address = parse_address(sender).expect("Checked before");
    match check_defs(script_defs, dep_defs, sender_address) {
        Ok(_) => Ok(()),
        Err(errors) => Err(into_exec_compiler_error(errors, offsets_map).apply_offsets()),
    }
}
