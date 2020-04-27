use rowan::TextSize;

use crate::rowan_parser::cursor::Cursor;
use crate::rowan_parser::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextSize,
}

impl Token {
    pub fn new(kind: SyntaxKind, len: TextSize) -> Token {
        Token { kind, len }
    }

    pub fn eof() -> Token {
        Token::new(SyntaxKind::EOF, TextSize::default())
    }
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    match c {
        // Usual ASCII suspects
        | '\u{0009}' // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space
            => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct ParseError {
    loc: (usize, usize),
    text: String,
}

impl ParseError {
    pub fn new(loc: (usize, usize), text: String) -> ParseError {
        ParseError { loc, text }
    }
}

impl Cursor<'_> {
    fn advance(&mut self) -> Result<(SyntaxKind, usize), ParseError> {
        let first_char = self.bump().unwrap();
        let syntax_kind = match first_char {
            c if is_whitespace(c) => self.consume_whitespace(),
            '0'..='9' => {
                if self.consume_if_next("x") && self.initial_len > 2 {
                    self.consume_hex_digits();
                    if self.len_consumed() == 2 {
                        SyntaxKind::NUM
                    } else {
                        SyntaxKind::ADDRESS
                    }
                } else {
                    self.consume_decimal_number()
                }
            }
            'x' => {
                if self.consume_if_next("\"") {
                    let start = self.initial_len - 2;
                    // Search the current source line for a closing quote.
                    self.consume_while(|c| c != '"');
                    if self.is_eof() {
                        return Err(ParseError::new(
                            (start, self.len_consumed()),
                            "Unterminated bytestring".to_string(),
                        ));
                    } else {
                        SyntaxKind::BYTESTRING
                    }
                } else {
                    self.consume_ident()
                }
            }
            'A'..='Z' | 'a'..='v' | 'y' | 'z' | '_' => self.consume_ident(),
            '&' => {
                if self.consume_if_next("mut ") {
                    SyntaxKind::AMP_MUT
                } else if self.consume_if_next("&") {
                    SyntaxKind::AMP_AMP
                } else {
                    SyntaxKind::AMP
                }
            }
            '|' => {
                if self.consume_if_next("|") {
                    SyntaxKind::PIPE_PIPE
                } else {
                    SyntaxKind::PIPE
                }
            }
            '=' => {
                if self.consume_if_next("=>") {
                    SyntaxKind::EqualEqualGreater
                } else if self.consume_if_next("=") {
                    SyntaxKind::EqualEqual
                } else {
                    SyntaxKind::Equal
                }
            }
            '!' => {
                if self.consume_if_next("=") {
                    SyntaxKind::BANG_EQUAL
                } else {
                    SyntaxKind::BANG
                }
            }
            '<' => {
                if self.consume_if_next("=") {
                    SyntaxKind::LessEqual
                } else if self.consume_if_next("<") {
                    SyntaxKind::LessLess
                } else {
                    SyntaxKind::Less
                }
            }
            '>' => {
                if self.consume_if_next("=") {
                    SyntaxKind::GreaterEqual
                } else if self.consume_if_next(">") {
                    SyntaxKind::GreaterGreater
                } else {
                    SyntaxKind::Greater
                }
            }
            ':' => {
                if self.consume_if_next(":") {
                    SyntaxKind::COLON_COLON
                } else {
                    SyntaxKind::COLON
                }
            }
            '%' => SyntaxKind::PERCENT,
            '(' => SyntaxKind::L_PAREN,
            ')' => SyntaxKind::R_PAREN,
            '[' => SyntaxKind::L_BRACK,
            ']' => SyntaxKind::R_BRACK,
            '*' => SyntaxKind::STAR,
            '+' => SyntaxKind::PLUS,
            ',' => SyntaxKind::COMMA,
            '-' => SyntaxKind::MINUS,
            '.' => {
                if self.consume_if_next(".") {
                    SyntaxKind::DOT_DOT
                } else {
                    SyntaxKind::DOT
                }
            }
            '/' => SyntaxKind::SLASH,
            ';' => SyntaxKind::SEMICOLON,
            '^' => SyntaxKind::Caret,
            '{' => SyntaxKind::L_CURLY,
            '}' => SyntaxKind::R_CURLY,
            _ => {
                return Err(ParseError::new(
                    (self.initial_len, self.initial_len),
                    "Invalid character".to_string(),
                ));
            }
        };
        Ok((syntax_kind, self.len_consumed()))
    }

    fn consume_whitespace(&mut self) -> SyntaxKind {
        self.consume_while(is_whitespace);
        SyntaxKind::WHITESPACE
    }

    // Return the length of the substring containing characters in [0-9a-fA-F].
    fn consume_hex_digits(&mut self) {
        let is_hex_digit = |c| matches!(c, 'a'..='f' | 'A'..='F' | '0'..='9');
        self.consume_while(is_hex_digit);
    }

    fn consume_decimal_number(&mut self) -> SyntaxKind {
        self.consume_while(|c| matches!(c, '0'..='9'));
        if self.is_next("u8") {
            self.bump_n_times(2);
            SyntaxKind::NUM_U8
        } else if self.is_next("u64") {
            self.bump_n_times(3);
            SyntaxKind::NUM_U64
        } else if self.is_next("u128") {
            self.bump_n_times(4);
            SyntaxKind::NUM_U128
        } else {
            SyntaxKind::NUM
        }
    }

    fn consume_ident(&mut self) -> SyntaxKind {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
        SyntaxKind::IDENT
    }
}

fn get_name_token_kind(name: &str) -> SyntaxKind {
    match name {
        "abort" => SyntaxKind::Abort_Kw,
        "acquires" => SyntaxKind::Acquires_Kw,
        "as" => SyntaxKind::As_Kw,
        "break" => SyntaxKind::Break_Kw,
        "continue" => SyntaxKind::Continue_Kw,
        "copy" => SyntaxKind::COPY_KW,
        "copyable" => SyntaxKind::Copyable_Kw,
        "define" => SyntaxKind::Define_Kw,
        "else" => SyntaxKind::Else_Kw,
        "false" => SyntaxKind::FALSE,
        "fun" => SyntaxKind::Fun_Kw,
        "if" => SyntaxKind::If_Kw,
        "invariant" => SyntaxKind::Invariant_Kw,
        "let" => SyntaxKind::Let_Kw,
        "loop" => SyntaxKind::Loop_Kw,
        "module" => SyntaxKind::Module_Kw,
        "move" => SyntaxKind::MOVE_KW,
        "native" => SyntaxKind::Native_Kw,
        "public" => SyntaxKind::Public_Kw,
        "resource" => SyntaxKind::Resource_Kw,
        "return" => SyntaxKind::Return_Kw,
        "spec" => SyntaxKind::Spec_Kw,
        "struct" => SyntaxKind::Struct_Kw,
        "true" => SyntaxKind::TRUE,
        "use" => SyntaxKind::Use_Kw,
        "while" => SyntaxKind::While_Kw,
        _ => SyntaxKind::NAME,
    }
}

/// Parses the first token from the provided input string.
fn first_token(input: &str) -> Result<(SyntaxKind, usize), ParseError> {
    let (mut kind, len) = Cursor::new(input).advance()?;
    if kind == SyntaxKind::IDENT {
        kind = get_name_token_kind(&input[..len]);
    }
    Ok((kind, len))
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = vec![];
    while !input.is_empty() {
        let (kind, len) = first_token(input)?;
        input = &input[len..];
        tokens.push(Token::new(kind, (len as u32).into()));
    }
    Ok(tokens)
}
