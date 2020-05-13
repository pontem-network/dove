use crate::dfinance::types::{AccountAddress, Definition, Error};
use move_lang::parser;
use move_lang::shared::Address;
use std::collections::HashMap;
use std::path::Path;

pub mod dfinance;
pub mod libra;

pub type FilePath = &'static str;
pub type FilesSourceText = HashMap<&'static str, String>;

#[derive(Debug, Clone)]
pub struct Location {
    pub fpath: FilePath,
    pub span: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct CompilerErrorPart {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub parts: Vec<CompilerErrorPart>,
}

impl From<Error> for CompilerError {
    fn from(error: Error) -> Self {
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
}

pub fn leaked_fpath<P: AsRef<Path>>(path: P) -> FilePath {
    let s = path.as_ref().to_str().unwrap();
    Box::leak(Box::new(s.to_owned()))
}

fn parse_files(
    current: (FilePath, String),
    deps: Vec<(FilePath, String)>,
) -> Result<
    (
        Vec<dfinance::types::Definition>,
        Vec<dfinance::types::Definition>,
    ),
    Vec<CompilerError>,
> {
    let (s_fpath, s_text) = current;
    let mut parse_errors: Vec<CompilerError> = vec![];

    let script_defs = match dfinance::parse_file(s_fpath, &s_text) {
        Ok(parsed) => parsed,
        Err(errors) => {
            for error in errors {
                parse_errors.push(error.into());
            }
            vec![]
        }
    };

    let mut dep_defs = vec![];
    for (fpath, text) in deps.into_iter() {
        let defs = match dfinance::parse_file(fpath, &text) {
            Ok(parsed) => parsed,
            Err(errors) => {
                for error in errors {
                    parse_errors.push(error.into());
                }
                vec![]
            }
        };
        dep_defs.extend(defs);
    }
    if !parse_errors.is_empty() {
        return Err(parse_errors);
    }
    Ok((script_defs, dep_defs))
}

fn check_parsed_program(
    current_file_defs: Vec<Definition>,
    dependencies: Vec<Definition>,
    sender: [u8; AccountAddress::LENGTH],
) -> Result<(), Vec<CompilerError>> {
    let ast_program = parser::ast::Program {
        source_definitions: current_file_defs,
        lib_definitions: dependencies,
    };
    let sender_address = Address::new(sender);
    match move_lang::check_program(Ok(ast_program), Some(sender_address)) {
        Ok(_) => Ok(()),
        Err(errors) => Err(errors.into_iter().map(CompilerError::from).collect()),
    }
}

pub fn check_with_compiler(
    current: (FilePath, String),
    deps: Vec<(FilePath, String)>,
    sender: [u8; dfinance::types::AccountAddress::LENGTH],
) -> Result<(), Vec<CompilerError>> {
    let (script_defs, dep_defs) = parse_files(current, deps)?;
    check_parsed_program(script_defs, dep_defs, sender)
}
