use jrust_transpiler_core::Lexer;

#[test]
fn test_cli_lexer_basic() {
    let mut lexer = Lexer::new("let x: number = 10;");
    let tokens = lexer.tokenize().unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_cli_lexer_function() {
    let mut lexer = Lexer::new("function main(): void { print(\"Hi\"); }");
    let tokens = lexer.tokenize().unwrap();
    assert!(!tokens.is_empty());
}
