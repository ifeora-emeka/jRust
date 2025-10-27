# Output and Transpilation

Understanding how jRust outputs logging and how it transpiles to Rust is essential for debugging and optimization.

## Console Output Functions

jRust provides multiple logging functions similar to JavaScript's `console` API:

### Standard Output: print()

Use `print()` for standard output:

```typescript
print("Hello, World!");
print("The answer is: " + 42);
```

**Transpiles to Rust:**
```rust
println!("Hello, World!");
println!("The answer is: {}", 42);
```

### Error Output: print.error()

Use `print.error()` for error messages (goes to stderr):

```typescript
print.error("Something went wrong!");
print.error("Failed to connect: " + error);
```

**Transpiles to Rust:**
```rust
eprintln!("Something went wrong!");
eprintln!("Failed to connect: {}", error);
```

### Info Messages: print.info()

Use `print.info()` for informational messages:

```typescript
print.info("Server started on port 3000");
print.info("Configuration loaded successfully");
```

**Transpiles to Rust:**
```rust
println!("[INFO] Server started on port 3000");
println!("[INFO] Configuration loaded successfully");
```

### Warning Messages: print.warn()

Use `print.warn()` for warning messages:

```typescript
print.warn("Deprecated function used");
print.warn("Connection timeout, retrying...");
```

**Transpiles to Rust:**
```rust
eprintln!("[WARN] Deprecated function used");
eprintln!("[WARN] Connection timeout, retrying...");
```

### Debug Messages: print.debug()

Use `print.debug()` for debug information:

```typescript
print.debug("Variable x = " + x);
print.debug("Function called with params: " + params);
```

**Transpiles to Rust:**
```rust
println!("[DEBUG] Variable x = {}", x);
println!("[DEBUG] Function called with params: {}", params);
```

## Output Levels Summary

| Function | Rust Equivalent | Output Stream | Use Case |
|----------|-----------------|---------------|----------|
| `print()` | `println!()` | stdout | Normal output |
| `print.error()` | `eprintln!()` | stderr | Errors |
| `print.info()` | `println!("[INFO]")` | stdout | Information |
| `print.warn()` | `eprintln!("[WARN]")` | stderr | Warnings |
| `print.debug()` | `println!("[DEBUG]")` | stdout | Debugging |

## Complete Transpilation Example

### jRust Code

```typescript
const APP_VERSION: string = "1.0.0";

function validateInput(input: &string): boolean {
    if (input == "") {
        print.error("Input cannot be empty");
        return false;
    }
    return true;
}

function processData(data: &string): void {
    print.info("Processing data...");
    print.debug("Input: " + data);
    print("Data processed successfully");
}

function main(): void {
    print("App started: " + APP_VERSION);
    
    let userInput: string = "test data";
    
    if (validateInput(&userInput)) {
        processData(&userInput);
    } else {
        print.warn("Validation failed");
    }
}
```

### Generated Rust

```rust
const APP_VERSION: &str = "1.0.0";

fn validateInput(input: &String) -> bool {
    if input == "" {
        eprintln!("Input cannot be empty");
        return false;
    }
    return true;
}

fn processData(data: &String) {
    println!("[INFO] Processing data...");
    println!("[DEBUG] Input: {}", data);
    println!("Data processed successfully");
}

fn main() {
    println!("App started: {}", APP_VERSION);
    
    let mut userInput: String = String::from("test data");
    
    if validateInput(&userInput) {
        processData(&userInput);
    } else {
        eprintln!("[WARN] Validation failed");
    }
}
```

## Output in Different Contexts

### Logging Errors

```typescript
function readFile(path: &string): string {
    // Simulate file read
    let success: boolean = false;
    
    if (!success) {
        print.error("Failed to read file: " + path);
        return "";
    }
    return "file content";
}
```

### Debugging Information

```typescript
function complexCalculation(x: number): number {
    print.debug("Starting calculation with x = " + x);
    
    let temp: number = x * 2;
    print.debug("Intermediate result: " + temp);
    
    let result: number = temp + 10;
    print.debug("Final result: " + result);
    
    return result;
}
```

### Informational Flow

```typescript
function startup(): void {
    print.info("Loading configuration...");
    print.info("Initializing database...");
    print.info("Starting server...");
    print("Application ready!");
}
```

## Best Practices for Output

1. **Use appropriate log levels** — Choose the right function for the message type
   ```typescript
   print("Normal operation");           // ✓ For general output
   print.info("Important milestone");   // ✓ For milestones
   print.debug("Variable state");       // ✓ For debugging
   print.warn("Unusual situation");     // ✓ For warnings
   print.error("Something failed");     // ✓ For errors
   ```

2. **Include context** — Help readers understand what's happening
   ```typescript
   print.error("Connection timeout");        // ✗ Vague
   print.error("Failed to connect to " + host + " after " + timeout + "ms");  // ✓ Clear
   ```

3. **Use constants for prefixes** — Avoid duplication
   ```typescript
   const ERROR_PREFIX: string = "[ERROR]";
   print(ERROR_PREFIX + " Something failed");
   ```

4. **Log at function boundaries** — Show what functions do
   ```typescript
   function processRequest(req: &string): void {
       print.debug("Processing request: " + req);
       // ... do work ...
       print.info("Request completed");
   }
   ```

5. **Avoid excessive logging** — Keep output relevant
   ```typescript
   // ✗ Too much
   let x: number = 0;
   print.debug("x initialized");
   x = 1;
   print.debug("x incremented");
   
   // ✓ Appropriate
   let x: number = 0;
   // ... significant operations ...
   print.debug("Final x value: " + x);
   ```

## Controlling Output in Generated Rust

The generated Rust code uses standard Rust macros:
- `println!()` — stdout with newline
- `eprintln!()` — stderr with newline

You can compile the Rust code with standard Rust techniques:

```bash
# Standard output
cargo run

# Redirect output
cargo run > output.txt
cargo run 2> errors.txt

# Suppress output
cargo run > /dev/null
```

## Environment-Based Logging (Coming Soon)

Future versions might support:

```typescript
const DEBUG_MODE: boolean = env.get("DEBUG") == "true";

function debug(msg: &string): void {
    if (DEBUG_MODE) {
        print.debug(msg);
    }
}
```

## Complete Logging Example

```typescript
const LOG_TAG: string = "[MyApp]";

function logMessage(level: &string, msg: &string): void {
    let formatted: string = LOG_TAG + " " + level + " " + msg;
    print(formatted);
}

function main(): void {
    print.info("Application starting");
    
    let count: number = 0;
    print.debug("Counter initialized: " + count);
    
    // Simulate processing
    count = count + 10;
    print.info("Processing complete, count: " + count);
    
    if (count > 20) {
        print.warn("Count exceeded expected threshold");
    }
    
    print("Application finished");
}
```

## Next Steps

Congratulations! You've completed the jRust documentation. You now understand:

✓ Introduction and design philosophy  
✓ Installation and project setup  
✓ Variables and constants  
✓ Primitive types  
✓ Functions and parameters  
✓ Comments and documentation  
✓ Ownership and borrowing (memory safety)  
✓ Strings and concatenation  
✓ Output and logging  

### Start Building!

1. Create a new project: `jnet init my-first-app`
2. Write your first program in `src/main.jr`
3. Build and run: `jnet build && jnet start`
4. Explore and experiment!

### Resources

- **GitHub Repository:** https://github.com/ifeora-emeka/jRust
- **Rust Documentation:** https://doc.rust-lang.org (for generated code)
- **Examples:** See `examples/` directory in the repository

Happy coding with jRust! 🦀🚀
