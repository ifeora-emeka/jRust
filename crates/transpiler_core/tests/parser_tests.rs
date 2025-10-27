use jrust_transpiler_core::{Lexer, Parser};

#[test]
fn parse_simple_variable_declaration() {
    let input = "let x: number = 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_const_declaration() {
    let input = "const MAX: number = 100;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_string_variable() {
    let input = r#"let name: string = "Alice";"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_print_statement() {
    let input = r#"print("Hello");"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_print_with_concatenation() {
    let input = r#"print("Hello" + "World");"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_simple_function() {
    let input = "function greet(name: string): void { print(\"Hi\"); }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_function_with_multiple_params() {
    let input = "function add(a: number, b: number): number { }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_function_with_return() {
    let input = "function getValue(): number { return 42; }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_multiple_statements() {
    let input = "let x: number = 5; let y: number = 10; print(x);";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 3);
}

#[test]
fn parse_arithmetic_expression() {
    let input = "let result: number = 5 + 3 * 2;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_function_call() {
    let input = "let x: number = add(5, 3);";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_nested_function_calls() {
    let input = r#"print(concat("Hello", "World"));"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_complex_program() {
    let input = r#"
        let x: number = 10;
        const MESSAGE: string = "Hello";
        
        function printNumber(n: number): void {
            print(n);
        }
        
        print(x);
        printNumber(42);
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    assert!(program.statements.len() > 2);
}

#[test]
fn parse_error_missing_semicolon() {
    let input = "let x: number = 42";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn parse_error_invalid_type() {
    let input = "let x: invalid = 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}
