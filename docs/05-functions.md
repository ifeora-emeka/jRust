# Functions

Functions are the building blocks of jRust programs. They allow you to organize code, avoid repetition, and build modular applications.

## Function Declaration

Use the `function` keyword to declare functions:

```typescript
function greet(name: string): void {
    print("Hello, " + name + "!");
}
```

### Anatomy of a Function

```typescript
function functionName(param1: type1, param2: type2): returnType {
    // function body
}
```

**Parts:**
- `function` — keyword
- `functionName` — function identifier (camelCase)
- `(param1: type1, param2: type2)` — parameters with types
- `: returnType` — return type annotation
- `{ ... }` — function body

## Parameters and Arguments

### Single Parameter

```typescript
function double(x: number): number {
    return x * 2;
}
```

### Multiple Parameters

```typescript
function add(a: number, b: number): number {
    return a + b;
}

function greet(firstName: string, lastName: string): void {
    print("Hello, " + firstName + " " + lastName + "!");
}
```

### No Parameters

```typescript
function getTime(): number {
    return 1000;
}
```

## Return Types

### void (No Return Value)

Functions that don't return a value use `void`:

```typescript
function printMessage(msg: string): void {
    print(msg);
}
```

### Returning Values

```typescript
function add(a: number, b: number): number {
    return a + b;
}

function isEven(x: number): boolean {
    return x % 2 == 0;
}

function getName(): string {
    return "Alice";
}
```

### Explicit Return

```typescript
function max(a: number, b: number): number {
    if (a > b) {
        return a;
    }
    return b;
}
```

**Note:** Early returns are supported (future control flow feature).

## Variable Scope

Variables declared in a function are local to that function:

```typescript
function example(): void {
    let local: number = 10;
    print(local);
}

// print(local);  // Error: local is not defined here
```

## Function Calls

Call functions by name with arguments:

```typescript
function add(a: number, b: number): number {
    return a + b;
}

function main(): void {
    let result: number = add(5, 3);
    print("Result: " + result);  // Output: Result: 8
}
```

## References as Parameters

Use `&` to pass immutable references:

```typescript
function printLength(s: &string): void {
    print("Length: " + s);  // Note: actual length property coming soon
}

function main(): void {
    let text: string = "Hello";
    printLength(&text);
    print(text);  // ✓ text is still valid
}
```

### Mutable References

Use `&mut` to pass mutable references:

```typescript
function append(s: &mut string): void {
    s = s + " World";
}

function main(): void {
    let message: string = "Hello";
    append(&mut message);
    print(message);  // Output: Hello World
}
```

**Borrowing Rules:**
- Can have multiple immutable references (`&`)
- OR one mutable reference (`&mut`)
- References must be valid

## Transpilation to Rust

**jRust:**
```typescript
function multiply(a: number, b: number): number {
    return a * b;
}

function main(): void {
    let result: number = multiply(6, 7);
    print("Result: " + result);
}
```

**Generated Rust:**
```rust
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn main() {
    let mut result: i32 = multiply(6, 7);
    println!("Result: {}", result);
}
```

## Function Best Practices

1. **Descriptive names** — Use clear function names that describe what they do
   ```typescript
   function calculateTotalPrice(): number { }  // ✓ Clear
   function cp(): number { }                   // ✗ Unclear
   ```

2. **Single responsibility** — Each function should do one thing well
   ```typescript
   function validateEmail(email: string): boolean { }  // ✓ One job
   function validateAndSendEmail(email: string): void { }  // ✗ Two jobs
   ```

3. **Explicit types** — Always annotate parameters and return types
   ```typescript
   function add(a: number, b: number): number { }  // ✓ Clear
   function add(a, b) { }                          // ✗ Missing types
   ```

4. **Use references when appropriate** — Avoid unnecessary copies
   ```typescript
   function process(data: &string): void { }  // ✓ Borrows
   function process(data: string): void { }   // Moves (slower)
   ```

## Complete Example

```typescript
const API_URL: string = "https://api.example.com";
const MAX_RETRIES: number = 3;

function logRequest(url: &string): void {
    print.info("Requesting: " + url);
}

function handleError(error: &string, retries: &mut number): void {
    print.error("Error: " + error);
    retries = retries - 1;
}

function fetchData(url: &string): string {
    logRequest(url);
    return "data";
}

function main(): void {
    let retryCount: number = MAX_RETRIES;
    let result: string = fetchData(&API_URL);
    print("Result: " + result);
}
```

## Coming Soon

Future versions will support:
- Default parameters
- Variadic parameters (`...args`)
- Named parameters
- Closures and anonymous functions
- Higher-order functions

Next, learn about **[Comments](06-comments.md)** to document your code.
