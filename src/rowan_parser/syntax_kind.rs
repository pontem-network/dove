#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    WHITESPACE,
    // literals
    ADDRESS,
    IDENT,
    NUM,
    NUM_U8,
    NUM_U64,
    NUM_U128,
    BYTESTRING,
    BYTESTRING_UNTERMINATED,
    NAME,
    // operators
    BANG,
    BANG_EQUAL,
    AMP,
    AMP_AMP,
    AMP_MUT,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    DOT,
    DOT_DOT,
    COMMA,
    COLON,
    COLON_COLON,
    SEMICOLON,
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
    L_CURLY,
    R_CURLY,
    PIPE,
    PIPE_PIPE,
    // bools
    FALSE,
    TRUE,
    // keywords
    Abort_Kw,
    Acquires_Kw,
    As_Kw,
    Break_Kw,
    Continue_Kw,
    COPY_KW,
    Copyable_Kw,
    Define_Kw,
    Else_Kw,
    If_Kw,
    Invariant_Kw,
    Let_Kw,
    Loop_Kw,
    Module_Kw,
    MOVE_KW,
    Native_Kw,
    Public_Kw,
    Resource_Kw,
    Return_Kw,
    Spec_Kw,
    Struct_Kw,
    Use_Kw,
    While_Kw,
    Fun_Kw,
    // composites
    MODULE_IDENT,
    Name,
    Use,
    FUN_DEF,
    SPEC_DEF,
    MODULE_DEF,
    // globals
    ADDRESS_GLOBAL,
    SOURCE_FILE,
}

impl From<u16> for SyntaxKind {
    fn from(d: u16) -> SyntaxKind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE)
    }
}
