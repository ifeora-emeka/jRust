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
fn codegen_const_string() {
    let rust_code = transpile(r#"const GREETING: string = "Hello";"#);
    assert!(rust_code.contains("const GREETING: &str = \"Hello\""));
}

#[test]
fn codegen_string_variable() {
    let rust_code = transpile(r#"let name: string = "Alice";"#);
    assert!(rust_code.contains("let mut name: String"));
    assert!(rust_code.contains("\"Alice\".to_string()"));
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
    // String concatenation now uses format!()
    assert!(rust_code.contains("format!"));
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

#[test]
fn codegen_if_statement() {
    let rust_code = transpile("if x > 5 { print(\"big\"); }");
    assert!(rust_code.contains("if x > 5"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_if_else_statement() {
    let rust_code = transpile("if x > 5 { print(\"big\"); } else { print(\"small\"); }");
    assert!(rust_code.contains("if x > 5"));
    assert!(rust_code.contains("} else {"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_for_loop() {
    let rust_code = transpile("for item in items { print(item); }");
    assert!(rust_code.contains("for item in"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_while_loop() {
    let rust_code = transpile("while x < 10 { print(x); }");
    assert!(rust_code.contains("while x < 10"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_array_declaration() {
    let rust_code = transpile("let nums: number[] = [1, 2, 3];");
    assert!(rust_code.contains("let mut nums: Vec<i32>"));
    assert!(rust_code.contains("vec![1, 2, 3]"));
}

#[test]
fn codegen_array_indexing() {
    let rust_code = transpile("let first: number = nums[0];");
    assert!(rust_code.contains("nums[0 as usize]"));
}

#[test]
fn codegen_member_access() {
    let rust_code = transpile("let len: number = nums.length;");
    assert!(rust_code.contains("nums.len() as i32"));
}

#[test]
fn codegen_any_type() {
    let rust_code = transpile("let value: any = 42;");
    assert!(rust_code.contains("let mut value: String"));
}

#[test]
fn codegen_string_array() {
    let rust_code = transpile(r#"let strs: string[] = ["a", "b"];"#);
    assert!(rust_code.contains("let mut strs: Vec<String>"));
    assert!(rust_code.contains("vec!["));
}

#[test]
fn codegen_nested_loops() {
    let rust_code = transpile(r#"for i in outer { for j in inner { print(j); } }"#);
    assert!(rust_code.contains("for i in"));
    assert!(rust_code.contains("for j in"));
}

#[test]
fn codegen_array_in_loop() {
    let rust_code = transpile("for num in nums { print(num); }");
    assert!(rust_code.contains("for num in nums"));
    assert!(rust_code.contains("println!"));
}

#[test]
fn codegen_break_statement() {
    let rust_code = transpile("for i in items { break; }");
    assert!(rust_code.contains("break;"));
    assert!(rust_code.contains("for i in items"));
}

#[test]
fn codegen_continue_statement() {
    let rust_code = transpile("for i in items { continue; }");
    assert!(rust_code.contains("continue;"));
    assert!(rust_code.contains("for i in items"));
}

#[test]
fn codegen_break_in_while() {
    let rust_code = transpile("while condition { break; }");
    assert!(rust_code.contains("break;"));
    assert!(rust_code.contains("while"));
}

#[test]
fn codegen_continue_in_while() {
    let rust_code = transpile("while condition { continue; }");
    assert!(rust_code.contains("continue;"));
    assert!(rust_code.contains("while"));
}

#[test]
fn codegen_dynamic_array() {
    let rust_code = transpile("let nums: number[] = [1, 2, 3];");
    assert!(rust_code.contains("let mut nums: Vec<i32>"));
    assert!(rust_code.contains("vec![1, 2, 3]"));
}

#[test]
fn codegen_static_array() {
    let rust_code = transpile("let nums: number[number, 5] = [1, 2, 3, 4, 5];");
    assert!(rust_code.contains("let mut nums: [i32; 5]"));
    assert!(rust_code.contains("[1, 2, 3, 4, 5]"));
}

#[test]
fn codegen_static_string_array() {
    let rust_code = transpile(r#"let names: string[string, 3] = ["Alice", "Bob", "Charlie"];"#);
    assert!(rust_code.contains("let mut names: [String; 3]"));
}

#[test]
fn codegen_static_bool_array() {
    let rust_code = transpile("let flags: boolean[boolean, 2] = [true, false];");
    assert!(rust_code.contains("let mut flags: [bool; 2]"));
    assert!(rust_code.contains("[true, false]"));
}
