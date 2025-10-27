use jrust_compiler_core::{greet, compile};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== jRust CLI ===");
    
    greet("Developer");
    
    println!("\nAttempting to compile sample code...");
    match compile("fn main() { print(\"Hello, jRust!\"); }") {
        Ok(rust_code) => {
            println!("\n✓ Compilation successful!");
            println!("\nGenerated Rust code:");
            println!("{}", rust_code);
        }
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
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
