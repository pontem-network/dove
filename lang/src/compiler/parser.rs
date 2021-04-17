use anyhow::Result;
use move_lang::{strip_comments_and_verify, FileCommentMap, parser};

use move_lang::name_pool::ConstPool;
use std::collections::{HashMap, BTreeMap};
use move_lang::parser::syntax::parse_file_string;
use crate::compiler::source_map::{FileOffsetMap, ProjectOffsetMap, len_difference};
use crate::compiler::dialects::{Dialect, line_endings};
use crate::compiler::file::MoveFile;
use move_lang::errors::{FilesSourceText, Errors};
use move_core_types::account_address::AccountAddress;

pub type CommentsMap = BTreeMap<&'static str, FileCommentMap>;

pub struct ParserArtifact {
    pub meta: ParsingMeta,
    pub result: Result<parser::ast::Program, Errors>,
}

pub struct ParsingMeta {
    pub source_map: FilesSourceText,
    pub offsets_map: ProjectOffsetMap,
    pub comments: CommentsMap,
}

pub fn parse_program(
    dialect: &dyn Dialect,
    targets: &[MoveFile],
    deps: &[MoveFile],
    sender: Option<AccountAddress>,
) -> ParserArtifact {
    let mut files: FilesSourceText = HashMap::new();
    let mut source_definitions = Vec::new();
    let mut comment_map = CommentsMap::new();

    let mut lib_definitions = Vec::new();
    let mut project_offsets_map = ProjectOffsetMap::default();
    let mut errors: Errors = Vec::new();

    for target in targets {
        let name = ConstPool::push(target.name());
        let (defs, comments, es, offsets_map) =
            parse_file(dialect, &mut files, name, target.content(), sender);
        source_definitions.extend(defs);
        comment_map.insert(name, comments);
        project_offsets_map.0.insert(name, offsets_map);
        errors.extend(es);
    }

    for dep in deps {
        let name = ConstPool::push(&dep.name());
        let (defs, _, es, offsets_map) =
            parse_file(dialect, &mut files, name, dep.content(), sender);
        project_offsets_map.0.insert(name, offsets_map);
        lib_definitions.extend(defs);
        errors.extend(es);
    }

    let res = if errors.is_empty() {
        Ok(parser::ast::Program {
            source_definitions,
            lib_definitions,
        })
    } else {
        Err(errors)
    };

    ParserArtifact {
        meta: ParsingMeta {
            source_map: files,
            offsets_map: project_offsets_map,
            comments: comment_map,
        },
        result: res,
    }
}

pub fn parse_file(
    dialect: &dyn Dialect,
    files: &mut FilesSourceText,
    fname: &'static str,
    source_buffer: &str,
    sender: Option<AccountAddress>,
) -> (
    Vec<parser::ast::Definition>,
    FileCommentMap,
    Errors,
    FileOffsetMap,
) {
    let (source_buffer, file_source_map) = normalize_source_text(dialect, source_buffer, sender);
    let (no_comments_buffer, comment_map) = match strip_comments_and_verify(fname, &source_buffer)
    {
        Err(errors) => {
            files.insert(fname, source_buffer);
            return (vec![], Default::default(), errors, file_source_map);
        }
        Ok(result) => result,
    };

    files.insert(fname, source_buffer);
    match parse_file_string(fname, &no_comments_buffer, FileCommentMap::default()) {
        Ok((defs, _)) => (defs, comment_map, Vec::default(), file_source_map),
        Err(errors) => (vec![], comment_map, errors, file_source_map),
    }
}

fn normalize_source_text(
    dialect: &dyn Dialect,
    source_text: &str,
    sender: Option<AccountAddress>,
) -> (String, FileOffsetMap) {
    let (mut source_text, mut file_source_map) = line_endings::normalize(source_text);
    if let Some(sender) = sender {
        source_text = replace_sender_placeholder(source_text, sender, &mut file_source_map);
    }
    source_text = dialect.replace_addresses(source_text, &mut file_source_map);
    (source_text, file_source_map)
}

/// replace {{sender}} and {{ sender }} inside source code
fn replace_sender_placeholder(
    s: String,
    sender: AccountAddress,
    file_source_map: &mut FileOffsetMap,
) -> String {
    let address = format!("{:#x}", sender);

    let mut new_s = s;
    for template in &["{{sender}}", "{{ sender }}"] {
        while let Some(pos) = new_s.find(template) {
            new_s.replace_range(pos..pos + template.len(), &address);
            file_source_map.insert_layer(pos + sender.len(), len_difference(template, &address));
        }
    }
    new_s
}
