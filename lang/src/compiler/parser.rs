use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_lang::{FileCommentMap, parser, strip_comments_and_verify};
use move_lang::errors::{Errors, FilesSourceText};
use move_lang::name_pool::ConstPool;
use move_lang::parser::syntax::parse_file_string;

use crate::compiler::dialects::{Dialect, line_endings};
use crate::compiler::file::MoveFile;
use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::{FileOffsetMap, len_difference, ProjectOffsetMap};

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

pub fn parse_target(
    dialect: &dyn Dialect,
    targets: &[&MoveFile],
    sender: Option<AccountAddress>,
) -> ParserArtifact {
    let mut files: FilesSourceText = HashMap::new();
    let mut source_definitions = Vec::new();
    let mut comment_map = CommentsMap::new();

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

    let res = if errors.is_empty() {
        Ok(parser::ast::Program {
            source_definitions,
            lib_definitions: vec![],
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

pub fn parse_program(
    dialect: &dyn Dialect,
    parser_artifact: ParserArtifact,
    deps: &[MoveFile],
    sender: Option<AccountAddress>,
) -> ParserArtifact {
    let ParserArtifact {
        meta:
            ParsingMeta {
                mut source_map,
                offsets_map,
                comments,
            },
        result,
    } = parser_artifact;
    let mut project_offsets_map = offsets_map;

    match result {
        Ok(ast) => {
            let mut lib_definitions = Vec::new();
            let mut errors: Errors = Vec::new();
            for dep in deps {
                let name = ConstPool::push(&dep.name());
                let (defs, _, es, offsets_map) =
                    parse_file(dialect, &mut source_map, name, dep.content(), sender);
                project_offsets_map.0.insert(name, offsets_map);
                lib_definitions.extend(defs);
                errors.extend(es);
            }

            let result = if errors.is_empty() {
                Ok(parser::ast::Program {
                    source_definitions: ast.source_definitions,
                    lib_definitions,
                })
            } else {
                Err(errors)
            };
            ParserArtifact {
                meta: ParsingMeta {
                    source_map,
                    offsets_map: project_offsets_map,
                    comments,
                },
                result,
            }
        }
        Err(errors) => ParserArtifact {
            meta: ParsingMeta {
                source_map,
                offsets_map: project_offsets_map,
                comments,
            },
            result: Err(errors),
        },
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
    files.insert(fname, source_buffer.to_owned());

    let sender_str = sender.map(|sender| format!("{:#x}", sender));
    let mut mut_source = MutString::new(source_buffer);
    let file_source_map =
        normalize_source_text(dialect, (source_buffer, &mut mut_source), &sender_str);

    let mutated_source = mut_source.freeze();

    let (no_comments_buffer, comment_map) =
        match strip_comments_and_verify(fname, &mutated_source) {
            Err(errors) => {
                return (vec![], Default::default(), errors, file_source_map);
            }
            Ok(result) => result,
        };

    match parse_file_string(fname, &no_comments_buffer, FileCommentMap::default()) {
        Ok((defs, _)) => (defs, comment_map, Vec::default(), file_source_map),
        Err(errors) => (vec![], comment_map, errors, file_source_map),
    }
}

fn normalize_source_text<'a, 'b>(
    dialect: &dyn Dialect,
    (source_text, mut_str): (&'a str, &mut MutString<'a, 'b>),
    sender: &'b Option<String>,
) -> FileOffsetMap {
    let mut file_source_map = line_endings::normalize(mut_str);

    if let Some(sender) = sender {
        replace_sender_placeholder(mut_str, sender, &mut file_source_map);
    }
    dialect.replace_addresses(source_text, mut_str, &mut file_source_map);
    file_source_map
}

/// replace {{sender}} and {{ sender }} inside source code
fn replace_sender_placeholder<'a, 'b>(
    mut_str: &mut MutString<'a, 'b>,
    sender: &'b str,
    file_source_map: &mut FileOffsetMap,
) {
    for template in &["{{sender}}", "{{ sender }}"] {
        let mut last_pos = 0;
        while let Some(mut pos) = mut_str.as_ref()[last_pos..].find(template) {
            pos += last_pos;
            last_pos = pos + template.len();
            mut_str.make_patch(pos, pos + template.len(), NewValue::Borrowed(sender));
            file_source_map.insert_layer(pos + sender.len(), len_difference(template, sender));
        }
    }
}

#[cfg(test)]
mod test {
    use move_core_types::language_storage::CORE_CODE_ADDRESS;

    use crate::compiler::mut_string::MutString;
    use crate::compiler::parser::replace_sender_placeholder;
    use crate::compiler::source_map::FileOffsetMap;

    #[test]
    pub fn test_replace_sender_placeholder() {
        let source = r"
            script {
                use {{sender}}::Event;
                use {{ sender }}::Math;
                use {{sender}}::Invald;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        ";

        let mut source_map = FileOffsetMap::default();

        let mut mut_source = MutString::new(source);
        let sender_str = format!("{:#x}", CORE_CODE_ADDRESS);
        replace_sender_placeholder(&mut mut_source, &sender_str, &mut source_map);

        let expected = r"
            script {
                use 0x0000000000000000000000000000000000000000000000000000000000000001::Event;
                use 0x0000000000000000000000000000000000000000000000000000000000000001::Math;
                use 0x0000000000000000000000000000000000000000000000000000000000000001::Invald;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        ";
        assert_eq!(expected, mut_source.freeze());
    }
}
