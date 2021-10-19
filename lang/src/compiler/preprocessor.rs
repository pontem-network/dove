use move_lang::errors::{Errors, FilesSourceText};

use crate::compiler::dialects::{Dialect, line_endings};
use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::{FileOffsetMap, len_difference, ProjectOffsetMap};
use move_lang::callback::Interact;
use std::borrow::Cow;
use std::mem;
use regex::Regex;

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

    pub fn into_source(&mut self) -> FilesSourceText {
        mem::take(&mut self.files)
    }

    pub fn transform(&self, errors: Errors) -> Errors {
        self.offsets_map.transform(errors)
    }
}

impl<'a> Interact for BuilderPreprocessor<'a> {
    fn preprocess<'b>(&mut self, name: &'static str, source: Cow<'b, str>) -> Cow<'b, str> {
        let source = attr_dialect(&self.dialect.name().to_string(), source);

        let mut mut_source = MutString::new(&source);
        let file_source_map =
            normalize_source_text(self.dialect, (&source, &mut mut_source), self.sender);
        let post_processed_source = mut_source.freeze();
        self.offsets_map.insert(name, file_source_map);
        self.files.insert(name, source.to_string());

        Cow::from(post_processed_source)
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

fn attr_dialect<'b>(dialect_project: &str, source: Cow<'b, str>) -> Cow<'b, str> {
    fn pr(dialect_project: &str, source: String, start: usize) -> String {
        let rg = Regex::new(r"\#!\[dialect\((?P<name>\w+)\)\]\s").unwrap();
        let find = match rg.find_at(&source, start) {
            Some(find) => find,
            None => return source,
        };
        if let Some(start_comment) = source[..find.start()].rfind("/*") {
            if !source[start_comment..find.end()].contains("*/") {
                return pr(dialect_project, source.to_owned(), find.end());
            }
        }
        if let Some(row) = source[..find.start()].rfind('\n') {
            if source[row..find.end()].contains("//") {
                return pr(dialect_project, source.to_owned(), find.end());
            }
        }

        if rg.replace(find.as_str(), "$name").as_ref() != dialect_project {
            return "".to_string();
        }

        let source = source[..find.start()].to_owned() + &source[find.end() - 1..];
        pr(dialect_project, source, find.start())
    }

    if source.contains("#![dialect(") {
        let nsource = pr(dialect_project, source.to_string(), 0);
        Cow::from(nsource)
    } else {
        source
    }
}
#[cfg(test)]
mod test {
    use move_core_types::language_storage::CORE_CODE_ADDRESS;

    use crate::compiler::mut_string::MutString;
    use crate::compiler::preprocessor::{replace_sender_placeholder, attr_dialect};
    use crate::compiler::source_map::FileOffsetMap;
    use std::borrow::Cow;

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

    #[test]
    pub fn test_attr_dialect() {
        let mut source;

        source = r"
                        #![dialect(pont)]
                        true
                    ";
        assert_eq!(attr_dialect("pont", Cow::from(source)).trim(), "true");
        assert_eq!(attr_dialect("diem", Cow::from(source)).trim(), "");

        source = r"
                        #![dialect(pont)]
                        #![dialect(diem)]
                        true
                    ";
        assert_eq!(attr_dialect("pont", Cow::from(source)).trim(), "");
        assert_eq!(attr_dialect("diem", Cow::from(source)).trim(), "");

        source = r"
                        // #![dialect(dfinance)]
                        /**/#![dialect(pont)]
                        // #![dialect(diem)]
                        /*
                        #![dialect(diem)]
                        */
                        true
                    ";
        assert_eq!(
            attr_dialect("pont", Cow::from(source)).trim(),
            "// #![dialect(dfinance)]\n                        \
            /**/\n                        \
            // #![dialect(diem)]\n                        \
            /*\n                        \
            #![dialect(diem)]\n                        \
            */\n                        \
            true"
        );
        assert_eq!(attr_dialect("diem", Cow::from(source)).trim(), "");

        source = r"
                /*
                 * test
                
                #![dialect(diem)]
                */
                module 0x1::T2{
                    public fun get():u8{
                        1
                    }
                }";
        assert_eq!(
            attr_dialect("pont", Cow::from(source)).trim(),
            "/*\n                 \
            * test\n                \
            \n                \
            #![dialect(diem)]\n                \
            */\n                \
            module 0x1::T2{\n                    \
            public fun get():u8{\n                        \
            1\n                    \
            }\n                \
            }"
        );
    }
}
