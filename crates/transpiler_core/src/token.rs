#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Function,
    Return,
    Void,
    Const,
    Print,
    If,
    Else,
    For,
    While,
    In,
    Any,
    Break,
    Continue,

    NumberType,
    StringType,
    BooleanType,

    Identifier(String),
    NumberLiteral(i32),
    StringLiteral(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    Colon,
    Semicolon,
    Comma,
    Dot,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AmpersandAmpersand,
    PipePipe,
    Bang,

    Ampersand,
    Mut,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}
