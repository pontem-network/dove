// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use crate::sources::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tok {
    EOF,
    NumValue,
    NumTypedValue,
    ByteStringValue,
    IdentifierValue,
    Exclaim,
    ExclaimEqual,
    Percent,
    Amp,
    AmpAmp,
    AmpMut,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Star,
    Plus,
    Comma,
    Minus,
    Period,
    PeriodPeriod,
    Slash,
    Colon,
    ColonColon,
    Semicolon,
    Less,
    LessEqual,
    LessLess,
    Equal,
    EqualEqual,
    EqualEqualGreater,
    Greater,
    GreaterEqual,
    GreaterGreater,
    Caret,
    Abort,
    Acquires,
    As,
    Break,
    Continue,
    Copy,
    Else,
    False,
    If,
    Invariant,
    Let,
    Loop,
    Module,
    Move,
    Native,
    Public,
    Return,
    Spec,
    Struct,
    True,
    Use,
    While,
    LBrace,
    Pipe,
    PipePipe,
    RBrace,
    Fun,
    Script,
    Const,
    Friend,
    NumSign,
    AtSign,
}

impl fmt::Display for Tok {
    fn fmt<'f>(&self, formatter: &mut fmt::Formatter<'f>) -> Result<(), fmt::Error> {
        use Tok::*;
        let s = match *self {
            EOF => "[end-of-file]",
            NumValue => "[Num]",
            NumTypedValue => "[NumTyped]",
            ByteStringValue => "[ByteString]",
            IdentifierValue => "[Identifier]",
            Exclaim => "!",
            ExclaimEqual => "!=",
            Percent => "%",
            Amp => "&",
            AmpAmp => "&&",
            AmpMut => "&mut",
            LParen => "(",
            RParen => ")",
            LBracket => "[",
            RBracket => "]",
            Star => "*",
            Plus => "+",
            Comma => ",",
            Minus => "-",
            Period => ".",
            PeriodPeriod => "..",
            Slash => "/",
            Colon => ":",
            ColonColon => "::",
            Semicolon => ";",
            Less => "<",
            LessEqual => "<=",
            LessLess => "<<",
            Equal => "=",
            EqualEqual => "==",
            EqualEqualGreater => "==>",
            Greater => ">",
            GreaterEqual => ">=",
            GreaterGreater => ">>",
            Caret => "^",
            Abort => "abort",
            Acquires => "acquires",
            As => "as",
            Break => "break",
            Continue => "continue",
            Copy => "copy",
            Else => "else",
            False => "false",
            If => "if",
            Invariant => "invariant",
            Let => "let",
            Loop => "loop",
            Module => "module",
            Move => "move",
            Native => "native",
            Public => "public",
            Return => "return",
            Spec => "spec",
            Struct => "struct",
            True => "true",
            Use => "use",
            While => "while",
            LBrace => "{",
            Pipe => "|",
            PipePipe => "||",
            RBrace => "}",
            Fun => "fun",
            Script => "script",
            Const => "const",
            Friend => "friend",
            NumSign => "#",
            AtSign => "@",
        };
        fmt::Display::fmt(s, formatter)
    }
}

pub struct Lexer<'input> {
    text: &'input str,
    prev_end: usize,
    cur_start: usize,
    cur_end: usize,
    token: Tok,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str) -> Lexer<'input> {
        Lexer {
            text,
            prev_end: 0,
            cur_start: 0,
            cur_end: 0,
            token: Tok::EOF,
        }
    }

    pub fn peek(&self) -> Tok {
        self.token
    }

    pub fn content(&self) -> &str {
        &self.text[self.cur_start..self.cur_end]
    }

    pub fn start_loc(&self) -> usize {
        self.cur_start
    }

    pub fn previous_end_loc(&self) -> usize {
        self.prev_end
    }

    // Look ahead to the next token after the current one and return it without advancing
    // the state of the lexer.
    pub fn lookahead(&self) -> Result<Tok, Error> {
        let text = self.text[self.cur_end..].trim_start();
        let offset = self.text.len() - text.len();
        let (tok, _) = find_token(text, offset)?;
        Ok(tok)
    }

    // Look ahead to the next two tokens after the current one and return them without advancing
    // the state of the lexer.
    pub fn lookahead2(&self) -> Result<(Tok, Tok), Error> {
        let text = self.text[self.cur_end..].trim_start();
        let offset = self.text.len() - text.len();
        let (first, length) = find_token(text, offset)?;
        let text2 = self.text[offset + length..].trim_start();
        let offset2 = self.text.len() - text2.len();
        let (second, _) = find_token(text2, offset2)?;
        Ok((first, second))
    }

    pub fn advance(&mut self) -> Result<(), Error> {
        self.prev_end = self.cur_end;
        let text = self.text[self.cur_end..].trim_start();
        self.cur_start = self.text.len() - text.len();
        let (token, len) = find_token(text, self.cur_start)?;
        self.cur_end = self.cur_start + len;
        self.token = token;
        Ok(())
    }

    // Replace the current token. The lexer will always match the longest token,
    // but sometimes the parser will prefer to replace it with a shorter one,
    // e.g., ">" instead of ">>".
    pub fn replace_token(&mut self, token: Tok, len: usize) {
        self.token = token;
        self.cur_end = self.cur_start + len
    }
}

// Find the next token and its length without changing the state of the lexer.
fn find_token(text: &str, start_offset: usize) -> Result<(Tok, usize), Error> {
    let c: char = match text.chars().next() {
        Some(next_char) => next_char,
        None => {
            return Ok((Tok::EOF, 0));
        }
    };
    let (tok, len) = match c {
        '0'..='9' => {
            if text.starts_with("0x") && text.len() > 2 {
                let (tok, hex_len) = get_hex_number(&text[2..]);
                if hex_len == 0 {
                    // Fall back to treating this as a "0" token.
                    (Tok::NumValue, 1)
                } else {
                    (tok, 2 + hex_len)
                }
            } else {
                get_decimal_number(text)
            }
        }
        'A'..='Z' | 'a'..='z' | '_' => {
            if text.starts_with("x\"") || text.starts_with("b\"") {
                let line = &text.lines().next().unwrap()[2..];
                match get_string_len(line) {
                    Some(last_quote) => (Tok::ByteStringValue, 2 + last_quote + 1),
                    None => {
                        return Err(vec![(
                            make_loc(start_offset, start_offset + line.len() + 2),
                            "Missing closing quote (\") after byte string".to_string(),
                        )])
                    }
                }
            } else {
                let len = get_name_len(&text);
                (get_name_token(&text[..len]), len)
            }
        }
        '&' => {
            if text.starts_with("&mut ") {
                (Tok::AmpMut, 5)
            } else if text.starts_with("&&") {
                (Tok::AmpAmp, 2)
            } else {
                (Tok::Amp, 1)
            }
        }
        '|' => {
            if text.starts_with("||") {
                (Tok::PipePipe, 2)
            } else {
                (Tok::Pipe, 1)
            }
        }
        '=' => {
            if text.starts_with("==>") {
                (Tok::EqualEqualGreater, 3)
            } else if text.starts_with("==") {
                (Tok::EqualEqual, 2)
            } else {
                (Tok::Equal, 1)
            }
        }
        '!' => {
            if text.starts_with("!=") {
                (Tok::ExclaimEqual, 2)
            } else {
                (Tok::Exclaim, 1)
            }
        }
        '<' => {
            if text.starts_with("<=") {
                (Tok::LessEqual, 2)
            } else if text.starts_with("<<") {
                (Tok::LessLess, 2)
            } else {
                (Tok::Less, 1)
            }
        }
        '>' => {
            if text.starts_with(">=") {
                (Tok::GreaterEqual, 2)
            } else if text.starts_with(">>") {
                (Tok::GreaterGreater, 2)
            } else {
                (Tok::Greater, 1)
            }
        }
        ':' => {
            if text.starts_with("::") {
                (Tok::ColonColon, 2)
            } else {
                (Tok::Colon, 1)
            }
        }
        '%' => (Tok::Percent, 1),
        '(' => (Tok::LParen, 1),
        ')' => (Tok::RParen, 1),
        '[' => (Tok::LBracket, 1),
        ']' => (Tok::RBracket, 1),
        '*' => (Tok::Star, 1),
        '+' => (Tok::Plus, 1),
        ',' => (Tok::Comma, 1),
        '-' => (Tok::Minus, 1),
        '.' => {
            if text.starts_with("..") {
                (Tok::PeriodPeriod, 2)
            } else {
                (Tok::Period, 1)
            }
        }
        '/' => (Tok::Slash, 1),
        ';' => (Tok::Semicolon, 1),
        '^' => (Tok::Caret, 1),
        '{' => (Tok::LBrace, 1),
        '}' => (Tok::RBrace, 1),
        '#' => (Tok::NumSign, 1),
        '@' => (Tok::AtSign, 1),
        _ => {
            let loc = make_loc(start_offset, start_offset);
            return Err(vec![(loc, format!("Invalid character: '{}'", c))]);
        }
    };

    Ok((tok, len))
}

// Return the length of the substring matching [a-zA-Z0-9_]. Note that
// this does not do any special check for whether the first character
// starts with a number, so the caller is responsible for any additional
// checks on the first character.
fn get_name_len(text: &str) -> usize {
    text.chars()
        .position(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
        .unwrap_or_else(|| text.len())
}

fn get_decimal_number(text: &str) -> (Tok, usize) {
    let num_text_len = text
        .chars()
        .position(|c| !matches!(c, '0'..='9'))
        .unwrap_or_else(|| text.len());
    get_number_maybe_with_suffix(text, num_text_len)
}

// Return the length of the substring containing characters in [0-9a-fA-F].
fn get_hex_number(text: &str) -> (Tok, usize) {
    let num_text_len = text
        .find(|c| !matches!(c, 'a'..='f' | 'A'..='F' | '0'..='9'))
        .unwrap_or_else(|| text.len());
    get_number_maybe_with_suffix(text, num_text_len)
}

// Given the text for a number literal and the length for the characters that match to the number
// portion, checks for a typed suffix.
fn get_number_maybe_with_suffix(text: &str, num_text_len: usize) -> (Tok, usize) {
    let rest = &text[num_text_len..];
    if rest.starts_with("u8") {
        (Tok::NumTypedValue, num_text_len + 2)
    } else if rest.starts_with("u64") {
        (Tok::NumTypedValue, num_text_len + 3)
    } else if rest.starts_with("u128") {
        (Tok::NumTypedValue, num_text_len + 4)
    } else {
        // No typed suffix
        (Tok::NumValue, num_text_len)
    }
}

// Return the length of the quoted string, or None if there is no closing quote.
fn get_string_len(text: &str) -> Option<usize> {
    let mut pos = 0;
    let mut iter = text.chars();
    while let Some(chr) = iter.next() {
        if chr == '\\' {
            // Skip over the escaped character (e.g., a quote or another backslash)
            if iter.next().is_some() {
                pos += 1;
            }
        } else if chr == '"' {
            return Some(pos);
        }
        pos += 1;
    }
    None
}

fn get_name_token(name: &str) -> Tok {
    match name {
        "abort" => Tok::Abort,
        "acquires" => Tok::Acquires,
        "as" => Tok::As,
        "break" => Tok::Break,
        "const" => Tok::Const,
        "continue" => Tok::Continue,
        "copy" => Tok::Copy,
        "else" => Tok::Else,
        "false" => Tok::False,
        "fun" => Tok::Fun,
        "friend" => Tok::Friend,
        "if" => Tok::If,
        "invariant" => Tok::Invariant,
        "let" => Tok::Let,
        "loop" => Tok::Loop,
        "module" => Tok::Module,
        "move" => Tok::Move,
        "native" => Tok::Native,
        "public" => Tok::Public,
        "return" => Tok::Return,
        "script" => Tok::Script,
        "spec" => Tok::Spec,
        "struct" => Tok::Struct,
        "true" => Tok::True,
        "use" => Tok::Use,
        "while" => Tok::While,
        _ => Tok::IdentifierValue,
    }
}
