use jrust_transpiler_core::Lexer;
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== jRust Lexer Demo ===\n");
    
    let examples = vec![
        ("Variable Declaration", "let x: number = 42;"),
        ("String Variable", "let name: string = \"Alice\";"),
        ("Function", "function greet(name: string): void { print(\"Hello\"); }"),
        ("Print Statement", "print(\"Hello, World!\");"),
        ("String Concatenation", "print(\"Hello\" + \" \" + \"World\");"),
        ("Const Declaration", "const MAX_SIZE: number = 100;"),
    ];
    
    for (description, code) in examples {
        println!("📝 {}", description);
        println!("   Code: {}", code);
        
        match tokenize(code) {
            Ok(token_count) => {
                println!("   ✅ Tokenized successfully ({} tokens)\n", token_count);
            }
            Err(e) => {
                eprintln!("   ❌ Error: {}\n", e);
            }
        }
    }
    
    println!("=== Custom Code ===");
    let custom = r#"
        let x: number = 42;
        print("The answer is: " + x);
    "#;
    
    println!("Input:\n{}\n", custom);
    match tokenize(custom) {
        Ok(token_count) => {
            println!("✅ Tokenized successfully ({} tokens)", token_count);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
    
    Ok(())
}

fn tokenize(source: &str) -> Result<usize, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    Ok(tokens.len())
}
