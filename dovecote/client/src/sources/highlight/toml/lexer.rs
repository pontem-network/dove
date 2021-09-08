use std::fmt;
use crate::sources::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tok {
    EOF,
    NumValue,
    Identifier,
    String,
    LBracket,
    RBracket,
    Comma,
    Equal,
    False,
    True,
    LBrace,
    RBrace,
    NumSign,
    Period,
}

impl fmt::Display for Tok {
    fn fmt<'f>(&self, formatter: &mut fmt::Formatter<'f>) -> Result<(), fmt::Error> {
        use Tok::*;
        let s = match *self {
            EOF => "[end-of-file]",
            NumValue => "[Num]",
            Identifier => "[Identifier]",
            String => "[String]",
            LBracket => "[",
            RBracket => "]",
            Comma => ",",
            Equal => "=",
            False => "false",
            True => "true",
            LBrace => "{",
            RBrace => "}",
            Period => ".",
            NumSign => "#",
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

    pub fn advance(&mut self) -> Result<(), Error> {
        self.prev_end = self.cur_end;
        let text = self.text[self.cur_end..].trim_start();
        self.cur_start = self.text.len() - text.len();
        let (token, len) = find_token(text, self.cur_start)?;
        self.cur_end = self.cur_start + len;
        self.token = token;
        Ok(())
    }
}

fn find_token(text: &str, start_offset: usize) -> Result<(Tok, usize), Error> {
    let c: char = match text.chars().next() {
        Some(next_char) => next_char,
        None => {
            return Ok((Tok::EOF, 0));
        }
    };
    let (tok, len) = match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-' => {
            let ident = get_ident(&text);
            let val = &text[0..ident];
            if val == "true" {
                (Tok::True, ident)
            } else if val == "false" {
                (Tok::False, ident)
            } else if is_number(val) {
                (Tok::NumValue, ident)
            } else {
                (Tok::Identifier, ident)
            }
        }
        '\"' => (Tok::String, get_string_len(&text)),
        '=' => (Tok::Equal, 1),
        '[' => (Tok::LBracket, 1),
        ']' => (Tok::RBracket, 1),
        ',' => (Tok::Comma, 1),
        '{' => (Tok::LBrace, 1),
        '}' => (Tok::RBrace, 1),
        '.' => (Tok::Period, 1),
        '#' => (Tok::NumSign, 1),
        _ => {
            let loc = make_loc(start_offset, start_offset);
            return Err(vec![(loc, format!("Invalid character: '{}'", c))]);
        }
    };

    Ok((tok, len))
}

fn get_ident(text: &str) -> usize {
    text.chars()
        .position(|c| !matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-'))
        .unwrap_or_else(|| text.len())
}

fn get_string_len(text: &str) -> usize {
    let mut pos = 1;
    let mut iter = text[1..].chars();
    while let Some(chr) = iter.next() {
        pos += 1;
        if chr == '"' {
            break;
        }
    }
    pos
}

fn is_number(text: &str) -> bool {
    text.parse::<i64>().is_ok()
}
