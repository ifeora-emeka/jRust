/// The jRust compiler core library
/// 
/// This crate contains the lexer, parser, AST, type checker,
/// and Rust code generator for the jRust language.

/// Greets the user with a friendly message
/// 
/// # Examples
/// 
/// ```
/// use jrust_compiler_core::greet;
/// 
/// greet("World");
/// ```
pub fn greet(name: &str) {
    println!("Hello from jRust compiler, {}!", name);
}

/// Compiles a jRust source string (placeholder for now)
/// 
/// # Arguments
/// 
/// * `source` - The jRust source code to compile
/// 
/// # Returns
/// 
/// A string containing the generated Rust code
pub fn compile(source: &str) -> Result<String, String> {
    if source.trim().is_empty() {
        return Err("Source code cannot be empty".to_string());
    }
    
    // Placeholder: just echo back for now
    Ok(format!("// Compiled from jRust:\n// {}\nfn main() {{\n    println!(\"jRust compiled successfully!\");\n}}", source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        // This test just ensures greet doesn't panic
        greet("Test");
    }

    #[test]
    fn test_compile_valid_source() {
        let result = compile("fn main() { }");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("jRust compiled successfully!"));
    }

    #[test]
    fn test_compile_empty_source() {
        let result = compile("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Source code cannot be empty");
    }

    #[test]
    fn test_compile_whitespace_only() {
        let result = compile("   \n  \t  ");
        assert!(result.is_err());
    }
}
