use jrust_transpiler_core::{Lexer, Parser, Codegen};

fn transpile(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexer failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");
    let mut codegen = Codegen::new();
    codegen.generate(&program)
}

#[test]
fn codegen_simple_variable() {
    let rust_code = transpile("let x: number = 42;");
    assert!(rust_code.contains("fn main()"));
    assert!(rust_code.contains("let mut x: i32 = 42;"));
}

#[test]
fn codegen_const_variable() {
    let rust_code = transpile("const MAX: number = 100;");
    assert!(rust_code.contains("const MAX: i32 = 100;"));
}

#[test]
fn codegen_string_variable() {
    let rust_code = transpile(r#"let name: string = "Alice";"#);
    assert!(rust_code.contains("let mut name: String"));
    assert!(rust_code.contains("\"Alice\""));
}

#[test]
fn codegen_print_statement() {
    let rust_code = transpile("print(\"Hello\");");
    assert!(rust_code.contains("println!"));
    assert!(rust_code.contains("\"Hello\""));
}

#[test]
fn codegen_string_concatenation() {
    let rust_code = transpile(r#"print("Hello" + "World");"#);
    assert!(rust_code.contains("to_string()"));
    assert!(rust_code.contains("+"));
}

#[test]
fn codegen_simple_function() {
    let rust_code = transpile("function greet(name: string): void { print(\"Hi\"); }");
    assert!(rust_code.contains("fn greet(name: String)"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_function_with_return() {
    let rust_code = transpile("function getValue(): number { return 42; }");
    assert!(rust_code.contains("fn getValue() -> i32"));
    assert!(rust_code.contains("return 42;"));
}

#[test]
fn codegen_function_with_params() {
    let rust_code = transpile("function add(a: number, b: number): number { return a + b; }");
    assert!(rust_code.contains("fn add(a: i32, b: i32) -> i32"));
}

#[test]
fn codegen_multiple_statements() {
    let rust_code = transpile("let x: number = 5; let y: number = 10;");
    assert!(rust_code.contains("let mut x: i32 = 5;"));
    assert!(rust_code.contains("let mut y: i32 = 10;"));
}

#[test]
fn codegen_arithmetic_expression() {
    let rust_code = transpile("let result: number = 5 + 3 * 2;");
    assert!(rust_code.contains("let mut result: i32"));
}

#[test]
fn codegen_function_call() {
    let rust_code = transpile("let x: number = add(5, 3);");
    assert!(rust_code.contains("add(5, 3)"));
}

#[test]
fn codegen_complex_program() {
    let source = r#"
        let x: number = 42;
        function double(n: number): number { return n * 2; }
        print(x);
    "#;
    let rust_code = transpile(source);
    assert!(rust_code.contains("fn main()"));
    assert!(rust_code.contains("let mut x: i32 = 42;"));
    assert!(rust_code.contains("fn double(n: i32) -> i32"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_contains_main_wrapper() {
    let rust_code = transpile("let x: number = 1;");
    assert!(rust_code.starts_with("fn main()"));
    assert!(rust_code.trim_end().ends_with("}"));
}
