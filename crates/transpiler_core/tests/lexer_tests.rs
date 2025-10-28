use jrust_transpiler_core::{Lexer, TokenKind};

#[test]
fn test_lexer_let_declaration() {
    let mut lexer = Lexer::new("let x: number = 42;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::NumberType);
    assert_eq!(tokens[4].kind, TokenKind::Equal);
    assert!(matches!(tokens[5].kind, TokenKind::NumberLiteral(42)));
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
    assert_eq!(tokens[7].kind, TokenKind::Eof);
}

#[test]
fn test_lexer_string_literal() {
    let mut lexer = Lexer::new("let name: string = \"Alice\";");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "name"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::StringType);
    assert_eq!(tokens[4].kind, TokenKind::Equal);
    assert!(matches!(tokens[5].kind, TokenKind::StringLiteral(ref s) if s == "Alice"));
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
}

#[test]
fn test_lexer_function_declaration() {
    let mut lexer = Lexer::new("function greet(name: string): void { print(\"Hello\"); }");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Function);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "greet"));
    assert_eq!(tokens[2].kind, TokenKind::LeftParen);
    assert!(matches!(tokens[3].kind, TokenKind::Identifier(ref s) if s == "name"));
    assert_eq!(tokens[4].kind, TokenKind::Colon);
    assert_eq!(tokens[5].kind, TokenKind::StringType);
    assert_eq!(tokens[6].kind, TokenKind::RightParen);
    assert_eq!(tokens[7].kind, TokenKind::Colon);
    assert_eq!(tokens[8].kind, TokenKind::Void);
    assert_eq!(tokens[9].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[10].kind, TokenKind::Print);
    assert_eq!(tokens[11].kind, TokenKind::LeftParen);
    assert!(matches!(tokens[12].kind, TokenKind::StringLiteral(ref s) if s == "Hello"));
    assert_eq!(tokens[13].kind, TokenKind::RightParen);
    assert_eq!(tokens[14].kind, TokenKind::Semicolon);
    assert_eq!(tokens[15].kind, TokenKind::RightBrace);
}

#[test]
fn test_lexer_print_statement() {
    let mut lexer = Lexer::new("print(\"Hello, World!\");");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Print);
    assert_eq!(tokens[1].kind, TokenKind::LeftParen);
    assert!(matches!(tokens[2].kind, TokenKind::StringLiteral(ref s) if s == "Hello, World!"));
    assert_eq!(tokens[3].kind, TokenKind::RightParen);
    assert_eq!(tokens[4].kind, TokenKind::Semicolon);
}

#[test]
fn test_lexer_string_concatenation() {
    let mut lexer = Lexer::new("\"Hello\" + \"World\"");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::StringLiteral(ref s) if s == "Hello"));
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert!(matches!(tokens[2].kind, TokenKind::StringLiteral(ref s) if s == "World"));
}

#[test]
fn test_lexer_string_escape_sequences() {
    let mut lexer = Lexer::new("\"Hello\\nWorld\"");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::StringLiteral(ref s) if s == "Hello\nWorld"));
}

#[test]
fn test_lexer_operators() {
    let mut lexer = Lexer::new("== != > >= < <= && ||");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::EqualEqual);
    assert_eq!(tokens[1].kind, TokenKind::BangEqual);
    assert_eq!(tokens[2].kind, TokenKind::Greater);
    assert_eq!(tokens[3].kind, TokenKind::GreaterEqual);
    assert_eq!(tokens[4].kind, TokenKind::Less);
    assert_eq!(tokens[5].kind, TokenKind::LessEqual);
    assert_eq!(tokens[6].kind, TokenKind::AmpersandAmpersand);
    assert_eq!(tokens[7].kind, TokenKind::PipePipe);
}

#[test]
fn test_lexer_comments() {
    let mut lexer = Lexer::new("let x: number = 42; // This is a comment");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::NumberType);
    assert_eq!(tokens[4].kind, TokenKind::Equal);
    assert!(matches!(tokens[5].kind, TokenKind::NumberLiteral(42)));
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
    assert_eq!(tokens[7].kind, TokenKind::Eof);
}

#[test]
fn test_lexer_return_statement() {
    let mut lexer = Lexer::new("return 42;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Return);
    assert!(matches!(tokens[1].kind, TokenKind::NumberLiteral(42)));
    assert_eq!(tokens[2].kind, TokenKind::Semicolon);
}

#[test]
fn test_lexer_const_declaration() {
    let mut lexer = Lexer::new("const MAX_SIZE: number = 100;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Const);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "MAX_SIZE"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::NumberType);
    assert_eq!(tokens[4].kind, TokenKind::Equal);
    assert!(matches!(tokens[5].kind, TokenKind::NumberLiteral(100)));
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
}

#[test]
fn test_lexer_multiple_statements() {
    let mut lexer = Lexer::new("let x: number = 10;\nlet y: string = \"test\";");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[6].kind, TokenKind::Semicolon);
    assert_eq!(tokens[7].kind, TokenKind::Let);
}

#[test]
fn test_lexer_invalid_string() {
    let mut lexer = Lexer::new("\"unterminated string");
    let result = lexer.tokenize();

    assert!(result.is_err());
}

#[test]
fn test_lexer_arithmetic_operators() {
    let mut lexer = Lexer::new("a + b - c * d / e % f");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "a"));
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "b"));
    assert_eq!(tokens[3].kind, TokenKind::Minus);
    assert!(matches!(tokens[4].kind, TokenKind::Identifier(ref s) if s == "c"));
    assert_eq!(tokens[5].kind, TokenKind::Star);
}

#[test]
fn test_lexer_if_else_keywords() {
    let mut lexer = Lexer::new("if x > 5 { print(\"big\"); } else { print(\"small\"); }");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::If);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
    assert_eq!(tokens[2].kind, TokenKind::Greater);
    assert!(matches!(tokens[3].kind, TokenKind::NumberLiteral(5)));
    assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[5].kind, TokenKind::Print);
}

#[test]
fn test_lexer_for_while_loops() {
    let mut lexer = Lexer::new("for i in items { } while x < 10 { }");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::For);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "i"));
    assert_eq!(tokens[2].kind, TokenKind::In);
    assert!(matches!(tokens[3].kind, TokenKind::Identifier(ref s) if s == "items"));
    assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[5].kind, TokenKind::RightBrace);
    assert_eq!(tokens[6].kind, TokenKind::While);
}

#[test]
fn test_lexer_array_brackets() {
    let mut lexer = Lexer::new("let nums: number[] = [1, 2, 3];");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "nums"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::NumberType);
    assert_eq!(tokens[4].kind, TokenKind::LeftBracket);
    assert_eq!(tokens[5].kind, TokenKind::RightBracket);
    assert_eq!(tokens[6].kind, TokenKind::Equal);
    assert_eq!(tokens[7].kind, TokenKind::LeftBracket);
    assert!(matches!(tokens[8].kind, TokenKind::NumberLiteral(1)));
    assert_eq!(tokens[9].kind, TokenKind::Comma);
}

#[test]
fn test_lexer_any_type() {
    let mut lexer = Lexer::new("let value: any = 42;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "value"));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::Any);
    assert_eq!(tokens[4].kind, TokenKind::Equal);
}

#[test]
fn test_lexer_break_keyword() {
    let mut lexer = Lexer::new("break;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Break);
    assert_eq!(tokens[1].kind, TokenKind::Semicolon);
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}

#[test]
fn test_lexer_continue_keyword() {
    let mut lexer = Lexer::new("continue;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Continue);
    assert_eq!(tokens[1].kind, TokenKind::Semicolon);
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}
