use move_lang::errors::{Errors, FilesSourceText};

use crate::compiler::dialects::{Dialect, line_endings};
use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::{FileOffsetMap, len_difference, ProjectOffsetMap};
use move_lang::interact::Interact;

pub struct BuilderPreprocessor<'a> {
    offsets_map: ProjectOffsetMap,
    dialect: &'a dyn Dialect,
    sender: &'a str,
    files: FilesSourceText,
}

impl<'a> BuilderPreprocessor<'a> {
    pub fn new(dialect: &'a dyn Dialect, sender: &'a str) -> BuilderPreprocessor<'a> {
        BuilderPreprocessor {
            offsets_map: Default::default(),
            dialect,
            sender,
            files: Default::default(),
        }
    }

    pub fn into_source(self) -> FilesSourceText {
        self.files
    }

    pub fn transform(&self, errors: Errors) -> Errors {
        self.offsets_map.transform(errors)
    }
}

impl<'a> Interact for BuilderPreprocessor<'a> {
    fn preprocess(&mut self, name: &'static str, source: String) -> String {
        let mut mut_source = MutString::new(&source);
        let file_source_map =
            normalize_source_text(self.dialect, (&source, &mut mut_source), self.sender);
        let post_processed_source = mut_source.freeze();

        self.offsets_map.insert(name, file_source_map);
        self.files.insert(name, source);

        post_processed_source
    }
}

pub fn normalize_source_text<'a, 'b>(
    dialect: &dyn Dialect,
    (source_text, mut_str): (&'a str, &mut MutString<'a, 'b>),
    sender: &'b str,
) -> FileOffsetMap {
    let mut file_source_map = line_endings::normalize(mut_str);
    replace_sender_placeholder(mut_str, sender, &mut file_source_map);
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
    use crate::compiler::preprocessor::replace_sender_placeholder;
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
