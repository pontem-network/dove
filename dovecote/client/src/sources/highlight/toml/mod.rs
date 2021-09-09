mod lexer;

use crate::sources::highlight::toml::lexer::{Lexer, Tok};
use crate::sources::highlight::{Line, Marker, StyleType};
use crate::sources::Error;

#[derive(Default)]
pub struct Toml {}

impl Marker for Toml {
    fn mark_line<'input>(&mut self, line: &'input str) -> Result<Line<'input>, Error> {
        let mut items = vec![];

        let mut lexer = Lexer::new(line);
        loop {
            lexer.advance()?;
            let element = &line[lexer.start_loc()..lexer.start_loc() + lexer.content().len()];

            if lexer.start_loc() - lexer.previous_end_loc() > 0 {
                items.push((
                    StyleType::Space,
                    &line[lexer.previous_end_loc()..lexer.start_loc()],
                ));
            }

            match lexer.peek() {
                Tok::EOF => {
                    break;
                }
                Tok::Identifier => {
                    items.push((StyleType::Var, element));
                }
                Tok::String => {
                    items.push((StyleType::String, element));
                }
                Tok::NumSign => {
                    items.push((StyleType::Comment, &line[lexer.start_loc()..]));
                    break;
                }
                Tok::NumValue => {
                    items.push((StyleType::Number, element));
                }
                Tok::True | Tok::False => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Period
                | Tok::LBracket
                | Tok::RBracket
                | Tok::Comma
                | Tok::Equal
                | Tok::LBrace
                | Tok::RBrace => {
                    items.push((StyleType::Normal, element));
                }
            }
        }

        Ok(Line { items })
    }
}

#[cfg(test)]
mod tests {
    use crate::sources::highlight::{Line, mark_code};
    use crate::sources::highlight::toml::Toml;
    use crate::sources::highlight::StyleType::*;

    #[test]
    pub fn test_toml_highlight() {
        let source = r#"
[package]
name = "dove"
version = "1.3.2"
authors = [
    "Alex Koz. <alexanderkozlovskii@wings.ai>",
    "Maxim Kurnikov <maximkurnikov@wings.ai>"
]
edition = "2018"
build = "build.rs"
exclude = [
    "dovecote/client",
]
int = -42
11 = ff
bool = false
bool_1 = true
[dependencies]
fs_extra = "1.2.0"
tiny-keccak = { version = "2.0.2", default-features = false, features = ["sha3"] }
lang = { path = "../lang" }
move-cli = { git = "https://github.com/pontem-network/diem.git", branch = "v1.3-r1" }
# move-prover deps
[features]
default = []
"#;

        let marked_code = mark_code::<Toml>(&"D.toml".to_string(), &source);
        assert_eq!(
            marked_code,
            vec![
                Line { items: vec![] },
                Line {
                    items: vec![(Normal, "["), (Var, "package"), (Normal, "]")]
                },
                Line {
                    items: vec![
                        (Var, "name"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"dove\"")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "version"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"1.3.2\"")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "authors"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "[")
                    ]
                },
                Line {
                    items: vec![
                        (Space, "    "),
                        (String, "\"Alex Koz. <alexanderkozlovskii@wings.ai>\""),
                        (Normal, ",")
                    ]
                },
                Line {
                    items: vec![
                        (Space, "    "),
                        (String, "\"Maxim Kurnikov <maximkurnikov@wings.ai>\"")
                    ]
                },
                Line {
                    items: vec![(Normal, "]")]
                },
                Line {
                    items: vec![
                        (Var, "edition"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"2018\"")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "build"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"build.rs\"")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "exclude"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "[")
                    ]
                },
                Line {
                    items: vec![
                        (Space, "    "),
                        (String, "\"dovecote/client\""),
                        (Normal, ",")
                    ]
                },
                Line {
                    items: vec![(Normal, "]")]
                },
                Line {
                    items: vec![
                        (Var, "int"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Number, "-42")
                    ]
                },
                Line {
                    items: vec![
                        (Number, "11"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Var, "ff")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "bool"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (KeyWords, "false")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "bool_1"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (KeyWords, "true")
                    ]
                },
                Line {
                    items: vec![(Normal, "["), (Var, "dependencies"), (Normal, "]")]
                },
                Line {
                    items: vec![
                        (Var, "fs_extra"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"1.2.0\"")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "tiny-keccak"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "{"),
                        (Space, " "),
                        (Var, "version"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"2.0.2\""),
                        (Normal, ","),
                        (Space, " "),
                        (Var, "default-features"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (KeyWords, "false"),
                        (Normal, ","),
                        (Space, " "),
                        (Var, "features"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "["),
                        (String, "\"sha3\""),
                        (Normal, "]"),
                        (Space, " "),
                        (Normal, "}")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "lang"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "{"),
                        (Space, " "),
                        (Var, "path"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"../lang\""),
                        (Space, " "),
                        (Normal, "}")
                    ]
                },
                Line {
                    items: vec![
                        (Var, "move-cli"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "{"),
                        (Space, " "),
                        (Var, "git"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"https://github.com/pontem-network/diem.git\""),
                        (Normal, ","),
                        (Space, " "),
                        (Var, "branch"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (String, "\"v1.3-r1\""),
                        (Space, " "),
                        (Normal, "}")
                    ]
                },
                Line {
                    items: vec![(Comment, "# move-prover deps")]
                },
                Line {
                    items: vec![(Normal, "["), (Var, "features"), (Normal, "]")]
                },
                Line {
                    items: vec![
                        (Var, "default"),
                        (Space, " "),
                        (Normal, "="),
                        (Space, " "),
                        (Normal, "["),
                        (Normal, "]")
                    ]
                }
            ]
        );
    }
}
