use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(Token::new(TokenKind::Eof, self.line, self.column));
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let line = self.line;
        let column = self.column;
        let ch = self.current_char();

        match ch {
            '+' => {
                self.advance();
                Ok(Token::new(TokenKind::Plus, line, column))
            }
            '-' => {
                self.advance();
                Ok(Token::new(TokenKind::Minus, line, column))
            }
            '*' => {
                self.advance();
                Ok(Token::new(TokenKind::Star, line, column))
            }
            '/' => {
                self.advance();
                Ok(Token::new(TokenKind::Slash, line, column))
            }
            '%' => {
                self.advance();
                Ok(Token::new(TokenKind::Percent, line, column))
            }
            ':' => {
                self.advance();
                Ok(Token::new(TokenKind::Colon, line, column))
            }
            ';' => {
                self.advance();
                Ok(Token::new(TokenKind::Semicolon, line, column))
            }
            ',' => {
                self.advance();
                Ok(Token::new(TokenKind::Comma, line, column))
            }
            '.' => {
                self.advance();
                Ok(Token::new(TokenKind::Dot, line, column))
            }
            '(' => {
                self.advance();
                Ok(Token::new(TokenKind::LeftParen, line, column))
            }
            ')' => {
                self.advance();
                Ok(Token::new(TokenKind::RightParen, line, column))
            }
            '{' => {
                self.advance();
                Ok(Token::new(TokenKind::LeftBrace, line, column))
            }
            '}' => {
                self.advance();
                Ok(Token::new(TokenKind::RightBrace, line, column))
            }
            '&' => {
                self.advance();
                if self.current_char() == '&' {
                    self.advance();
                    Ok(Token::new(TokenKind::AmpersandAmpersand, line, column))
                } else {
                    Ok(Token::new(TokenKind::Ampersand, line, column))
                }
            }
            '|' => {
                self.advance();
                if self.current_char() == '|' {
                    self.advance();
                    Ok(Token::new(TokenKind::PipePipe, line, column))
                } else {
                    Err(format!("Unexpected character '|' at line {}, column {}", line, column))
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::BangEqual, line, column))
                } else {
                    Ok(Token::new(TokenKind::Bang, line, column))
                }
            }
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::EqualEqual, line, column))
                } else {
                    Ok(Token::new(TokenKind::Equal, line, column))
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::GreaterEqual, line, column))
                } else {
                    Ok(Token::new(TokenKind::Greater, line, column))
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::LessEqual, line, column))
                } else {
                    Ok(Token::new(TokenKind::Less, line, column))
                }
            }
            '"' => self.read_string(line, column),
            _ if ch.is_ascii_digit() => self.read_number(line, column),
            _ if ch.is_alphabetic() || ch == '_' => self.read_identifier(line, column),
            _ => Err(format!(
                "Unexpected character '{}' at line {}, column {}",
                ch, line, column
            )),
        }
    }

    fn read_identifier(&mut self, line: usize, column: usize) -> Result<Token, String> {
        let mut ident = String::new();

        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            ident.push(self.current_char());
            self.advance();
        }

        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "function" => TokenKind::Function,
            "return" => TokenKind::Return,
            "void" => TokenKind::Void,
            "const" => TokenKind::Const,
            "print" => TokenKind::Print,
            "number" => TokenKind::NumberType,
            "string" => TokenKind::StringType,
            "boolean" => TokenKind::BooleanType,
            "mut" => TokenKind::Mut,
            _ => TokenKind::Identifier(ident),
        };

        Ok(Token::new(kind, line, column))
    }

    fn read_string(&mut self, line: usize, column: usize) -> Result<Token, String> {
        self.advance();
        let mut value = String::new();

        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(format!("Unterminated string at line {}, column {}", line, column));
                }
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(self.current_char());
                    }
                }
                self.advance();
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}, column {}", line, column));
        }

        self.advance();
        Ok(Token::new(TokenKind::StringLiteral(value), line, column))
    }

    fn read_number(&mut self, line: usize, column: usize) -> Result<Token, String> {
        let mut num_str = String::new();

        while !self.is_at_end() && self.current_char().is_ascii_digit() {
            num_str.push(self.current_char());
            self.advance();
        }

        match num_str.parse::<i32>() {
            Ok(num) => Ok(Token::new(TokenKind::NumberLiteral(num), line, column)),
            Err(_) => Err(format!("Invalid number '{}' at line {}, column {}", num_str, line, column)),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }

            match self.current_char() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                }
                '/' if self.peek_ahead() == Some('/') => {
                    self.advance();
                    self.advance();
                    while !self.is_at_end() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn peek_ahead(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}
