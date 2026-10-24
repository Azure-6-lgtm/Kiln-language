use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\r' | '\n')) {
            self.advance();
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.peek().is_some() {
            self.skip_whitespace();

            let Some(ch) = self.peek() else {
                break;
            };

            let line = self.line;
            let column = self.column;

            let kind = match ch {
                '(' => {
                    self.advance();
                    TokenKind::LeftParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RightParen
                }
                '{' => {
                    self.advance();
                    TokenKind::LeftBrace
                }
                '}' => {
                    self.advance();
                    TokenKind::RightBrace
                }
                '[' => {
                    self.advance();
                    TokenKind::LeftBracket
                }
                ']' => {
                    self.advance();
                    TokenKind::RightBracket
                }
                ':' => {
                    self.advance();
                    TokenKind::Colon
                }
                ';' => {
                    self.advance();
                    TokenKind::Semicolon
                }
                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                '.' => {
                    self.advance();
                    TokenKind::Dot
                }
                '+' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::PlusEqual
                    } else {
                        TokenKind::Plus
                    }
                }
                '-' => {
                    self.advance();
                    if self.peek() == Some('>') {
                        self.advance();
                        TokenKind::Arrow
                    } else if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::MinusEqual
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::StarEqual
                    } else {
                        TokenKind::Star
                    }
                }
                '/' => {
                    self.advance();

                    match self.peek() {
                        Some('/') => {
                            self.advance();

                            while let Some(ch) = self.peek() {
                                if ch == '\n' {
                                    break;
                                }
                                self.advance();
                            }

                            continue;
                        }

                        Some('*') => {
                            self.advance();

                            let mut closed = false;

                            while let Some(ch) = self.peek() {
                                if ch == '*' {
                                    self.advance();

                                    if self.peek() == Some('/') {
                                        self.advance();
                                        closed = true;
                                        break;
                                    }
                                } else {
                                    self.advance();
                                }
                            }

                            if !closed {
                                return Err(LexerError {
                                    message: "Multiline comment was not properly closed [E010]"
                                        .to_string(),
                                    line,
                                    column,
                                });
                            }

                            continue;
                        }

                        Some('=') => {
                            self.advance();
                            TokenKind::SlashEqual
                        }

                        _ => TokenKind::Slash,
                    }
                }
                '%' => {
                    self.advance();
                    TokenKind::Percent
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                '"' => {
                    self.advance();
                    let mut value = String::new();

                    while let Some(c) = self.peek() {
                        if c == '"' {
                            self.advance();
                            break;
                        }

                        value.push(self.advance().unwrap());
                    }

                    if !self
                        .source
                        .get(self.position.saturating_sub(1))
                        .is_some_and(|&c| c == '"')
                    {
                        return Err(LexerError {
                            message: "Unterminated string literal [E022]".to_string(),
                            line,
                            column,
                        });
                    }

                    TokenKind::StringSlice(value)
                }
                c if c.is_ascii_digit() => {
                    let mut number = String::new();

                    while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                        number.push(self.advance().unwrap());
                    }

                    if self.peek() == Some('.')
                        && matches!(
                            self.source.get(self.position + 1),
                            Some(c) if c.is_ascii_digit()
                        )
                    {
                        number.push(self.advance().unwrap());

                        while matches!(
                            self.peek(),
                            Some(c) if c.is_ascii_digit()
                        ) {
                            number.push(self.advance().unwrap());
                        }
                    }

                    TokenKind::Number(number)
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut word = String::new();

                    while matches!(
                        self.peek(),
                        Some(c) if c.is_ascii_alphanumeric() || c == '_'
                    ) {
                        word.push(self.advance().unwrap());
                    }

                    match word.as_str() {
                        "import" => TokenKind::Import,
                        "struct" => TokenKind::Struct,
                        "enum" => TokenKind::Enum,
                        "func" => TokenKind::Fn,
                        "const" => TokenKind::Let,
                        "mut" => TokenKind::Var,
                        "return" => TokenKind::Return,
                        "switch" => TokenKind::Switch,
                        "case" => TokenKind::Case,
                        "for" => TokenKind::For,
                        "in" => TokenKind::In,
                        "try" => TokenKind::Try,
                        "int" => TokenKind::IntType,
                        "float" => TokenKind::FloatType,
                        "string" => TokenKind::StringType,
                        "void" => TokenKind::VoidType,
                        "bool" => TokenKind::BooleanType,
                        "True" => TokenKind::True,
                        "False" => TokenKind::False,
                        "when" => TokenKind::If,
                        "otherwise" => TokenKind::Else,
                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,
                        "while" => TokenKind::While,
                        _ => TokenKind::Identifier(word),
                    }
                }
                _ => {
                    return Err(LexerError {
                        message: format!("Unexpected character '{ch}' [E023]"),
                        line,
                        column,
                    });
                }
            };

            tokens.push(Token::new(kind, line, column));
        }

        tokens.push(Token::new(TokenKind::EOF, self.line, self.column));
        Ok(tokens)
    }
}
