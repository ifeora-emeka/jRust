use jrust_transpiler_core::{Lexer, Parser, Codegen};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== jRust Transpiler Demo ===\n");
    
    let examples = vec![
        ("Variable Declaration", "let x: number = 42;"),
        ("String Variable", "let name: string = \"Alice\";"),
        ("Print Statement", "print(\"Hello, World!\");"),
        ("String Concatenation", "print(\"Hello\" + \" \" + \"World\");"),
        ("Const Declaration", "const MAX_SIZE: number = 100;"),
    ];
    
    for (description, code) in examples {
        println!("📝 {}", description);
        println!("   Input:  {}", code);
        
        match transpile(code) {
            Ok(rust_code) => {
                println!("   Output: {}", rust_code.lines().next().unwrap_or(""));
                println!("   ✅ Transpiled successfully\n");
            }
            Err(e) => {
                eprintln!("   ❌ Error: {}\n", e);
            }
        }
    }
    
    println!("=== Complex Program ===");
    let complex = r#"
        let x: number = 42;
        const MSG: string = "The answer is: ";
        
        function printAnswer(value: number): void {
            print(MSG + value);
        }
        
        printAnswer(x);
        print("Done");
    "#;
    
    println!("Input:\n{}\n", complex);
    match transpile(complex) {
        Ok(rust_code) => {
            println!("Output:\n{}\n", rust_code);
            println!("✅ Transpiled successfully");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
    
    Ok(())
}

fn transpile(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut codegen = Codegen::new();
    let rust_code = codegen.generate(&program);
    
    Ok(rust_code)
}
