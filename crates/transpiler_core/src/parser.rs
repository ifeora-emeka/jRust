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
            TokenKind::Import => self.parse_import_stmt(),
            TokenKind::Export => self.parse_export_stmt(),
            TokenKind::Let => self.parse_variable_decl(false),
            TokenKind::Const => self.parse_variable_decl(true),
            TokenKind::Function => self.parse_function_decl(),
            TokenKind::Struct => self.parse_struct_decl(),
            TokenKind::Enum => self.parse_enum_decl(),
            TokenKind::Print => self.parse_print_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::If => self.parse_if_else_stmt(),
            TokenKind::For => self.parse_for_loop(),
            TokenKind::While => self.parse_while_loop(),
            TokenKind::Break => self.parse_break_stmt(),
            TokenKind::Continue => self.parse_continue_stmt(),
            TokenKind::Try => self.parse_try_catch(),
            TokenKind::Throw => self.parse_throw_stmt(),
            _ => {
                let expr = self.parse_expression()?;
                self.consume(TokenKind::Semicolon, "Expected ';' after statement")?;
                Ok(Statement::ExpressionStmt(expr))
            }
        }
    }

    fn parse_import_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        
        let mut imports = Vec::new();
        let path: String;
        let is_external: bool;
        
        if self.check(&TokenKind::LeftBrace) {
            self.advance();
            loop {
                let name = self.expect_identifier()?;
                let alias = if self.match_token(&TokenKind::As) {
                    Some(self.expect_identifier()?)
                } else {
                    None
                };
                imports.push(ImportItem { name, alias });
                
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.consume(TokenKind::RightBrace, "Expected '}' after import list")?;
            self.consume(TokenKind::From, "Expected 'from' after import list")?;
        } else if matches!(self.peek().kind, TokenKind::Identifier(_)) {
            let name = self.expect_identifier()?;
            
            if self.match_token(&TokenKind::From) {
                imports.push(ImportItem { name, alias: None });
            } else {
                return Err("Expected 'from' after import identifier".to_string());
            }
        } else if matches!(self.peek().kind, TokenKind::StringLiteral(_)) {
        } else {
            return Err("Expected '{', identifier, or string literal in import statement".to_string());
        }
        
        if let TokenKind::StringLiteral(ref s) = self.peek().kind {
            path = s.clone();
            is_external = path.contains("::") || !path.starts_with('.');
            self.advance();
        } else {
            return Err("Expected string literal for import path".to_string());
        }
        
        let alias = if self.match_token(&TokenKind::As) {
            Some(self.expect_identifier()?)
        } else {
            None
        };
        
        if alias.is_some() && !imports.is_empty() {
            imports[0].alias = alias;
        }
        
        self.consume(TokenKind::Semicolon, "Expected ';' after import statement")?;
        
        Ok(Statement::ImportStmt(ImportStmt {
            imports,
            path,
            is_external,
        }))
    }
    
    fn parse_export_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        
        let inner_stmt = match &self.peek().kind {
            TokenKind::Function => self.parse_function_decl()?,
            TokenKind::Struct => self.parse_struct_decl()?,
            TokenKind::Enum => self.parse_enum_decl()?,
            TokenKind::Const => self.parse_variable_decl(true)?,
            TokenKind::Let => self.parse_variable_decl(false)?,
            _ => return Err("Expected function, struct, enum, const, or let after export".to_string()),
        };
        
        Ok(Statement::ExportStmt(Box::new(inner_stmt)))
    }

    fn parse_variable_decl(&mut self, is_const: bool) -> Result<Statement, String> {
        self.advance();
        let name = self.expect_identifier()?;
        
        let var_type = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
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

    fn parse_if_else_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after if condition")?;
        let then_body = self.parse_block()?;
        self.consume(TokenKind::RightBrace, "Expected '}' after if block")?;

        let else_body = if self.match_token(&TokenKind::Else) {
            self.consume(TokenKind::LeftBrace, "Expected '{' after else")?;
            let block = self.parse_block()?;
            self.consume(TokenKind::RightBrace, "Expected '}' after else block")?;
            Some(block)
        } else {
            None
        };

        Ok(Statement::IfElse(IfElseStmt {
            condition,
            then_body,
            else_body,
        }))
    }

    fn parse_for_loop(&mut self) -> Result<Statement, String> {
        self.advance();
        let variable = self.expect_identifier()?;
        self.consume(TokenKind::In, "Expected 'in' in for loop")?;
        let iterable = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after for condition")?;
        let body = self.parse_block()?;
        self.consume(TokenKind::RightBrace, "Expected '}' after for block")?;

        Ok(Statement::ForLoop(ForLoopStmt {
            variable,
            iterable,
            body,
        }))
    }

    fn parse_while_loop(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after while condition")?;
        let body = self.parse_block()?;
        self.consume(TokenKind::RightBrace, "Expected '}' after while block")?;

        Ok(Statement::WhileLoop(WhileLoopStmt { condition, body }))
    }

    fn parse_break_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        self.consume(TokenKind::Semicolon, "Expected ';' after break")?;
        Ok(Statement::BreakStmt)
    }

    fn parse_continue_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        self.consume(TokenKind::Semicolon, "Expected ';' after continue")?;
        Ok(Statement::ContinueStmt)
    }

    fn parse_struct_decl(&mut self) -> Result<Statement, String> {
        self.advance();
        let name = self.expect_identifier()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after struct name")?;
        
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let field_name = self.expect_identifier()?;
            self.consume(TokenKind::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type()?;
            
            fields.push(StructField {
                name: field_name,
                field_type,
            });
            
            if !self.match_token(&TokenKind::Comma) && !self.check(&TokenKind::RightBrace) {
                return Err("Expected ',' or '}' after struct field".to_string());
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after struct fields")?;
        
        Ok(Statement::StructDecl(StructDecl { name, fields }))
    }

    fn parse_enum_decl(&mut self) -> Result<Statement, String> {
        self.advance();
        let name = self.expect_identifier()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after enum name")?;
        
        let mut variants = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let variant_name = self.expect_identifier()?;
            
            let fields = if self.match_token(&TokenKind::LeftParen) {
                let mut field_types = Vec::new();
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        field_types.push(self.parse_type()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::RightParen, "Expected ')' after enum variant fields")?;
                Some(field_types)
            } else {
                None
            };
            
            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });
            
            if !self.match_token(&TokenKind::Comma) && !self.check(&TokenKind::RightBrace) {
                return Err("Expected ',' or '}' after enum variant".to_string());
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after enum variants")?;
        
        Ok(Statement::EnumDecl(EnumDecl { name, variants }))
    }

    fn parse_try_catch(&mut self) -> Result<Statement, String> {
        self.advance();
        self.consume(TokenKind::LeftBrace, "Expected '{' after 'try'")?;
        
        let try_body = self.parse_block()?;
        
        self.consume(TokenKind::RightBrace, "Expected '}' after try block")?;
        self.consume(TokenKind::Catch, "Expected 'catch' after try block")?;
        
        let catch_param = if self.match_token(&TokenKind::LeftParen) {
            let param = Some(self.expect_identifier()?);
            self.consume(TokenKind::RightParen, "Expected ')' after catch parameter")?;
            param
        } else {
            None
        };
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after 'catch'")?;
        let catch_body = self.parse_block()?;
        self.consume(TokenKind::RightBrace, "Expected '}' after catch block")?;
        
        Ok(Statement::TryCatch(TryCatchStmt {
            try_body,
            catch_param,
            catch_body,
        }))
    }

    fn parse_throw_stmt(&mut self) -> Result<Statement, String> {
        self.advance();
        let expression = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';' after throw statement")?;
        
        Ok(Statement::ThrowStmt(ThrowStmt { expression }))
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_logical_and()?;

        while let Some(op) = self.match_binary_op(&[TokenKind::PipePipe]) {
            let right = self.parse_logical_and()?;
            expr = Expression::BinaryOp(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_binary_op(&[TokenKind::AmpersandAmpersand]) {
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_additive()?;

        while let Some(op) = self.match_binary_op(&[
            TokenKind::EqualEqual,
            TokenKind::BangEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let right = self.parse_additive()?;
            expr = Expression::BinaryOp(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }

        Ok(expr)
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

        let mut expr = match &token.kind {
            TokenKind::NumberLiteral(n) => {
                self.advance();
                Expression::NumberLiteral(*n)
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Expression::StringLiteral(s.clone())
            }
            TokenKind::BooleanLiteral(b) => {
                let val = *b;
                self.advance();
                Expression::BooleanLiteral(val)
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
                    Expression::FunctionCall(name, args)
                } else if self.check(&TokenKind::LeftBrace) && self.is_struct_literal_ahead() {
                    self.advance();
                    let mut fields = Vec::new();
                    
                    while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                        let field_name = self.expect_identifier()?;
                        self.consume(TokenKind::Colon, "Expected ':' after field name")?;
                        let field_value = self.parse_expression()?;
                        fields.push((field_name, field_value));
                        
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    
                    self.consume(TokenKind::RightBrace, "Expected '}' after struct literal")?;
                    Expression::StructLiteral { name, fields }
                } else {
                    Expression::Identifier(name)
                }
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !self.check(&TokenKind::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::RightBracket, "Expected ']' after array elements")?;
                Expression::ArrayLiteral(elements)
            }
            TokenKind::Bang => {
                self.advance();
                let expr = self.parse_primary()?;
                Expression::BinaryOp(Box::new(Expression::BooleanLiteral(false)), BinaryOp::And, Box::new(expr))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
                expr
            }
            _ => return Err(format!(
                "Unexpected token in expression: {:?}",
                token.kind
            )),
        };

        while self.check(&TokenKind::LeftBracket) || self.check(&TokenKind::Dot) {
            if self.match_token(&TokenKind::LeftBracket) {
                let index = self.parse_expression()?;
                self.consume(TokenKind::RightBracket, "Expected ']' after index")?;
                expr = Expression::IndexAccess {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&TokenKind::Dot) {
                let member = self.expect_identifier()?;
                
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
                    self.consume(TokenKind::RightParen, "Expected ')' after method arguments")?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: member,
                        arguments: args,
                    };
                } else {
                    expr = Expression::MemberAccess {
                        object: Box::new(expr),
                        member,
                    };
                }
            }
        }

        Ok(expr)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let base_type = match &self.peek().kind {
            TokenKind::NumberType => {
                self.advance();
                Type::Number
            }
            TokenKind::StringType => {
                self.advance();
                Type::String
            }
            TokenKind::BooleanType => {
                self.advance();
                Type::Boolean
            }
            TokenKind::Void => {
                self.advance();
                Type::Void
            }
            TokenKind::Any => {
                self.advance();
                Type::Any
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Type::Custom(name)
            }
            _ => return Err(format!("Expected type, found: {:?}", self.peek().kind)),
        };

        if self.match_token(&TokenKind::LeftBracket) {
            if self.check(&TokenKind::RightBracket) {
                self.advance();
                return Ok(Type::Array {
                    element_type: Box::new(base_type),
                    size: None,
                });
            }

            let element_type = self.parse_type()?;
            
            let size = if self.match_token(&TokenKind::Comma) {
                match &self.peek().kind {
                    TokenKind::NumberLiteral(n) => {
                        let n = *n as usize;
                        self.advance();
                        Some(n)
                    }
                    _ => return Err("Expected number literal for array size".to_string()),
                }
            } else {
                None
            };

            self.consume(TokenKind::RightBracket, "Expected ']' after array type")?;

            Ok(Type::Array {
                element_type: Box::new(element_type),
                size,
            })
        } else {
            Ok(base_type)
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
                    TokenKind::EqualEqual => BinaryOp::Equal,
                    TokenKind::BangEqual => BinaryOp::NotEqual,
                    TokenKind::Greater => BinaryOp::Greater,
                    TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                    TokenKind::Less => BinaryOp::Less,
                    TokenKind::LessEqual => BinaryOp::LessEqual,
                    TokenKind::AmpersandAmpersand => BinaryOp::And,
                    TokenKind::PipePipe => BinaryOp::Or,
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

    fn is_struct_literal_ahead(&self) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        
        let next_token = &self.tokens[self.current + 1].kind;
        matches!(next_token, TokenKind::Identifier(_) | TokenKind::RightBrace)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }
}
