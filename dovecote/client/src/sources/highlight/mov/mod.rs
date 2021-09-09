use crate::sources::highlight::{Marker, Line, StyleType};
use crate::sources::Error;
use crate::sources::highlight::mov::lexer::{Lexer, Tok};
use crate::sources::highlight::mov::parser::parse_num_value;

pub mod lexer;
pub mod parser;

#[derive(Default)]
pub struct Move {
    ctx: Context,
}

impl Marker for Move {
    fn reset(&mut self) {
        self.ctx = Context::None;
    }

    fn mark_line<'input>(&mut self, line: &'input str) -> Result<Line<'input>, Error> {
        let mut items = vec![];

        let mut lexer = Lexer::new(line);
        loop {
            lexer.advance()?;
            let element = &line[lexer.start_loc()..lexer.start_loc() + lexer.content().len()];

            if lexer.start_loc() - lexer.previous_end_loc() > 0 {
                items.push((StyleType::Space, &line[lexer.previous_end_loc()..lexer.start_loc()]));
            }
            match lexer.peek() {
                Tok::EOF => {
                    break;
                }
                Tok::NumValue => {
                    match self.ctx {
                        Context::Address => {
                            self.ctx = Context::None;
                            items.push((StyleType::Normal, parse_num_value(&mut lexer, line)?));
                        }
                        _ => {}
                    }
                }
                Tok::NumTypedValue => {}
                Tok::ByteStringValue => {}
                Tok::IdentifierValue => {
                    match self.ctx {
                        Context::Address => {
                            self.ctx = Context::None;
                            items.push((StyleType::Normal, element));
                        }
                        _ => {
                            match element {
                                "address" => {
                                    self.ctx = Context::Address;
                                    items.push((StyleType::KeyWords, element))
                                }

                                &_ => {
                                    items.push((StyleType::Normal, element));
                                }
                            }
                        }
                    }
                }
                Tok::Exclaim => {}
                Tok::ExclaimEqual => {}
                Tok::Percent => {}
                Tok::Amp => {}
                Tok::AmpAmp => {}
                Tok::AmpMut => {}
                Tok::LParen => {}
                Tok::RParen => {}
                Tok::LBracket => {}
                Tok::RBracket => {}
                Tok::Star => {}
                Tok::Plus => {}
                Tok::Comma => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Minus => {}
                Tok::Period => {}
                Tok::PeriodPeriod => {}
                Tok::Slash => {}
                Tok::Colon => {}
                Tok::ColonColon => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Semicolon => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Less => {}
                Tok::LessEqual => {}
                Tok::LessLess => {}
                Tok::Equal => {}
                Tok::EqualEqual => {}
                Tok::EqualEqualGreater => {}
                Tok::Greater => {}
                Tok::GreaterEqual => {}
                Tok::GreaterGreater => {}
                Tok::Caret => {}
                Tok::Abort => {}
                Tok::Acquires => {}
                Tok::As => {}
                Tok::Break => {}
                Tok::Continue => {}
                Tok::Copy => {}
                Tok::Else => {}
                Tok::False => {}
                Tok::If => {}
                Tok::Invariant => {}
                Tok::Let => {}
                Tok::Loop => {}
                Tok::Module => {
                    let (first, second, third) = lexer.lookahead3()?;
                    if first == Tok::ColonColon || second == Tok::ColonColon || third == Tok::ColonColon {
                        self.ctx = Context::Address;
                    }

                    items.push((StyleType::KeyWords, element));
                }
                Tok::Move => {}
                Tok::Native => {}
                Tok::Public => {}
                Tok::Return => {}
                Tok::Spec => {}
                Tok::Struct => {}
                Tok::True => {}
                Tok::Use => {
                    self.ctx = Context::Address;
                    items.push((StyleType::KeyWords, element));
                }
                Tok::While => {}
                Tok::LBrace => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Pipe => {}
                Tok::PipePipe => {}
                Tok::RBrace => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Fun => {}
                Tok::Script => {}
                Tok::Const => {}
                Tok::Friend => {}
                Tok::NumSign => {}
                Tok::AtSign => {}
            }
        }

        Ok(Line {
            items
        })
    }
}

enum Context {
    None,
    Address,
}

impl Default for Context {
    fn default() -> Self {
        Context::None
    }
}

#[cfg(test)]
mod tests {
    use crate::sources::highlight::{Line, mark_code};
    use crate::sources::highlight::mov::Move;
    use crate::sources::highlight::StyleType::*;

    #[test]
    pub fn test_move_highlight() {
        let source = r#"
address 0x1 {
    use 0x1::Diem::{Self, Diem, Preburn};
    use wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh::Roles;
    use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Roles;
}

module 0x1::T {
}

module T {}
        "#;

        let marked_code = mark_code::<Move>(&"D.move".to_string(), &source);
        assert_eq!(marked_code, vec![
            Line { items: vec![] },
            Line { items: vec![(KeyWords, "address"), (Space, " "), (Normal, "0x1"), (Space, " "), (Normal, "{")] },
            Line { items: vec![(Space, "    "), (KeyWords, "use"), (Space, " "), (Normal, "0x1"), (Normal, "::"), (Normal, "Diem"), (Normal, "::"), (Normal, "{"), (Normal, "Self"), (Normal, ","), (Space, " "), (Normal, "Diem"), (Normal, ","), (Space, " "), (Normal, "Preburn"), (Normal, "}"), (KeyWords, ";")] },
            Line { items: vec![(Space, "    "), (KeyWords, "use"), (Space, " "), (Normal, "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"), (Normal, "::"), (Normal, "Roles"), (KeyWords, ";")] },
            Line { items: vec![(Space, "    "), (KeyWords, "use"), (Space, " "), (Normal, "1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE"), (Normal, "::"), (Normal, "Roles"), (KeyWords, ";")] },
            Line { items: vec![(Normal, "}")] },
            Line { items: vec![] },
            Line { items: vec![(KeyWords, "module"), (Space, " "), (Normal, "0x1"), (Normal, "::"), (Normal, "T"), (Space, " "), (Normal, "{")] },
            Line { items: vec![(Normal, "}")] },
            Line { items: vec![] },
            Line { items: vec![(KeyWords, "module"), (Space, " "), (Normal, "T"), (Space, " "), (Normal, "{"), (Normal, "}")] },
            Line { items: vec![(Space, "        ")] }

        ]);
    }
}
