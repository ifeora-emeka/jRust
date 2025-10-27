/// jRust standard library
/// 
/// Core functions and types available to all jRust programs

pub use jrust_runtime;

/// Prints a message to stdout
pub fn print(message: &str) {
    println!("{}", message);
}

/// Prints a formatted message
pub fn printf(format: &str, args: &[&str]) {
    let mut result = format.to_string();
    for arg in args {
        result = result.replacen("{}", arg, 1);
    }
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        // Just ensure print doesn't panic
        print("Hello from test");
    }

    #[test]
    fn test_printf() {
        printf("Hello, {}!", &["World"]);
    }
}
