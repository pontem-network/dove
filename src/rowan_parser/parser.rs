use rowan::{GreenNode, GreenNodeBuilder};

use crate::rowan_parser::lexer;
use crate::rowan_parser::syntax_kind::SyntaxKind;
use crate::rowan_parser::tokens::Tokens;
use crate::rowan_parser::tree::SyntaxNode;

pub struct Parse {
    pub green: GreenNode,
    pub errors: Vec<String>,
}

#[allow(dead_code)]
impl Parse {
    fn to_syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

pub fn parse(text: &str) -> Parse {
    let raw_tokens: Vec<_> = lexer::tokenize(text).collect();
    let token_source = Tokens::new(text, &raw_tokens);
    Parser::new(token_source).parse()
}

fn get_precedence(kind: SyntaxKind) -> u32 {
    match kind {
        // Reserved minimum precedence value is 1
        SyntaxKind::EqualEqualGreater => 2,
        SyntaxKind::PIPE_PIPE => 3,
        SyntaxKind::AMP_AMP => 4,
        SyntaxKind::EqualEqual => 5,
        SyntaxKind::BANG_EQUAL => 5,
        SyntaxKind::Less => 5,
        SyntaxKind::Greater => 5,
        SyntaxKind::LessEqual => 5,
        SyntaxKind::GreaterEqual => 5,
        SyntaxKind::DOT_DOT => 6,
        SyntaxKind::PIPE => 7,
        SyntaxKind::Caret => 8,
        SyntaxKind::AMP => 9,
        SyntaxKind::LessLess => 10,
        SyntaxKind::GreaterGreater => 10,
        SyntaxKind::PLUS => 11,
        SyntaxKind::MINUS => 11,
        SyntaxKind::STAR => 12,
        SyntaxKind::SLASH => 12,
        SyntaxKind::PERCENT => 12,
        _ => 0, // anything else is not a binary operator
    }
}

pub struct Parser<'i> {
    tokens: Tokens<'i>,
    builder: GreenNodeBuilder<'i>,
    errors: Vec<String>,
}

impl<'i> Parser<'i> {
    fn new(tokens: Tokens) -> Parser {
        Parser {
            tokens,
            builder: GreenNodeBuilder::new(),
            errors: vec![],
        }
    }

    fn expect_token(&mut self, kind: SyntaxKind) {
        if self.tokens.current() == kind {
            self.builder
                .token(kind.into(), self.tokens.current_text().into());
            self.tokens.bump();
            if self.tokens.current() == SyntaxKind::WHITESPACE {
                self.builder.token(
                    SyntaxKind::WHITESPACE.into(),
                    self.tokens.current_text().into(),
                );
                self.tokens.bump();
            }
        } else {
            self.errors.push(format!("Invalid token {:?}", kind));
        }
    }

    // fn parse_token(&mut self, token_kind: SyntaxKind) -> Result<(), String> {
    //     if self.tokens.current() == token_kind {
    //         self.builder
    //             .token(token_kind.into(), self.tokens.current_text().into());
    //         self.tokens.bump();
    //         Ok(())
    //     } else {
    //         Err(format!("Invalid token {:?}", token_kind))
    //     }
    // }

    // fn parse_module_ident(&mut self) {
    //     self.builder.start_node(SyntaxKind::MODULE_IDENT.into());
    //     self.token(SyntaxKind::ADDRESS);
    //     self.token(SyntaxKind::COLON_COLON);
    //     self.token(SyntaxKind::NAME);
    //     self.builder.finish_node();
    // }

    // fn parse_use(&mut self) {
    //     self.builder.start_node(SyntaxKind::Use.into());
    //     self.parse_token(SyntaxKind::Use_Kw);
    //     self.parse_module_ident();
    //     self.parse_token(SyntaxKind::SEMICOLON);
    //     self.builder.finish_node();
    // }

    fn parse_unary_expr(&mut self) {
        let kind = self.tokens.current();
        match self.tokens.current() {
            SyntaxKind::BANG
            | SyntaxKind::AMP
            | SyntaxKind::AMP_MUT
            | SyntaxKind::STAR
            | SyntaxKind::MOVE_KW
            | SyntaxKind::COPY_KW => {
                self.builder.start_node(kind.into());
                self.expect_token(kind);
                self.parse_expr();
                self.builder.finish_node()
            }
            _ => self.expect_token(SyntaxKind::NUM),
        }
    }

    fn parse_expr(&mut self) {
        self.parse_unary_expr();
        let _ = get_precedence(self.tokens.current());
    }

    fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::SOURCE_FILE.into());
        self.parse_unary_expr();
        // loop {
        //     let token = self.tokens.current();
        //     match token {
        //         SyntaxKind::EOF => {
        //             break;
        //         }
        //         // SyntaxKind::Use_Kw => {
        //         //     self.parse_use();
        //         // }
        //         _ => unreachable!("unknown token {:?}", token),
        //     }
        // }
        // while self.tokens.current() != SyntaxKind::EOF {
        //     // if self.tokens.current() == SyntaxKind::Use {
        //     //     self.parse_use();
        //     // }
        // }
        self.builder.finish_node();
        let green = self.builder.finish();
        Parse {
            green,
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_expression() {
        let tree = parse("move !&*1").to_syntax_node();
        dbg!(tree);
    }
}
