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
    
    println!("=== Complex Program (All Phase 2 Features + Phase 3.1) ===");
    let complex = r#"
        let nums: number[] = [1, 2, 3, 4, 5];
        const LIMIT: number = 3;
        
        function processNumbers(values: number[]): void {
            print("Numbers greater than 2:");
            
            for n in values {
                if n > 2 {
                    print(n);
                }
            }
        }
        
        function getFirstElement(arr: number[]): number {
            return arr[0];
        }
        
        function getArrayLength(arr: number[]): number {
            return arr.length;
        }
        
        processNumbers(nums);
        
        let first: number = getFirstElement(nums);
        print("First element: ");
        print(first);
        
        let len: number = getArrayLength(nums);
        print("Array length: ");
        print(len);
        
        let x: number = 10;
        let y: number = 20;
        
        if x < y {
            print("x is less than y");
        } else {
            print("x is not less than y");
        }
        
        print("Loop with break and continue:");
        for item in [1, 2, 3, 4, 5, 6] {
            if item == 2 {
                continue;
            }
            if item == 5 {
                break;
            }
            print(item);
        }
        
        let mixed: any = "flexible type";
        print(mixed);
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
