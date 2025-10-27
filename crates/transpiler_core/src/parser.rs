use crate::token::{Token, TokenKind};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.peek().kind {
            TokenKind::Let => self.parse_variable_decl(false),
            TokenKind::Const => self.parse_variable_decl(true),
            TokenKind::Function => self.parse_function_decl(),
            TokenKind::Print => self.parse_print_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            _ => {
                let expr = self.parse_expression()?;
                self.consume(TokenKind::Semicolon, "Expected ';' after statement")?;
                Ok(Statement::ExpressionStmt(expr))
            }
        }
    }

    fn parse_variable_decl(&mut self, is_const: bool) -> Result<Statement, String> {
        self.advance();
        let name = self.expect_identifier()?;
        self.consume(TokenKind::Colon, "Expected ':' after variable name")?;
        let var_type = self.parse_type()?;
        self.consume(TokenKind::Equal, "Expected '=' in variable declaration")?;
        let value = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Statement::VariableDecl(VariableDecl {
            name,
            var_type,
            value,
            is_const,
        }))
    }

    fn parse_function_decl(&mut self) -> Result<Statement, String> {
        self.advance();
        let name = self.expect_identifier()?;
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.consume(TokenKind::Colon, "Expected ':' in parameter")?;
                let param_type = self.parse_type()?;
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                });

                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenKind::Colon, "Expected ':' after function signature")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            body.push(self.parse_statement()?);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;

        Ok(Statement::FunctionDecl(FunctionDecl {
            name,
            parameters,
            return_type,
            body,
        }))
    }

    fn parse_print_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        self.consume(TokenKind::LeftParen, "Expected '(' after 'print'")?;
        let expression = self.parse_expression()?;
        self.consume(TokenKind::RightParen, "Expected ')' after print expression")?;
        self.consume(TokenKind::Semicolon, "Expected ';' after print statement")?;

        Ok(Statement::PrintStmt(PrintStmt { expression }))
    }

    fn parse_return_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume(TokenKind::Semicolon, "Expected ';' after return statement")?;

        Ok(Statement::ReturnStmt(ReturnStmt { value }))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_multiplicative()?;

        while let Some(op) = self.match_binary_op(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_multiplicative()?;
            expr = Expression::BinaryOp(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.match_binary_op(&[
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
        ]) {
            let right = self.parse_primary()?;
            expr = Expression::BinaryOp(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        match &token.kind {
            TokenKind::NumberLiteral(n) => {
                self.advance();
                Ok(Expression::NumberLiteral(*n))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(Expression::StringLiteral(s.clone()))
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(&TokenKind::LeftParen) {
                    let mut args = Vec::new();
                    if !self.check(&TokenKind::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenKind::RightParen, "Expected ')' after function arguments")?;
                    Ok(Expression::FunctionCall(name, args))
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                token.kind
            )),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match &self.peek().kind {
            TokenKind::NumberType => {
                self.advance();
                Ok(Type::Number)
            }
            TokenKind::StringType => {
                self.advance();
                Ok(Type::String)
            }
            TokenKind::BooleanType => {
                self.advance();
                Ok(Type::Boolean)
            }
            TokenKind::Void => {
                self.advance();
                Ok(Type::Void)
            }
            _ => Err(format!("Expected type, found: {:?}", self.peek().kind)),
        }
    }

    fn match_binary_op(&mut self, kinds: &[TokenKind]) -> Option<BinaryOp> {
        for kind in kinds {
            if self.check(kind) {
                let op = match kind {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Subtract,
                    TokenKind::Star => BinaryOp::Multiply,
                    TokenKind::Slash => BinaryOp::Divide,
                    TokenKind::Percent => BinaryOp::Modulo,
                    _ => return None,
                };
                self.advance();
                return Some(op);
            }
        }
        None
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let result = name.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(format!("Expected identifier, found: {:?}", self.peek().kind)),
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, String> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(format!(
                "{} (found: {:?} at line:column {}:{})",
                message,
                self.peek().kind,
                self.peek().line,
                self.peek().column
            ))
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn skip_newlines(&mut self) {
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }
}
