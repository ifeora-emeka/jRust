# jRust Language Syntax

jRust is a TypeScript-like language that transpiles to Rust, offering familiar syntax with Rust's performance and safety.

## File Extension
`.jr` or `.jrust`

## Types

jRust supports explicit static typing:

```typescript
number   // i32, i64, f64
string   // String
boolean  // bool
void     // ()
```

## Variables

### Mutable Variables (default)

Variables declared with `let` are **mutable by default** and transpile to Rust's `let mut`:

```typescript
let x: number = 42;
let name: string = "Alice";
let isActive: boolean = true;

x = 100;
name = "Bob";
```

### Constants

Constants must be declared with **ALL_CAPS** naming convention and cannot be reassigned:

```typescript
const APP_NAME: string = "MyApp";
const MAX_CONNECTIONS: number = 100;
const API_URL: string = "https://api.example.com";
```

Constants transpile to Rust's `const` and must use uppercase with underscores.

## Functions

Functions use the `function` keyword with explicit parameter and return types:

```typescript
function greet(name: string): void {
    print("Hello, " + name + "!");
}

function add(a: number, b: number): number {
    return a + b;
}

function main(): void {
    let result: number = add(5, 3);
    print("Result: " + result);
}
```

### Function Syntax Rules
- Function parameters require type annotations: `name: type`
- Return type is specified after the parameter list: `: returnType`
- Use `void` for functions that don't return a value
- Function body is enclosed in `{ }`

## Printing to Console

jRust provides console logging functions similar to JavaScript's `console` API:

### Standard Output
```typescript
print("Hello, World!");
print("Value: " + x);
```

### Error Messages
```typescript
print.error("Something went wrong!");
print.error("Failed to connect: " + errorMsg);
```

### Informational Messages
```typescript
print.info("Server started on port 3000");
print.info("Configuration loaded successfully");
```

### Warning Messages
```typescript
print.warn("Deprecated function used");
print.warn("Connection timeout, retrying...");
```

### Debug Messages
```typescript
print.debug("Variable x = " + x);
print.debug("Function called with params: " + params);
```

**Transpilation to Rust:**
- `print()` → `println!()`
- `print.error()` → `eprintln!()`
- `print.info()` → `println!("[INFO] ...")`
- `print.warn()` → `eprintln!("[WARN] ...")`
- `print.debug()` → `println!("[DEBUG] ...")`

## String Concatenation

Strings can be concatenated using the `+` operator:

```typescript
let firstName: string = "John";
let lastName: string = "Doe";
let fullName: string = firstName + " " + lastName;
print(fullName);
```

## Comments

```typescript
// Single-line comment

/*
   Multi-line comment
   across multiple lines
*/
```

## Ownership and Borrowing

jRust follows Rust's ownership and borrowing rules to ensure memory safety:

### Move Semantics
```typescript
let s1: string = "hello";
let s2: string = s1;
```

After assignment, `s1` is moved to `s2` and can no longer be used (compile error if accessed).

### Borrowing (References)
```typescript
function printLength(s: &string): void {
    print("Length: " + s.length);
}

function main(): void {
    let text: string = "Hello";
    printLength(&text);
    print(text);
}
```

Use `&` to borrow a reference without taking ownership. The original variable remains valid.

### Mutable Borrowing
```typescript
function appendWorld(s: &mut string): void {
    s = s + " World";
}

function main(): void {
    let message: string = "Hello";
    appendWorld(&mut message);
    print(message);
}
```

Use `&mut` to borrow a mutable reference. Only one mutable reference allowed at a time.

**Rules:**
- Each value has one owner
- When owner goes out of scope, value is dropped
- Can have multiple immutable references OR one mutable reference
- References must always be valid

## Complete Example

```typescript
const APP_NAME: string = "Calculator";
const VERSION: string = "1.0.0";

function calculateSum(a: number, b: number): number {
    return a + b;
}

function greet(name: &string): void {
    print("Hello, " + name + "!");
}

function displayInfo(app: &string, ver: &string): void {
    print.info("Application: " + app);
    print.info("Version: " + ver);
}

function main(): void {
    displayInfo(&APP_NAME, &VERSION);
    
    let x: number = 10;
    let y: number = 20;
    let sum: number = calculateSum(x, y);
    
    print("Sum: " + sum);
    
    let username: string = "Developer";
    greet(&username);
    
    x = 50;
    print.debug("Updated x = " + x);
}
```

## Transpilation Output

jRust code transpiles to idiomatic Rust with ownership semantics:

**jRust:**
```typescript
const MAX_SIZE: number = 100;

function add(a: number, b: number): number {
    return a + b;
}

function greet(name: &string): void {
    print("Hello, " + name);
    print.error("Error occurred!");
}

function main(): void {
    let x: number = 10;
    x = 20;
}
```

**Generated Rust:**
```rust
const MAX_SIZE: i32 = 100;

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn greet(name: &String) {
    println!("Hello, {}", name);
    eprintln!("Error occurred!");
}

fn main() {
    let mut x: i32 = 10;
    x = 20;
}
```

## Reserved Keywords

```
function, let, const, return, if, else, while, for, 
true, false, number, string, boolean, void, print
```

## Notes

- All statements must end with a semicolon `;`
- Function names and variables use camelCase convention
- Constants must use UPPER_SNAKE_CASE convention
- Variables declared with `let` are mutable by default (transpile to `let mut`)
- Constants declared with `const` are immutable and must be uppercase
- Type annotations are required (type inference coming in future versions)
- jRust enforces Rust's ownership and borrowing rules at compile time
- Use `&` for immutable references and `&mut` for mutable references
- The `main()` function is the entry point of every jRust program
- `print.error()`, `print.info()`, `print.warn()`, and `print.debug()` provide logging levels
