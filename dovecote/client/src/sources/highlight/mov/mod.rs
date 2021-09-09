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
                        _ => {
                            items.push((StyleType::Number, parse_num_value(&mut lexer, line)?));
                        }
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
                Tok::Exclaim => {
                    items.push((StyleType::Normal, element));
                }
                Tok::ExclaimEqual => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Percent => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Amp => {
                    items.push((StyleType::Normal, element));
                }
                Tok::AmpAmp => {
                    items.push((StyleType::Normal, element));
                }
                Tok::AmpMut => {
                    //& mut
                }
                Tok::LParen => {}
                Tok::RParen => {}
                Tok::LBracket => {}
                Tok::RBracket => {}
                Tok::Star => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Plus => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Comma => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Minus => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Period => {
                    items.push((StyleType::Normal, element));
                }
                Tok::PeriodPeriod => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Slash => {
                    //
                }
                Tok::Colon => {
                    items.push((StyleType::Normal, element));
                }
                Tok::ColonColon => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Semicolon => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Less => {
                    items.push((StyleType::Normal, element));
                }
                Tok::LessEqual => {
                    items.push((StyleType::Normal, element));
                }
                Tok::LessLess => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Equal => {
                    items.push((StyleType::Normal, element));
                }
                Tok::EqualEqual => { items.push((StyleType::Normal, element));}
                Tok::EqualEqualGreater => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Greater => { items.push((StyleType::Normal, element));}
                Tok::GreaterEqual => {
                    items.push((StyleType::Normal, element));
                }
                Tok::GreaterGreater => { items.push((StyleType::Normal, element));}
                Tok::Caret => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Abort => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Acquires => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::As => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Break => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Continue => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Copy => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Else => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::False => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::If => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Invariant => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Let => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Loop => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Module => {
                    self.ctx = Context::Address;
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Move => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Native => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Public => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Return => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Spec => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Struct => {
                    items.push((StyleType::KeyWords, element));

                }
                Tok::True => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Use => {
                    self.ctx = Context::Address;
                    items.push((StyleType::KeyWords, element));
                }
                Tok::While => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::LBrace => {
                    self.ctx = Context::None;
                    items.push((StyleType::Normal, element));
                }
                Tok::Pipe => {
                    items.push((StyleType::Normal, element));
                }
                Tok::PipePipe => {
                    items.push((StyleType::Normal, element));
                }
                Tok::RBrace => {
                    items.push((StyleType::Normal, element));
                }
                Tok::Fun => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Script => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Const => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::Friend => {
                    items.push((StyleType::KeyWords, element));
                }
                Tok::NumSign => {
                    //#
                }
                Tok::AtSign => {
                    items.push((StyleType::KeyWords, element));
                }
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

module 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::T {
    struct D {

    }
    struct D1<T> {

    }
}
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
            Line { items: vec![(Space, "        ")] },
            Line { items: vec![(KeyWords, "module"), (Space, " "), (Normal, "1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE"), (Normal, "::"), (Normal, "T"), (Space, " "), (Normal, "{")] },
            Line { items: vec![(Space, "    "), (Space, " "), (Normal, "D"), (Space, " "), (Normal, "{")] },
            Line { items: vec![] },
            Line { items: vec![(Space, "    "), (Normal, "}")] },
            Line { items: vec![(Space, "    "), (Space, " "), (Normal, "D1"), (Normal, "T"), (Space, " "), (Normal, "{")] },
            Line { items: vec![] },
            Line { items: vec![(Space, "    "), (Normal, "}")] },
            Line { items: vec![(Normal, "}")] },
            Line { items: vec![(Space, "        ")] }


        ]);
    }
}
