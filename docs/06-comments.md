# Comments

Comments are text in your code that are ignored by the transpiler. Use them to explain your code and make it more readable.

## Single-Line Comments

Single-line comments start with `//` and extend to the end of the line:

```typescript
// This is a single-line comment
let x: number = 10;  // Initialize x

// Calculate the result
let result: number = x * 2;
```

## Multi-Line Comments

Multi-line comments start with `/*` and end with `*/`. They can span multiple lines:

```typescript
/*
  This is a multi-line comment.
  You can write multiple lines here.
  It's useful for longer explanations.
*/
let y: number = 20;
```

You can also nest multi-line comments within single lines:

```typescript
/* Quick explanation */ let z: number = 30;
```

## Comment Styles

### Explaining What Code Does

```typescript
// Convert temperature from Celsius to Fahrenheit
function celsiusToFahrenheit(celsius: number): number {
    return (celsius * 9 / 5) + 32;
}
```

### Explaining Why Code Exists

```typescript
// We use a larger buffer size here because the old value
// caused performance issues on low-memory devices
const BUFFER_SIZE: number = 8192;
```

### Marking TODO Items

```typescript
function processData(data: &string): void {
    // TODO: Add input validation
    print("Processing: " + data);
    
    // TODO: Implement error handling
}
```

## Doc Comments (Future)

Future versions will support documentation comments:

```typescript
/// Calculates the sum of two numbers
/// 
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The sum of a and b
function add(a: number, b: number): number {
    return a + b;
}
```

## Transpilation

Comments are preserved in generated Rust code:

**jRust:**
```typescript
// Calculate total price
function calculateTotal(price: number, tax: number): number {
    // Add tax to price
    return price + tax;
}
```

**Generated Rust:**
```rust
// Calculate total price
fn calculateTotal(price: i32, tax: i32) -> i32 {
    // Add tax to price
    return price + tax;
}
```

## Best Practices

1. **Don't over-comment** — Code should be self-explanatory
   ```typescript
   // ✓ Good: explains why
   const RETRY_LIMIT: number = 3;  // Empirically determined optimal value
   
   // ✗ Bad: obvious from code
   let count: number = 0;  // Initialize count to zero
   ```

2. **Keep comments up-to-date** — Outdated comments are worse than no comments
   ```typescript
   // ✓ Stays correct
   function calculateTax(amount: number): number { }
   
   // ✗ Gets out of sync
   // This calculates the base price
   function calculateTax(amount: number): number { }
   ```

3. **Use comments for context** — Explain non-obvious decisions
   ```typescript
   // We cap at 1000 because larger requests timeout
   // on the server side (see ticket #1234)
   const MAX_REQUEST_SIZE: number = 1000;
   ```

4. **Comment complex logic** — But prefer clear code first
   ```typescript
   // Calculate Fibonacci number (recursive with memoization)
   function fib(n: number): number {
       // ... implementation
   }
   ```

## Complete Example

```typescript
const APP_NAME: string = "DataProcessor";
const MAX_BUFFER: number = 1024;

// Processes incoming data stream
function processStream(data: &string): void {
    // Validate input
    // TODO: Add error handling for malformed data
    
    print.info("Processing " + data);
    
    /* 
    The next section handles edge cases
    that were discovered during testing
    */
    
    print.debug("Buffer size: " + MAX_BUFFER);
}

function main(): void {
    let input: string = "example data";
    
    // Start processing
    processStream(&input);
}
```

## Comment Transpilation Rules

- Single-line comments (`//`) are preserved as-is in Rust
- Multi-line comments (`/* */`) are preserved as-is in Rust
- Comments before statements help with source mapping to jRust lines

Next, explore **[Ownership and Borrowing](07-ownership-and-borrowing.md)** to understand memory safety in jRust.
