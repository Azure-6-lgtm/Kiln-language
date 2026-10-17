use crate::ast::{BinaryOp, Expr, Function, Item, Parameter, Program, Stmt};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_return_type: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            current_return_type: None,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek_next(&self) -> &Token {
        let next_position = (self.position + 1).min(self.tokens.len() - 1);
        &self.tokens[next_position]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.position].clone();

        if !matches!(token.kind, TokenKind::EOF) {
            self.position += 1;
        }

        token
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            line: self.peek().line,
            column: self.peek().column,
        }
    }

    fn error_at(&self, line: usize, column: usize, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            line,
            column,
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        if self.peek().kind == expected {
            Ok(self.advance())
        } else {
            Err(self.error(&format!(
                "Expected {:?}, found {:?} [E001]",
                expected,
                self.peek().kind
            )))
        }
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        if matches!(self.peek().kind, TokenKind::Identifier(_))
            && self.peek_next().kind == TokenKind::Equal
        {
            let name = match self.advance().kind {
                TokenKind::Identifier(name) => name,
                _ => unreachable!(),
            };

            self.expect(TokenKind::Equal)?;

            let value = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;

            return Ok(Stmt::Assign { name, value });
        }

        match self.peek().kind {
            TokenKind::If => self.parse_if_statement(),

            TokenKind::Let => {
                self.advance();
                self.parse_variable(false)
            }

            TokenKind::Var => {
                self.advance();
                self.parse_variable(true)
            }

            TokenKind::Return => {
                let return_token = self.advance();

                if self.peek().kind == TokenKind::Semicolon {
                    self.advance();
                    return Ok(Stmt::Return(None));
                }

                let value = self.parse_expression()?;
                self.expect(TokenKind::Semicolon)?;

                if self.current_return_type.as_deref() == Some("void") {
                    return Err(ParseError {
                        message: "A void function cannot return a value [E002]".to_string(),
                        line: return_token.line,
                        column: return_token.column,
                    });
                }

                Ok(Stmt::Return(Some(value)))
            }

            _ => {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();

        while !matches!(self.peek().kind, TokenKind::EOF) {
            match self.peek().kind {
                TokenKind::Fn => {
                    let function = self.parse_function()?;
                    items.push(Item::Function(function));
                }

                _ => {
                    return Err(self.error("Expected a main function declaration [E003]"));
                }
            }
        }

        Ok(Program { items })
    }

    fn parse_variable(&mut self, mutable: bool) -> Result<Stmt, ParseError> {
        let token = self.advance();
        let name = match token.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(self.error_at(
                    token.line,
                    token.column,
                    "Expected variable name [E004]",
                ));
            }
        };

        self.expect(TokenKind::Equal)?;

        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::Let {
            name,
            mutable,
            value,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;

        let Condition = self.parse_expression()?;

        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;

        let then_branch = self.parse_function_body()?;

        let else_branch = if self.peek().kind == TokenKind::Else {
            self.advance();

            if self.peek().kind == TokenKind::If {
                // Parse "otherwise when" as a nested conditional.
                let nested_if = self.parse_if_statement()?;
                Some(vec![nested_if])
            } else {
                // Parse a normal "otherwise" block.
                self.expect(TokenKind::LeftBrace)?;
                Some(self.parse_function_body()?)
            }
        } else {
            None
        };

        Ok(Stmt::If {
            Condition,
            then_branch,
            else_branch,
        })
    }

    // Entry point for expression parsing.
    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_comparison()
    }

    // Lowest precedence: comparisons and equality.
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_addition()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::EqualEqual => BinaryOp::Equal,
                TokenKind::BangEqual => BinaryOp::NotEqual,
                TokenKind::Less => BinaryOp::Less,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };

            self.advance();

            let right = self.parse_addition()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Addition and subtraction.
    fn parse_addition(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplication()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_multiplication()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Multiplication and division.
    fn parse_multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                _ => break,
            };

            self.advance();

            let right = self.parse_primary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Literals, variables, function calls, and parentheses.
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();

        match token.kind {
            TokenKind::Number(number) => Ok(Expr::Number(number)),

            TokenKind::StringSlice(value) => Ok(Expr::String(value)),

            TokenKind::True => Ok(Expr::Boolean(true)),

            TokenKind::False => Ok(Expr::Boolean(false)),

            TokenKind::Identifier(name) => {
                if self.peek().kind == TokenKind::LeftParen {
                    self.advance();

                    let mut args = Vec::new();

                    if self.peek().kind != TokenKind::RightParen {
                        loop {
                            args.push(self.parse_expression()?);

                            if self.peek().kind != TokenKind::Comma {
                                break;
                            }

                            self.advance();
                        }
                    }

                    self.expect(TokenKind::RightParen)?;

                    Ok(Expr::Call {
                        callee: Box::new(Expr::Variable(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Variable(name))
                }
            }

            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }

            TokenKind::Minus => {
                let expr = self.parse_primary()?;

                Ok(Expr::Binary {
                    left: Box::new(Expr::Number("0".to_string())),
                    op: BinaryOp::Subtract,
                    right: Box::new(expr),
                })
            }

            _ => Err(ParseError {
                message: format!("Unexpected token: {:?} [E009]", token.kind),
                line: token.line,
                column: token.column,
            }),
        }
    }

    pub fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect(TokenKind::Fn)?;
        let mut token = self.advance();

        let name = match token.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(self.error_at(
                    token.line,
                    token.column,
                    "Expected function name [E005]",
                ));
            }
        };

        self.expect(TokenKind::LeftParen)?;

        let mut params = Vec::new();

        if self.peek().kind != TokenKind::RightParen {
            loop {
                let token = self.advance();

                let param_name = match token.kind {
                    TokenKind::Identifier(name) => name,
                    _ => {
                        return Err(self.error_at(
                            token.line,
                            token.column,
                            "Expected parameter name [E006]",
                        ));
                    }
                };

                self.expect(TokenKind::Colon)?;
                let type_token = self.advance();

                let ty = match type_token.kind {
                    TokenKind::Identifier(name) => name,
                    TokenKind::IntType => "int".to_string(),
                    TokenKind::FloatType => "float".to_string(),
                    TokenKind::StringType => "string".to_string(),
                    TokenKind::BooleanType => "bool".to_string(),
                    _ => {
                        return Err(self.error_at(
                            type_token.line,
                            type_token.column,
                            "Expected parameter type [E007]",
                        ));
                    }
                };

                params.push(Parameter {
                    name: param_name,
                    ty,
                });

                if self.peek().kind != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        self.expect(TokenKind::RightParen)?;

        let return_type = if self.peek().kind == TokenKind::Arrow {
            self.advance();

            Some(match self.advance().kind {
                TokenKind::Identifier(name) => name,
                TokenKind::IntType => "int".to_string(),
                TokenKind::FloatType => "float".to_string(),
                TokenKind::StringType => "string".to_string(),
                TokenKind::VoidType => "void".to_string(),
                TokenKind::BooleanType => "bool".to_string(),
                _ => return Err(self.error("Expected return type [E008]")),
            })
        } else {
            None
        };

        self.expect(TokenKind::LeftBrace)?;

        let previous_return_type =
            std::mem::replace(&mut self.current_return_type, return_type.clone());

        let body_result = self.parse_function_body();

        self.current_return_type = previous_return_type;

        let body = body_result?;

        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_function_body(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut body = Vec::new();

        while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(body)
    }
}
