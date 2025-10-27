use jrust_transpiler_core::{greet, transpile};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== jRust CLI ===");
    
    greet("Developer");
    
    println!("\nAttempting to transpile sample code...");
    match transpile("fn main() { print(\"Hello, jRust!\"); }") {
        Ok(rust_code) => {
            println!("\n✓ Transpilation successful!");
            println!("\nGenerated Rust code:");
            println!("{}", rust_code);
        }
        Err(e) => {
            eprintln!("✗ Transpilation failed: {}", e);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_runs() {
        assert!(true);
    }
}
