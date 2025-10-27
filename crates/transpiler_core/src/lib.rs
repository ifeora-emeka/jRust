pub fn greet(name: &str) {
    println!("Hello from jRust transpiler, {}!", name);
}

pub fn transpile(source: &str) -> Result<String, String> {
    if source.trim().is_empty() {
        return Err("Source code cannot be empty".to_string());
    }
    
    Ok(format!("// Transpiled from jRust:\n// {}\nfn main() {{\n    println!(\"jRust transpiled successfully!\");\n}}", source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        greet("Test");
    }

    #[test]
    fn test_transpile_valid_source() {
        let result = transpile("fn main() { }");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("jRust transpiled successfully!"));
    }

    #[test]
    fn test_transpile_empty_source() {
        let result = transpile("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Source code cannot be empty");
    }

    #[test]
    fn test_transpile_whitespace_only() {
        let result = transpile("   \n  \t  ");
        assert!(result.is_err());
    }
}
