use rowan::{TextRange, TextSize};

use crate::rowan_parser::lexer::Token;
use crate::rowan_parser::syntax_kind::SyntaxKind;

pub fn syntax_kind_at(pos: usize, tokens: &[Token]) -> SyntaxKind {
    tokens.get(pos).map(|t| t.kind).unwrap_or(SyntaxKind::EOF)
}

pub struct Tokens<'i> {
    text: &'i str,
    start_offsets: Vec<TextSize>,
    tokens: Vec<Token>,
    // current token kind and current position
    curr: (SyntaxKind, usize),
}

impl<'t> Tokens<'t> {
    pub fn new(text: &'t str, raw_tokens: &'t [Token]) -> Tokens<'t> {
        let mut tokens = vec![];
        let mut start_offsets = vec![];
        let mut last_token_offset = TextSize::default();
        for &token in raw_tokens.iter() {
            if !token.kind.is_trivia() {
                tokens.push(token);
                start_offsets.push(last_token_offset);
            }
            last_token_offset += token.len;
        }
        let first_kind = syntax_kind_at(0, &tokens);
        Tokens {
            text,
            start_offsets,
            tokens,
            curr: (first_kind, 0),
        }
    }

    pub fn current_kind(&self) -> SyntaxKind {
        self.curr.0
    }

    pub fn current_pos(&self) -> usize {
        self.curr.1
    }

    pub fn current_start_loc(&self) -> TextSize {
        self.start_offsets[self.current_pos()]
    }

    pub fn current_text(&self) -> &str {
        let text_range = TextRange::new(self.current_start_loc(), self.next_start_loc());
        &self.text[text_range]
    }

    pub fn lookahead_token(&self) -> Token {
        if self.current_pos() + 1 < self.tokens.len() {
            self.tokens[self.current_pos() + 1]
        } else {
            Token::eof()
        }
    }

    pub fn lookahead_kind(&self) -> SyntaxKind {
        self.lookahead_token().kind
    }

    pub fn next_start_loc(&self) -> TextSize {
        self.current_start_loc() + self.tokens[self.current_pos()].len
    }

    pub fn bump(&mut self) {
        if self.curr.0 == SyntaxKind::EOF {
            return;
        }
        let pos = self.curr.1 + 1;
        self.curr = (syntax_kind_at(pos, &self.tokens), pos);
    }
}
