#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Import,
    Struct,
    Enum,
    Fn,
    Let,
    Var,
    Return,
    Switch,
    Case,
    For,
    In,
    Try,
    If,
    Else,
    While,

    // Some Data ypes
    IntType,
    FloatType,
    StringType,
    VoidType,
    BooleanType,

    // Identifiers and literals?
    Identifier(String),
    Number(String),
    StringSlice(String),

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Boolean Results
    True,
    False,

    // Relational and assignement operators
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Other operators
    Bang,
    Arrow, // -> (functions same as in rust)

    // Important punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,

    // Logical Operators
    And,
    Or,
    Not,

    // Special things
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
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
