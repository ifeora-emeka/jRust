use jrust_transpiler_core::{Lexer, Parser, Codegen};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== jRust Transpiler Demo ===\n");
    
    let examples = vec![
        ("Variable Declaration", "let x: number = 42;"),
        ("Type Inference", "let x = 42;"),
        ("String Variable", "let name: string = \"Alice\";"),
        ("Print Statement", "print(\"Hello, World!\");"),
        ("String Concatenation", "print(\"Hello\" + \" \" + \"World\");"),
        ("Const Declaration", "const MAX_SIZE: number = 100;"),
        ("String Method", "let upper = \"hello\".toUpperCase();"),
        ("Array Method", "let len = [1, 2, 3].length;"),
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
    
    println!("=== Complex Program (All Features: Phase 2 + Phase 3.1 + Phase 3.2) ===");
    let complex = r#"
        struct User {
            name: string,
            age: number,
            active: boolean
        }
        
        enum Role {
            Admin,
            User,
            Guest(string)
        }
        
        let alice = User { name: "Alice", age: 30, active: true };
        
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
        
        let x = 10;
        let y = 20;
        
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
        
        let message: string = "Hello, World!";
        let upper = message.toUpperCase();
        let lower = message.toLowerCase();
        let sub = message.substring(0, 5);
        print(upper);
        print(sub);
        
        let mixed: any = "flexible type";
        print(mixed);
    "#;
    
    println!("Input:\n{}\n", complex);
    match transpile(complex) {
        Ok(rust_code) => {
            println!("Output:\n{}\n", rust_code);
            println!("✅ Successfully transpiled complex program with ALL features!");
            println!("   • Type inference (x, y, upper, lower, sub)");
            println!("   • Structs (User)");
            println!("   • Enums (Role with variants)");
            println!("   • Array methods (length, indexing)");
            println!("   • String methods (toUpperCase, toLowerCase, substring)");
            println!("   • Functions, loops, conditionals");
            println!("   • Constants, variables, break/continue");
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
